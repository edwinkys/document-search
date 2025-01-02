use aws_sdk_s3::Client;

#[derive(Debug)]
pub struct Storage {
    pub bucket: String,
    pub client: Client,
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
