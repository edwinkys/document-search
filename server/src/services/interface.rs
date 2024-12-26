use super::*;

#[derive(Serialize)]
pub struct GetRootResponse {
    version: String,
}

pub async fn get_root() -> axum::Json<GetRootResponse> {
    axum::Json(GetRootResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
