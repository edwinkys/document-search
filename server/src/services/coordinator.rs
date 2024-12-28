use super::*;
use protos::coordinator_server::Coordinator;

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
        _request: Request<protos::RegisterWorkerRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_heartbeat() {
        let service = setup().await;
        let request = Request::new(());
        let response = service.heartbeat(request).await.unwrap();
        assert_eq!(response.get_ref().version, env!("CARGO_PKG_VERSION"));
    }

    async fn setup() -> Arc<Service> {
        let config = Configuration::default();
        Arc::new(Service::new(&config).await)
    }
}
