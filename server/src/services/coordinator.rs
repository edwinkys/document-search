use super::*;
use protos::coordinator_server::Coordinator;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl Coordinator for Arc<Service> {
    async fn heartbeat(
        &self,
        _request: Request<()>,
    ) -> Result<Response<protos::HeartbeatResponse>, Status> {
        Ok(Response::new(protos::HeartbeatResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }

    async fn register_worker(
        &self,
        request: Request<protos::RegisterWorkerRequest>,
    ) -> Result<Response<()>, Status> {
        let worker: Worker = request.into_inner().try_into()?;
        self.add_worker(&worker).await;
        tracing::info!("A worker is registered: {}", worker.id);
        Ok(Response::new(()))
    }

    async fn update_document(
        &self,
        request: Request<protos::UpdateDocumentRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let namespace = self.get_namespace(&request.namespace).await?;
        let id = self.validate_uuid(&request.document_id)?;
        let status = DocumentStatus::from(request.status());

        let schema = namespace.schema();
        sqlx::query(&format!(
            "UPDATE {schema}.documents
            SET status = $2, updated_at = NOW()
            WHERE id = $1;",
        ))
        .bind(id)
        .bind(&status)
        .execute(&self.database)
        .await
        .map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to update the document: {_e:?}");
            Status::internal("Failed to update the document.")
        })?;

        Ok(Response::new(()))
    }

    async fn create_chunk(
        &self,
        request: Request<protos::CreateChunkRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let namespace = self.get_namespace(&request.namespace).await?;
        let document_id = self.validate_uuid(&request.document_id)?;

        let model = namespace.config.embedding.model()?;
        let mut embeddings = Vec::new();
        for chunk in &request.chunks {
            let embedding = model.generate(&chunk.content).await?;
            embeddings.push(embedding);
        }

        let mut tx = self.database.begin().await.map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to start a transaction: {_e:?}");
            Status::internal("Failed to start a transaction.")
        })?;

        let schema = namespace.schema();
        let language = "english";
        for (chunk, embedding) in request.chunks.iter().zip(embeddings) {
            sqlx::query(&format!(
                "INSERT INTO {schema}.chunks
                (document_id, page, content, semantic_vector, text_vector) VALUES ($1, $2, $3, $4, to_tsvector('{language}', $5));",
            ))
            .bind(document_id)
            .bind(chunk.page as i32)
            .bind(&chunk.content)
            .bind(embedding)
            .bind(&chunk.content)
            .execute(&mut *tx)
            .await
            .map_err(|_e| {
                #[cfg(test)]
                eprintln!("Failed to create a chunk: {_e:?}");
                Status::internal("Failed to create a chunk.")
            })?;
        }

        sqlx::query(&format!(
            "UPDATE {schema}.documents
            SET status = $2, updated_at = NOW()
            WHERE id = $1;",
        ))
        .bind(document_id)
        .bind(DocumentStatus::Completed)
        .execute(&mut *tx)
        .await
        .map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to create a chunk: {_e:?}");
            Status::internal("Failed to create a chunk.")
        })?;

        tx.commit().await.map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to commit the transaction: {_e:?}");
            Status::internal("Failed to commit the transaction.")
        })?;

        Ok(Response::new(()))
    }
}

impl From<ErrorResponse> for Status {
    fn from(error: ErrorResponse) -> Self {
        let message = error.message;
        match error.code {
            StatusCode::NOT_FOUND => Status::not_found(message),
            StatusCode::INTERNAL_SERVER_ERROR => Status::internal(message),
            _ => Status::invalid_argument(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_heartbeat() {
        let service = setup().await;
        let request = Request::new(());
        let response = service.heartbeat(request).await.unwrap();
        assert_eq!(response.get_ref().version, env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn test_register_worker() {
        let service = setup().await;
        let request = Request::new(protos::RegisterWorkerRequest {
            id: Uuid::new_v4().to_string(),
            address: "[::]:2510".to_string(),
        });

        service.register_worker(request).await.unwrap();
        let workers = service.workers().await;
        assert_eq!(workers.len(), 1);
    }

    #[tokio::test]
    async fn test_update_document() {
        let service = setup().await;
        let namespace = setup_namespace(service.clone()).await;

        let metadata = serde_json::json!({});
        let document = service
            .create_document(&namespace, &metadata)
            .await
            .unwrap();

        let request = Request::new(protos::UpdateDocumentRequest {
            namespace: namespace.name.clone(),
            document_id: document.id.to_string(),
            status: protos::DocumentStatus::Processing as i32,
        });

        service.update_document(request).await.unwrap();

        let schema = namespace.schema();
        let _document: Document = sqlx::query_as(&format!(
            "SELECT * FROM {schema}.documents
            WHERE id = $1;",
        ))
        .bind(&document.id)
        .fetch_one(&service.database)
        .await
        .unwrap();

        assert_eq!(_document.status, DocumentStatus::Processing);
    }

    #[tokio::test]
    async fn test_create_chunk() {
        let service = setup().await;
        let namespace = setup_namespace(service.clone()).await;

        let metadata = serde_json::json!({});
        let document = service
            .create_document(&namespace, &metadata)
            .await
            .unwrap();

        let request = Request::new(protos::CreateChunkRequest {
            namespace: namespace.name.clone(),
            document_id: document.id.to_string(),
            chunks: vec![protos::Chunk {
                page: 1,
                content: "DocuLens is a robust search API platform for PDFs."
                    .to_string(),
            }],
        });

        service.create_chunk(request).await.unwrap();

        let schema = namespace.schema();
        let chunks: Vec<Chunk> = sqlx::query_as(&format!(
            "SELECT id, document_id, page, content FROM {schema}.chunks
            WHERE document_id = $1;",
        ))
        .bind(&document.id)
        .fetch_all(&service.database)
        .await
        .unwrap();

        assert_eq!(chunks.len(), 1);
    }

    async fn setup() -> Arc<Service> {
        dotenv().ok();
        let config = Configuration::default();
        Arc::new(Service::new(&config).await)
    }

    async fn setup_namespace(service: Arc<Service>) -> Namespace {
        let name = "coordinator_ns";
        let config = NamespaceConfig::default();
        let _ = service.remove_namespace(name).await;
        service.create_namespace(name, &config).await.unwrap()
    }
}
