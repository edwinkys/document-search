mod coordinator;
pub mod interface;

use crate::apis::{QueueAPI, StorageAPI};
use crate::protos;
use crate::types::*;
use axum::http::StatusCode;
use interface::ErrorResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;
use uuid::Uuid;

const QUEUE_NAME: &str = "tasks";

#[derive(Debug, Clone)]
pub struct Configuration {
    pub secret: String,
    pub bucket: String,
    pub queue_url: Url,
    pub database_url: Url,
    pub pool_size: u16,
}

#[cfg(test)]
impl Default for Configuration {
    fn default() -> Self {
        let queue = "amqp://localhost:5672/%2f";
        let database = "postgres://postgres:password@localhost:5432/postgres";

        Configuration {
            secret: "secretkey".to_string(),
            bucket: "dl-9799a9487ced".to_string(),
            queue_url: Url::parse(queue).unwrap(),
            database_url: Url::parse(database).unwrap(),
            pool_size: 2,
        }
    }
}

#[derive(Debug)]
pub struct Service {
    config: Configuration,
    workers: Mutex<Vec<Worker>>,
    storage: StorageAPI,
    queue: QueueAPI,
    pool: PgPool,
}

impl Service {
    /// Creates an instance of the service with the given configuration.
    pub async fn new(config: &Configuration) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(config.pool_size as u32)
            .connect(config.database_url.as_str())
            .await
            .expect("Failed to connect to the database");

        Service {
            config: config.clone(),
            workers: Mutex::new(Vec::new()),
            queue: QueueAPI::new(QUEUE_NAME, config.queue_url.as_str()).await,
            storage: StorageAPI::new(&config.bucket).await,
            pool,
        }
    }

    /// Validates the authorization secret from the request.
    pub fn validate_secret(
        &self,
        secret: impl AsRef<str>,
    ) -> Result<(), ErrorResponse> {
        let secret = secret.as_ref();
        if secret != self.config.secret {
            return Err(ErrorResponse {
                code: StatusCode::UNAUTHORIZED,
                message: String::from("Please provide a valid secret key."),
                solution: None,
            });
        }

        Ok(())
    }

    /// Validates that ID is a valid UUID.
    pub fn validate_uuid(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Uuid, ErrorResponse> {
        Uuid::parse_str(id.as_ref()).map_err(|_| ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: String::from("Please provide a valid UUID."),
            solution: None,
        })
    }

    /// Adds a worker to the list of workers if it doesn't already exist.
    pub async fn add_worker(&self, worker: &Worker) {
        let mut workers = self.workers.lock().await;
        if workers.iter().any(|w| w.id == worker.id) {
            return;
        }

        workers.push(worker.clone());
    }

    /// Removes multiple workers from the list of workers by their IDs.
    pub async fn remove_workers(&self, ids: &[WorkerID]) {
        let mut workers = self.workers.lock().await;
        workers.retain(|worker| !ids.contains(&worker.id));
    }

    /// Returns a list of all workers.
    pub async fn workers(&self) -> Vec<Worker> {
        self.workers.lock().await.clone()
    }

    /// Creates a new namespace with the given name.
    pub async fn create_namespace(
        &self,
        name: impl AsRef<str>,
        config: &NamespaceConfig,
    ) -> Result<Namespace, ErrorResponse> {
        let name = name.as_ref();
        let config = serde_json::to_value(config).unwrap();

        let namespace: Namespace = sqlx::query_as(
            "INSERT INTO namespaces (name, config)
            VALUES ($1, $2)
            RETURNING *;",
        )
        .bind(name)
        .bind(&config)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to create a new namespace: {_e:?}");
            ErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: String::from("Failed to create a new namespace."),
                solution: Some(String::from(
                    "Please contact the support team.",
                )),
            }
        })?;

        namespace.provision(&self.pool).await?;
        Ok(namespace)
    }

    /// Removes a namespace and its resources given its name.
    pub async fn remove_namespace(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<Namespace>, ErrorResponse> {
        let name = name.as_ref();
        let namespace: Option<Namespace> = sqlx::query_as(
            "DELETE FROM namespaces
            WHERE name = $1
            RETURNING *;",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: String::from("Failed to remove the namespace."),
            solution: Some(String::from("Please contact the support team.")),
        })?;

        if let Some(namespace) = &namespace {
            namespace.teardown(&self.pool).await?;
        }

        Ok(namespace)
    }

    /// Returns a namespace given its name if it exists.
    pub async fn get_namespace(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Namespace, ErrorResponse> {
        let name = name.as_ref();
        let namespace: Option<Namespace> = sqlx::query_as(
            "SELECT * FROM namespaces
            WHERE name = $1;",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: String::from("Failed to retrieve the namespace."),
            solution: None,
        })?;

        if namespace.is_none() {
            return Err(ErrorResponse {
                code: StatusCode::NOT_FOUND,
                message: "The specified namespace is not found".to_string(),
                solution: Some(String::from(
                    "Please use an existing namespace or create a new one.",
                )),
            });
        }

        Ok(namespace.unwrap())
    }

    /// Creates a new document record within the given namespace.
    pub async fn create_document(
        &self,
        namespace: &Namespace,
        metadata: &Value,
    ) -> Result<Document, ErrorResponse> {
        let schema = namespace.schema();
        let document: Document = sqlx::query_as(&format!(
            "INSERT INTO {schema}.documents (metadata)
            VALUES ($1)
            RETURNING *;",
        ))
        .bind(metadata)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to create a new document: {_e:?}");
            ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Failed to create a new document."),
                solution: None,
            }
        })?;

        Ok(document)
    }

    /// Removes a document record from the database.
    pub async fn remove_document(
        &self,
        namespace: &Namespace,
        id: &DocumentID,
    ) -> Result<Option<Document>, ErrorResponse> {
        let schema = namespace.schema();
        let document: Option<Document> = sqlx::query_as(&format!(
            "DELETE FROM {schema}.documents
            WHERE id = $1
            RETURNING *;",
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to remove the document: {_e:?}");
            ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to remove the document.".to_string(),
                solution: None,
            }
        })?;

        Ok(document)
    }
}
