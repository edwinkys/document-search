use super::*;
use axum::body::Body;
use axum::extract::{Json, State};
use axum::http::Response;
use axum::response::IntoResponse;
use axum_extra::headers::authorization::{Authorization, Bearer};
use axum_extra::TypedHeader;
use regex::bytes::Regex;
use serde_json::json;

pub struct SuccessResponse<T: Serialize> {
    pub code: StatusCode,
    pub data: T,
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response<Body> {
        (self.code, Json(self.data)).into_response()
    }
}

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
pub struct HeartbeatResponse {
    pub version: String,
}

#[derive(Deserialize)]
pub struct CreateNamespaceRequest {
    pub name: String,
}

pub async fn heartbeat() -> SuccessResponse<HeartbeatResponse> {
    SuccessResponse {
        code: StatusCode::OK,
        data: HeartbeatResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
    }
}

pub async fn create_namespace(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(service): State<Arc<Service>>,
    Json(payload): Json<CreateNamespaceRequest>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::{Request, StatusCode};
    use axum::routing::{self, MethodRouter};
    use axum::Router;
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
        let payload = json!({ "name": "test_ns" });
        let request = create_request("POST", "/namespaces", &payload);

        let app = setup("/namespaces", routing::post(create_namespace)).await;
        let response = app.oneshot(request).await.unwrap();

        let status = response.status();
        assert_eq!(status, StatusCode::CREATED);

        let namespace: Namespace = consume_body(response.into_body()).await;
        assert_eq!(namespace.name, "test_ns");
    }

    async fn setup(
        path: &str,
        method_router: MethodRouter<Arc<Service>>,
    ) -> Router {
        let config = Configuration::default();
        let state = Arc::new(Service::new(&config).await);

        let namespaces: Vec<Namespace> =
            sqlx::query_as("DELETE FROM namespaces RETURNING *;")
                .fetch_all(&state.pool)
                .await
                .unwrap();

        for namespace in namespaces {
            let schema = namespace.schema();
            let q = format!("DROP SCHEMA IF EXISTS {schema} CASCADE");
            sqlx::query(q.as_str()).execute(&state.pool).await.unwrap();
        }

        Router::new().route(path, method_router).with_state(state)
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
