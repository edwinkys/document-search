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
        tracing::info!("A worker is registered: {}", worker.id.0);
        Ok(Response::new(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    async fn setup() -> Arc<Service> {
        let config = Configuration::default();
        Arc::new(Service::new(&config).await)
    }
}
