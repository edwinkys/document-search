import os
import boto3
from uuid import UUID

TMP_PATH = "dl-tmp"


class HeartbeatResponse:
    version: str

    def __init__(self, version: str):
        self.version = version


class Chunk:
    page: int
    headings: list[str]
    content: str

    def __init__(self, page: int, headings: list[str], content: str):
        self.page = page
        self.headings = headings
        self.content = content


class ExtractionTask:
    namespace: str
    document_key: str
    document_id: UUID

    def __init__(self, namespace: str, document_key: str, document_id: UUID):
        self.namespace = namespace
        self.document_key = document_key
        self.document_id = document_id

    def download_document(self) -> str:
        os.makedirs(TMP_PATH, exist_ok=True)
        filename = self.document_key.split("/")[-1]
        dest = os.path.join(TMP_PATH, filename)

        # This value is guaranteed to exist when the program is run.
        bucket = os.getenv("DL_BUCKET_NAME")

        # Download the document from the bucket.
        s3 = boto3.client("s3")
        s3.download_file(bucket, self.document_key, dest)

        return dest

    def cleanup(self):
        # Remove the downloaded document.
        filename = self.document_key.split("/")[-1]
        os.remove(os.path.join(TMP_PATH, filename))
