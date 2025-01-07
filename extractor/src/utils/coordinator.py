import grpc
from google.protobuf.empty_pb2 import Empty
from ..stubs import coordinator_pb2 as protos
from ..stubs.coordinator_pb2_grpc import CoordinatorStub
from .types import HeartbeatResponse, Chunk


class Coordinator:
    connection: CoordinatorStub

    def __init__(self, address: str):
        channel = grpc.insecure_channel(address)
        self.connection = CoordinatorStub(channel)

    def heartbeat(self) -> HeartbeatResponse:
        response = self.connection.Heartbeat(Empty())
        return HeartbeatResponse(version=response.version)

    def register_worker(self, id: str, address: str):
        request = protos.RegisterWorkerRequest(id=id, address=address)
        self.connection.RegisterWorker(request=request)

    def update_document(self, namespace: str, id: str, status: str):
        status = protos.DocumentStatus.Value(status.upper())
        request = protos.UpdateDocumentRequest(
            namespace=namespace,
            id=id,
            status=status,
        )

        self.connection.UpdateDocument(request=request)

    def create_chunk(self, namespace: str, id: str, chunks: list[Chunk]):
        chunks = [chunk.to_proto() for chunk in chunks]
        request = protos.CreateChunkRequest(
            namespace=namespace,
            id=id,
            chunks=chunks,
        )

        self.connection.CreateChunk(request=request)
