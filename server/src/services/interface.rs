use super::*;
use axum::body::Body;
use axum::extract::{Json, Multipart, Path, State};
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

    let namespace = service.create_namespace(&payload.name).await?;
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

    // Upload the document to the storage.

    // Create a new document record in the database.

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::{Request, StatusCode};
    use dotenv::dotenv;
    use serde::de::DeserializeOwned;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_heartbeat() {
        let response = heartbeat().await;
        assert_eq!(response.data.version, env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn test_create_namespace() {
        let app = setup().await;
        let payload = json!({ "name": "test_ns" });
        let request = create_request("POST", "/namespaces", &payload);
        let response = app.oneshot(request).await.unwrap();

        let status = response.status();
        assert_eq!(status, StatusCode::CREATED);

        let namespace: Namespace = consume_body(response.into_body()).await;
        assert_eq!(namespace.name, "test_ns");
    }

    #[tokio::test]
    async fn test_remove_namespace() {
        let uri = "/namespaces/existing_ns";
        let request = create_request("DELETE", uri, &Value::Null);

        let app = setup().await;
        let response = app.oneshot(request).await.unwrap();

        let status = response.status();
        assert_eq!(status, StatusCode::OK);

        let namespace: Option<Namespace>;
        namespace = consume_body(response.into_body()).await;
        assert_eq!(namespace.unwrap().name, "existing_ns");
    }

    async fn setup() -> Router {
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

        state.create_namespace("existing_ns").await.unwrap();
        create_router(state)
    }

    fn create_request(method: &str, uri: &str, body: &Value) -> Request<Body> {
        Request::builder()
            .method(method)
            .uri(uri)
            .header("authorization", "bearer secretkey")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    async fn consume_body<T: DeserializeOwned>(body: Body) -> T {
        let bytes = to_bytes(body, 2048).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }
}
