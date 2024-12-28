from fastapi.testclient import TestClient
from src.core.api import api

client = TestClient(api)

def test_get_root():
    response = client.get("/")
    assert response.status_code == 200
    assert response.json().get("version") is not None
