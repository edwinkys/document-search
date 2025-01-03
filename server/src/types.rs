use crate::protos;
use crate::services::interface::ErrorResponse;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::Type;
use sqlx::{FromRow, PgPool, Row};
use std::net::SocketAddr;
use tonic::Status;
use uuid::Uuid;

pub type NamespaceID = Uuid;
pub type WorkerID = Uuid;
pub type DocumentID = Uuid;

#[derive(Debug, Clone)]
pub struct Worker {
    pub id: WorkerID,
    pub address: SocketAddr,
}

impl TryFrom<protos::RegisterWorkerRequest> for Worker {
    type Error = Status;
    fn try_from(
        value: protos::RegisterWorkerRequest,
    ) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&value.id).map_err(|_| {
            Status::invalid_argument("Worker ID must be a valid UUID.")
        })?;

        let address = value.address.parse::<SocketAddr>().map_err(|_| {
            let message = "Address must be formatted as host:port.";
            Status::invalid_argument(message)
        })?;

        Ok(Worker { id, address })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    pub m: u8,
    pub ef_construction: u16,
}

impl Default for IndexConfig {
    fn default() -> Self {
        IndexConfig {
            m: 32,
            ef_construction: 128,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NamespaceConfig {
    pub index: IndexConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    pub id: NamespaceID,
    pub name: String,
    pub config: NamespaceConfig,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Namespace {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let config: Value = row.try_get("config")?;
        let config = serde_json::from_value(config).map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to parse namespace configuration: {_e:?}");
            sqlx::Error::ColumnDecode {
                index: "config".to_string(),
                source: "Failed to parse the configuration from JSON.".into(),
            }
        })?;

        Ok(Namespace {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
            config,
        })
    }
}

impl Namespace {
    /// Returns the database schema name for the namespace.
    pub fn schema(&self) -> String {
        let id = self.id.simple().to_string();
        let schema = id.split_at(12).0;
        format!("ns_{schema}")
    }

    /// Provisions the namespace with the required schema tables and indexes.
    pub async fn provision(&self, pool: &PgPool) -> Result<(), ErrorResponse> {
        let m = self.config.index.m;
        let ef_construction = self.config.index.ef_construction;

        let schema = self.schema();
        let query = format!(
            "CREATE SCHEMA IF NOT EXISTS {schema};

            CREATE TABLE IF NOT EXISTS {schema}.documents (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                status doc_status NOT NULL DEFAULT 'pending',
                metadata JSONB,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            CREATE INDEX IF NOT EXISTS documents_status_idx
            ON {schema}.documents (status);

            CREATE TABLE IF NOT EXISTS {schema}.chunks (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                document_id UUID NOT NULL ,
                content TEXT NOT NULL,
                semantic_vector VECTOR(1536) NOT NULL,
                text_vector TSVECTOR NOT NULL,

                FOREIGN KEY (document_id)
                REFERENCES {schema}.documents (id)
                ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS chunks_semantic_vector_idx
            ON {schema}.chunks USING HNSW (semantic_vector vector_cosine_ops)
            WITH (m = {m}, ef_construction = {ef_construction});

            CREATE INDEX IF NOT EXISTS chunks_text_vector_idx
            ON {schema}.chunks USING GIN (text_vector);"
        );

        sqlx::raw_sql(&query).execute(pool).await.map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to provision the namespace: {_e:?}");
            ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Failed to provision the namespace"),
                solution: None,
            }
        })?;

        Ok(())
    }

    /// Teardown the namespace by dropping the schema and all its tables.
    pub async fn teardown(&self, pool: &PgPool) -> Result<(), ErrorResponse> {
        let schema = self.schema();
        sqlx::query(&format!("DROP SCHEMA IF EXISTS {schema} CASCADE;"))
            .execute(pool)
            .await
            .map_err(|_| ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Failed to teardown the namespace"),
                solution: None,
            })?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Type)]
#[sqlx(type_name = "doc_status", rename_all = "lowercase")]
pub enum DocumentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentID,
    pub status: DocumentStatus,
    pub metadata: Value,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Document {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Document {
            id: row.try_get("id")?,
            status: row.try_get("status")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_namespace_schema() {
        let id = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
        let namespace = Namespace {
            id: Uuid::from_str(id).unwrap(),
            name: "default".to_string(),
            config: NamespaceConfig::default(),
            created_at: Utc::now(),
        };

        assert_eq!(namespace.schema(), "ns_f47ac10b58cc");
    }
}
