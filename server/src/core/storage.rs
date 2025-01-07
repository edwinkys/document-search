use super::*;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use axum::http::StatusCode;

#[derive(Debug)]
pub struct Storage {
    bucket: String,
    client: Client,
}

impl Storage {
    /// Creates an instance of the storage layer.
    pub async fn new(bucket: impl AsRef<str>) -> Self {
        let config = aws_config::load_from_env().await;
        let storage = Storage {
            bucket: bucket.as_ref().to_string(),
            client: Client::new(&config),
        };

        #[cfg(not(test))]
        storage._provision().await;
        storage
    }

    /// Uploads the data to the S3 bucket.
    pub async fn upload(
        &self,
        key: impl AsRef<str>,
        data: Vec<u8>,
    ) -> Result<(), ErrorResponse> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key.as_ref())
            .body(ByteStream::from(data))
            .send()
            .await
            .map_err(|_| ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to upload the object to S3.".to_string(),
                solution: None,
            })?;

        Ok(())
    }

    /// Removes an object from the S3 bucket.
    pub async fn remove(
        &self,
        key: impl AsRef<str>,
    ) -> Result<(), ErrorResponse> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key.as_ref())
            .send()
            .await
            .map_err(|_e| {
                #[cfg(test)]
                eprintln!("Failed to remove the object: {_e:?}");
                ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to remove the object from S3.".to_string(),
                    solution: None,
                }
            })?;

        Ok(())
    }

    async fn _provision(&self) {
        let client = &self.client;
        let bucket = &self.bucket;

        tracing::info!("Checking for the S3 bucket: {bucket}");
        let bucket_exists = {
            let response = client.head_bucket().bucket(bucket).send().await;
            response.ok().is_some()
        };

        if !bucket_exists {
            let response = client.create_bucket().bucket(bucket).send().await;
            response.expect("Failed to create the bucket");
            tracing::info!("The service S3 bucket is created: {bucket}");
        }
    }
}
