use crate::protos;
use crate::services::interface::ErrorResponse;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::net::SocketAddr;
use tonic::Status;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkerID(pub Uuid);

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
        let id = WorkerID(Uuid::parse_str(&value.id).map_err(|_| {
            Status::invalid_argument("Worker ID must be a valid UUID.")
        })?);

        let address = value.address.parse::<SocketAddr>().map_err(|_| {
            let message = "Address must be formatted as host:port.";
            Status::invalid_argument(message)
        })?;

        Ok(Worker { id, address })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Namespace {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
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
        let schema = self.schema();
        let query = format!(
            "CREATE SCHEMA IF NOT EXISTS {schema};

            CREATE TABLE IF NOT EXISTS {schema}.documents (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                url TEXT NOT NULL,
                status doc_status NOT NULL DEFAULT 'pending',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            CREATE INDEX IF NOT EXISTS documents_status_idx
            ON {schema}.documents (status);

            CREATE TABLE IF NOT EXISTS {schema}.chunks (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                document_id UUID NOT NULL REFERENCES {schema}.documents (id),
                content TEXT NOT NULL,
                semantic_vector VECTOR(384) NOT NULL,
                text_vector TSVECTOR NOT NULL,
            );

            CREATE INDEX IF NOT EXISTS chunks_semantic_vector_idx
            ON {schema}.chunks USING HNSW (semantic_vector vector_cosine_ops)
            WITH (m = 32, ef_construction = 128);

            CREATE INDEX IF NOT EXISTS chunks_text_vector_idx
            ON {schema}.chunks USING GIN (text_vector);"
        );

        sqlx::raw_sql(&query).execute(pool).await.map_err(|_e| {
            #[cfg(test)]
            println!("Failed to provision the namespace: {:?}", _e);

            ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Failed to provision the namespace"),
                solution: None,
            }
        })?;

        Ok(())
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
            created_at: Utc::now(),
        };

        assert_eq!(namespace.schema(), "ns_f47ac10b58cc");
    }
}
