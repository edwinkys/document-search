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
}
