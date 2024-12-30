import importlib.metadata
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.middleware.gzip import GZipMiddleware
from fastapi.responses import JSONResponse


CORS = {
    "allow_origins": ["*"],
    "allow_methods": ["*"],
    "allow_headers": ["*"],
    "allow_credentials": True,
}

api = FastAPI()
api.add_middleware(GZipMiddleware)
api.add_middleware(CORSMiddleware, **CORS)


@api.get("/")
async def heartbeat():
    version = importlib.metadata.version("dl-extractor")
    return JSONResponse(content={"version": version})
