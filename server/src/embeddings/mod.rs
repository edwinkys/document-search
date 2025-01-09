mod openai;

pub use openai::EmbeddingOpenAI;

use crate::services::interface::ErrorResponse;
use async_trait::async_trait;
use axum::http::StatusCode;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::env;

pub type DenseVector = Vec<f32>;

#[async_trait]
pub trait EmbeddingModel: Send + Sync {
    /// Generates a vector embedding for the given text.
    async fn generate(&self, text: &str) -> Result<DenseVector, ErrorResponse>;
}
