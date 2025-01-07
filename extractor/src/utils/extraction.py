from docling.document_converter import DocumentConverter
from docling_core.transforms.chunker.hybrid_chunker import HybridChunker
from .types import Chunk


class Extraction:
    path: str
    tokenizer: str

    def __init__(self, path: str, tokenizer: str = "BAAI/bge-small-en-v1.5"):
        self.path = path
        self.tokenizer = tokenizer

    def extract(self) -> list[Chunk]:
        converter = DocumentConverter()
        chunker = HybridChunker(tokenizer=self.tokenizer)

        result = converter.convert(self.path)
        doc = result.document

        chunks = []
        for chunk in list(chunker.chunk(doc)):
            meta = chunk.meta.export_json_dict()

            page = 0
            doc_items = meta.get("doc_items", [])
            if len(doc_items) > 0:
                provs = doc_items[0].get("prov", [])
                page = provs[0].get("page_no", 0) if len(provs) > 0 else 0

            headings = meta.get("headings", [])
            chunks.append(Chunk(page, headings, chunk.text))

        return chunks
