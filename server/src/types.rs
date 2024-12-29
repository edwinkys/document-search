use crate::protos;
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
