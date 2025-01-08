mod openai;

pub use openai::EmbeddingOpenAI;

use crate::services::interface::ErrorResponse;
use async_trait::async_trait;
use std::env;

pub type DenseVector = Vec<f32>;

#[async_trait]
pub trait EmbeddingModel: Send + Sync {
    /// Generates a vector embedding for the given text.
    async fn generate(&self, text: &str) -> Result<DenseVector, ErrorResponse>;
}
