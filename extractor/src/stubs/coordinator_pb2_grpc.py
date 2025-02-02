# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc
import warnings

from . import coordinator_pb2 as coordinator__pb2
from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2

GRPC_GENERATED_VERSION = "1.67.0"
GRPC_VERSION = grpc.__version__
_version_not_supported = False

try:
    from grpc._utilities import first_version_is_lower

    _version_not_supported = first_version_is_lower(
        GRPC_VERSION, GRPC_GENERATED_VERSION
    )
except ImportError:
    _version_not_supported = True

if _version_not_supported:
    raise RuntimeError(
        f"The grpc package installed is at version {GRPC_VERSION},"
        + f" but the generated code in coordinator_pb2_grpc.py depends on"
        + f" grpcio>={GRPC_GENERATED_VERSION}."
        + f" Please upgrade your grpc module to grpcio>={GRPC_GENERATED_VERSION}"
        + f" or downgrade your generated code using grpcio-tools<={GRPC_VERSION}."
    )


class CoordinatorStub(object):
    """Coordinator service definition.

    The coordinator service is used to coordinate document ingestion and
    retrieval workflows with the extractor workers. We choose gRPC for this
    service because Protobuf provides effcient data transfer between the
    coordinator and the workers.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Heartbeat = channel.unary_unary(
            "/coordinator.Coordinator/Heartbeat",
            request_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
            response_deserializer=coordinator__pb2.HeartbeatResponse.FromString,
            _registered_method=True,
        )
        self.RegisterWorker = channel.unary_unary(
            "/coordinator.Coordinator/RegisterWorker",
            request_serializer=coordinator__pb2.RegisterWorkerRequest.SerializeToString,
            response_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            _registered_method=True,
        )
        self.UpdateDocument = channel.unary_unary(
            "/coordinator.Coordinator/UpdateDocument",
            request_serializer=coordinator__pb2.UpdateDocumentRequest.SerializeToString,
            response_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            _registered_method=True,
        )
        self.CreateChunk = channel.unary_unary(
            "/coordinator.Coordinator/CreateChunk",
            request_serializer=coordinator__pb2.CreateChunkRequest.SerializeToString,
            response_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            _registered_method=True,
        )


class CoordinatorServicer(object):
    """Coordinator service definition.

    The coordinator service is used to coordinate document ingestion and
    retrieval workflows with the extractor workers. We choose gRPC for this
    service because Protobuf provides effcient data transfer between the
    coordinator and the workers.
    """

    def Heartbeat(self, request, context):
        """Checks if the coordinator service is running."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def RegisterWorker(self, request, context):
        """Adds an extraction worker to the coordinator service."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def UpdateDocument(self, request, context):
        """Updates the document record in the database."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")

    def CreateChunk(self, request, context):
        """Creates chunk records from the extracted document content."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details("Method not implemented!")
        raise NotImplementedError("Method not implemented!")


def add_CoordinatorServicer_to_server(servicer, server):
    rpc_method_handlers = {
        "Heartbeat": grpc.unary_unary_rpc_method_handler(
            servicer.Heartbeat,
            request_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            response_serializer=coordinator__pb2.HeartbeatResponse.SerializeToString,
        ),
        "RegisterWorker": grpc.unary_unary_rpc_method_handler(
            servicer.RegisterWorker,
            request_deserializer=coordinator__pb2.RegisterWorkerRequest.FromString,
            response_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
        ),
        "UpdateDocument": grpc.unary_unary_rpc_method_handler(
            servicer.UpdateDocument,
            request_deserializer=coordinator__pb2.UpdateDocumentRequest.FromString,
            response_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
        ),
        "CreateChunk": grpc.unary_unary_rpc_method_handler(
            servicer.CreateChunk,
            request_deserializer=coordinator__pb2.CreateChunkRequest.FromString,
            response_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
        ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
        "coordinator.Coordinator", rpc_method_handlers
    )
    server.add_generic_rpc_handlers((generic_handler,))
    server.add_registered_method_handlers(
        "coordinator.Coordinator", rpc_method_handlers
    )


# This class is part of an EXPERIMENTAL API.
class Coordinator(object):
    """Coordinator service definition.

    The coordinator service is used to coordinate document ingestion and
    retrieval workflows with the extractor workers. We choose gRPC for this
    service because Protobuf provides effcient data transfer between the
    coordinator and the workers.
    """

    @staticmethod
    def Heartbeat(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/coordinator.Coordinator/Heartbeat",
            google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
            coordinator__pb2.HeartbeatResponse.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True,
        )

    @staticmethod
    def RegisterWorker(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/coordinator.Coordinator/RegisterWorker",
            coordinator__pb2.RegisterWorkerRequest.SerializeToString,
            google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True,
        )

    @staticmethod
    def UpdateDocument(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/coordinator.Coordinator/UpdateDocument",
            coordinator__pb2.UpdateDocumentRequest.SerializeToString,
            google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True,
        )

    @staticmethod
    def CreateChunk(
        request,
        target,
        options=(),
        channel_credentials=None,
        call_credentials=None,
        insecure=False,
        compression=None,
        wait_for_ready=None,
        timeout=None,
        metadata=None,
    ):
        return grpc.experimental.unary_unary(
            request,
            target,
            "/coordinator.Coordinator/CreateChunk",
            coordinator__pb2.CreateChunkRequest.SerializeToString,
            google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            options,
            channel_credentials,
            insecure,
            call_credentials,
            compression,
            wait_for_ready,
            timeout,
            metadata,
            _registered_method=True,
        )
