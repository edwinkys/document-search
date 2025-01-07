import warnings
from src.utils.extraction import Extraction

extraction = Extraction("./tests/static/example.pdf")


def test_extraction_extract():
    with warnings.catch_warnings():
        warnings.simplefilter("ignore", category=DeprecationWarning)
        results = extraction.extract()

    assert len(results) == 1
    assert results[0].page == 1
    assert results[0].headings == ["Product Quantization"]
