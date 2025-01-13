use super::*;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Json, Multipart, Path, State};
use axum::http::Response;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::Router;
use axum_extra::headers::authorization::{Authorization, Bearer};
use axum_extra::TypedHeader;
use regex::bytes::Regex;
use serde_json::json;

pub fn create_router(service: Arc<Service>) -> Router {
    Router::new()
        .route("/", get(heartbeat))
        .route("/namespaces", post(create_namespace))
        .route("/namespaces/:name", delete(remove_namespace))
        .route("/namespaces/:name/documents", post(upload_document))
        .route("/namespaces/:name/documents/:id", delete(remove_document))
        .route("/namespaces/:name/queries", post(create_query))
        .layer(DefaultBodyLimit::max(64 * 1024 * 1024))
        .with_state(service)
}

#[derive(Debug)]
pub struct SuccessResponse<T: Serialize> {
    pub code: StatusCode,
    pub data: T,
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response<Body> {
        (self.code, Json(self.data)).into_response()
    }
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub code: StatusCode,
    pub message: String,
    pub solution: Option<String>,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        let error = json!({
            "message": self.message,
            "solution": self.solution
        });

        (self.code, Json(error)).into_response()
    }
}

#[derive(Serialize)]
struct HeartbeatResponse {
    pub version: String,
}

#[derive(Deserialize)]
struct CreateNamespacePayload {
    pub name: String,
    pub config: Option<Value>,
}

#[derive(Deserialize)]
struct CreateQueryPayload {
    pub query: String,
    pub k: Option<usize>,
}

async fn heartbeat() -> SuccessResponse<HeartbeatResponse> {
    SuccessResponse {
        code: StatusCode::OK,
        data: HeartbeatResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
    }
}

async fn create_namespace(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Json(payload): Json<CreateNamespacePayload>,
) -> Result<SuccessResponse<Namespace>, ErrorResponse> {
    service.validate_secret(bearer.token())?;

    // Validate the namespace name.
    let re = Regex::new(r"^[a-z_]+$").unwrap();
    if !re.is_match(payload.name.as_bytes()) {
        return Err(ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: "Invalid name for a namespace".to_string(),
            solution: Some(String::from(
                "A namespace must be lowercase letters with underscores.",
            )),
        });
    }

    let mut config = NamespaceConfig::default();
    if let Some(conf) = &payload.config {
        config = serde_json::from_value(conf.clone()).map_err(|_e| {
            #[cfg(test)]
            eprintln!("Failed to parse the namespace configuration: {_e:?}");
            ErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: "Please provide a valid configuration.".to_string(),
                solution: Some(String::from(
                    "Check the documentation for the available options.",
                )),
            }
        })?;

        // This validates the provided embedding model to be valid.
        config.embedding.model()?;
    }

    let namespace = service.create_namespace(&payload.name, &config).await?;
    tracing::info!("NamespaceCreated: {namespace:?}");

    Ok(SuccessResponse {
        code: StatusCode::CREATED,
        data: namespace,
    })
}

async fn remove_namespace(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Path(namespace): Path<String>,
) -> Result<SuccessResponse<Option<Namespace>>, ErrorResponse> {
    service.validate_secret(bearer.token())?;

    let namespace = service.remove_namespace(namespace).await?;
    if let Some(namespace) = &namespace {
        tracing::info!("NamespaceRemoved: {namespace:?}");
    }

    Ok(SuccessResponse {
        code: StatusCode::OK,
        data: namespace,
    })
}

async fn upload_document(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Path(namespace): Path<String>,
    mut multipart: Multipart,
) -> Result<SuccessResponse<Document>, ErrorResponse> {
    service.validate_secret(bearer.token())?;
    let namespace = service.get_namespace(&namespace).await?;

    let mut data: Vec<u8> = Vec::new();
    let mut metadata: Value = Value::Null;

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(name) = field.name() {
            if name == "metadata" {
                let bytes = field.bytes().await.unwrap();
                metadata = serde_json::from_slice(&bytes).map_err(|_| {
                    ErrorResponse {
                        code: StatusCode::BAD_REQUEST,
                        message: "Failed to parse the metadata.".to_string(),
                        solution: None,
                    }
                })?;
            } else if name == "file" {
                data = field.bytes().await.unwrap().to_vec();
            }
        }
    }

    if data.is_empty() {
        return Err(ErrorResponse {
            code: StatusCode::BAD_REQUEST,
            message: "Please upload a valid document.".to_string(),
            solution: None,
        });
    }

    let document = service.create_document(&namespace, &metadata).await?;
    let key = document.key(&namespace);
    service.storage.upload(&key, data).await?;
    tracing::info!("DocumentCreated: {document:?}");

    let task = ExtractionTask {
        namespace: namespace.name,
        document_id: document.id,
        document_key: key,
    };

    service.queue.publish(&task).await?;

    Ok(SuccessResponse {
        code: StatusCode::CREATED,
        data: document,
    })
}

async fn remove_document(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Path((namespace, id)): Path<(String, String)>,
) -> Result<SuccessResponse<Option<Document>>, ErrorResponse> {
    service.validate_secret(bearer.token())?;
    let namespace = service.get_namespace(namespace).await?;
    let id = service.validate_uuid(&id)?;

    let document = service.remove_document(&namespace, &id).await?;
    if let Some(document) = &document {
        let key = document.key(&namespace);
        service.storage.remove(&key).await?;
        tracing::info!("DocumentRemoved: {document:?}");
    }

    Ok(SuccessResponse {
        code: StatusCode::OK,
        data: document,
    })
}

async fn create_query(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Path(namespace): Path<String>,
    Json(payload): Json<CreateQueryPayload>,
) -> Result<SuccessResponse<Vec<Chunk>>, ErrorResponse> {
    service.validate_secret(bearer.token())?;
    let namespace = service.get_namespace(namespace).await?;

    let CreateQueryPayload { query, k } = payload;
    let k = k.unwrap_or(10);

    let results = service.create_query(&namespace, query, k as u8).await?;
    Ok(SuccessResponse {
        code: StatusCode::OK,
        data: results,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::multipart::{MultipartForm, Part};
    use axum_test::TestServer;
    use dotenv::dotenv;
    use futures_lite::StreamExt;
    use lapin::options::BasicConsumeOptions;
    use lapin::types::FieldTable;
    use lapin::{Connection, ConnectionProperties};
    use protos::coordinator_server::Coordinator;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use tonic::Request;

    const BEARER: &str = "secretkey";

    #[tokio::test]
    async fn test_heartbeat() {
        let response = heartbeat().await;
        assert_eq!(response.data.version, env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn test_create_namespace() {
        let app = setup().await;
        let payload = json!({ "name": "test_ns" });
        let response = app
            .post("/namespaces")
            .authorization_bearer(BEARER)
            .json(&payload)
            .await;

        let namespace: Namespace = response.json();
        assert_eq!(namespace.name, "test_ns");
    }

    #[tokio::test]
    async fn test_create_namespace_custom_config() {
        let app = setup().await;
        let payload = json!({
            "name": "test_ns",
            "config": {
                "index": { "m": 16, "ef_construction": 64 },
                "embedding": {
                    "provider": "OpenAI",
                    "model": "text-embedding-ada-002"
                }
            }
        });

        let response = app
            .post("/namespaces")
            .authorization_bearer(BEARER)
            .json(&payload)
            .await;

        let namespace: Namespace = response.json();
        assert_eq!(namespace.name, "test_ns");
        assert_eq!(namespace.config.index.m, 16);
    }

    #[tokio::test]
    async fn test_remove_namespace() {
        let app = setup().await;
        let response = app
            .delete("/namespaces/existing_ns")
            .authorization_bearer(BEARER)
            .await;

        let namespace: Option<Namespace> = response.json();
        assert_eq!(namespace.unwrap().name, "existing_ns");
    }

    #[tokio::test]
    async fn test_upload_and_remove_document() {
        let mut buffer = Vec::new();
        let file = File::open(".cargo/example.pdf").unwrap();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buffer).unwrap();

        let metadata = json!({ "title": "Product Quantization" });

        let form = MultipartForm::new()
            .add_text("metadata", metadata.to_string())
            .add_part("file", Part::bytes(buffer));

        let app = setup().await;
        let document: Document = app
            .post("/namespaces/existing_ns/documents")
            .authorization_bearer(BEARER)
            .multipart(form)
            .await
            .json();

        assert_eq!(document.metadata, metadata);
        assert_eq!(document.status, DocumentStatus::Pending);

        let id = document.id;
        let _document: Option<Document> = app
            .delete(&format!("/namespaces/existing_ns/documents/{id}"))
            .authorization_bearer(BEARER)
            .await
            .json();

        assert!(_document.is_some());
        assert_eq!(document.id, _document.unwrap().id);

        // Check if after the document is uploaded, a task is queued up.

        let config = Configuration::default();
        let url = config.queue_url.as_str();
        let properties = ConnectionProperties::default();
        let connection = Connection::connect(url, properties).await.unwrap();

        let channel = connection.create_channel().await.unwrap();

        let options = BasicConsumeOptions::default();
        let table = FieldTable::default();
        let mut consumer = channel
            .basic_consume(QUEUE_NAME, "consumer", options, table)
            .await
            .unwrap();

        let payload = consumer.next().await.unwrap().unwrap().data;
        let task: ExtractionTask = serde_json::from_slice(&payload).unwrap();
        assert_eq!(document.id, task.document_id);
    }

    #[tokio::test]
    async fn test_create_query() {
        let app = setup_populated().await;
        let payload = json!({ "query": "Do you like banana?", "k": 2 });
        let response = app
            .post("/namespaces/existing_ns/queries")
            .authorization_bearer(BEARER)
            .json(&payload)
            .await;

        let chunks: Vec<Chunk> = response.json();
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].content.contains("Bananas"));
        assert!(chunks[1].content.contains("Oranges"));
    }

    async fn setup() -> TestServer {
        dotenv().ok();

        let config = Configuration::default();
        let state = Arc::new(Service::new(&config).await);
        teardown(state.clone()).await;

        state
            .create_namespace("existing_ns", &NamespaceConfig::default())
            .await
            .unwrap();

        TestServer::new(create_router(state)).unwrap()
    }

    async fn setup_populated() -> TestServer {
        dotenv().ok();

        let config = Configuration::default();
        let state = Arc::new(Service::new(&config).await);
        teardown(state.clone()).await;

        let namespace = state
            .create_namespace("existing_ns", &NamespaceConfig::default())
            .await
            .unwrap();

        let document = state
            .create_document(&namespace, &json!({ "key": "value" }))
            .await
            .unwrap();

        let sentences = vec![
            "Approximate nearest neighbor finds similar items fast.",
            "ANNS balances speed over perfect accuracy.",
            "Popular ANNS methods include hashing and graphs.",
            "Bananas are packed with potassium and energy.",
            "Oranges are juicy and full of vitamin C.",
        ];

        let request = protos::CreateChunkRequest {
            namespace: namespace.name.clone(),
            document_id: document.id.to_string(),
            chunks: sentences
                .iter()
                .map(|sentence| protos::Chunk {
                    page: 1,
                    content: sentence.to_string(),
                })
                .collect(),
        };

        state.create_chunk(Request::new(request)).await.unwrap();
        TestServer::new(create_router(state)).unwrap()
    }

    async fn teardown(service: Arc<Service>) {
        service.queue.purge().await.unwrap();

        sqlx::query("TRUNCATE TABLE namespaces")
            .execute(&service.database)
            .await
            .unwrap();
    }
}
