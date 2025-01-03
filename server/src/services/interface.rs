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
use uuid::Uuid;

pub fn create_router(service: Arc<Service>) -> Router {
    Router::new()
        .route("/", get(heartbeat))
        .route("/namespaces", post(create_namespace))
        .route("/namespaces/:name", delete(remove_namespace))
        .route("/namespaces/:name/documents", post(upload_document))
        .route("/namespaces/:name/documents/:id", delete(remove_document))
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
    }

    let namespace = service.create_namespace(&payload.name, &config).await?;
    tracing::info!("A new namespace is created: {}", &namespace.name);

    Ok(SuccessResponse {
        code: StatusCode::CREATED,
        data: namespace,
    })
}

async fn remove_namespace(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Path(name): Path<String>,
) -> Result<SuccessResponse<Option<Namespace>>, ErrorResponse> {
    service.validate_secret(bearer.token())?;

    let namespace = service.remove_namespace(name).await?;
    if let Some(namespace) = &namespace {
        tracing::info!("A namespace is removed: {}", &namespace.name);
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
    let key = format!("{}/{}.pdf", namespace.schema(), document.id);
    service.storage.upload(key, data).await?;

    Ok(SuccessResponse {
        code: StatusCode::CREATED,
        data: document,
    })
}

pub async fn remove_document(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Path((name, id)): Path<(String, String)>,
) -> Result<SuccessResponse<Option<Document>>, ErrorResponse> {
    service.validate_secret(bearer.token())?;
    let namespace = service.get_namespace(name).await?;

    let id = Uuid::parse_str(&id).map_err(|_| ErrorResponse {
        code: StatusCode::BAD_REQUEST,
        message: "Please provide a valid document ID.".to_string(),
        solution: Some(String::from(
            "A document ID should be in the form of UUID.",
        )),
    })?;

    let key = format!("{}/{}.pdf", namespace.schema(), id);
    service.storage.remove(key).await?;
    let document = service.remove_document(&namespace, &id).await?;

    Ok(SuccessResponse {
        code: StatusCode::OK,
        data: document,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::multipart::{MultipartForm, Part};
    use axum_test::TestServer;
    use dotenv::dotenv;
    use std::fs::File;
    use std::io::{BufReader, Read};

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
                "index": { "m": 16, "ef_construction": 64 }
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
    }

    async fn setup() -> TestServer {
        dotenv().ok();

        let config = Configuration::default();
        let state = Arc::new(Service::new(&config).await);

        let namespaces: Vec<Namespace> =
            sqlx::query_as("SELECT * FROM namespaces")
                .fetch_all(&state.pool)
                .await
                .unwrap();

        for namespace in namespaces {
            state.remove_namespace(&namespace.name).await.unwrap();
        }

        state
            .create_namespace("existing_ns", &NamespaceConfig::default())
            .await
            .unwrap();

        TestServer::new(create_router(state)).unwrap()
    }
}
