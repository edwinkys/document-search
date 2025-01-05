from uuid import UUID


class HeartbeatResponse:
    version: str

    def __init__(self, version: str):
        self.version = version

    def __str__(self):
        return f"version: {self.version}"


class ExtractionTask:
    namespace: str
    document_key: str
    document_id: UUID

    def __init__(self, namespace: str, document_key: str, document_id: UUID):
        self.namespace = namespace
        self.document_key = document_key
        self.document_id = document_id
