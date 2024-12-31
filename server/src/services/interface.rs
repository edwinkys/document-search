use super::*;
use axum::body::Body;
use axum::extract::{Json, State};
use axum::http::Response;
use axum::response::IntoResponse;
use axum_extra::headers::authorization::{Authorization, Bearer};
use axum_extra::TypedHeader;
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
) -> Result<SuccessResponse<()>, ErrorResponse> {
    service.validate_secret(bearer.token())?;
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_heartbeat() {
        let response = heartbeat().await;
        assert_eq!(response.data.version, env!("CARGO_PKG_VERSION"));
    }
}
