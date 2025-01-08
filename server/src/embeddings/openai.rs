use super::*;

const models: [&str; 3] = [
    "text-embedding-ada-002",
    "text-embedding-3-small",
    "text-embedding-3-large",
];

pub struct EmbeddingOpenAI {
    model: String,
    secret: String,
}

impl EmbeddingOpenAI {
    pub fn new(model: impl AsRef<str>) -> Result<Self, ErrorResponse> {
        let model = model.as_ref();
        if !models.contains(&model) {
            return Err(ErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: "Please provide a supported model.".to_string(),
                solution: Some(format!("Available models: {models:?}.",)),
            });
        }

        let secret = env::var("OPENAI_API_KEY").map_err(|_| ErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to retrieve the OpenAI API key.".to_string(),
            solution: None,
        })?;

        Ok(EmbeddingOpenAI {
            model: model.to_string(),
            secret,
        })
    }
}

#[async_trait]
impl EmbeddingModel for EmbeddingOpenAI {
    async fn generate(&self, text: &str) -> Result<DenseVector, ErrorResponse> {
        unimplemented!()
    }
}
