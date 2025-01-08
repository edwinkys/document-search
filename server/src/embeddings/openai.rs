use super::*;

pub struct EmbeddingOpenAI {
    model: String,
    secret: String,
}

impl EmbeddingOpenAI {
    pub fn new(model: impl AsRef<str>) -> Self {
        let secret = env::var("OPENAI_API_KEY")
            .expect("The OPENAI_API_KEY environment variable is not set");

        EmbeddingOpenAI {
            model: model.as_ref().to_string(),
            secret,
        }
    }
}

#[async_trait]
impl EmbeddingModel for EmbeddingOpenAI {
    async fn generate(&self, text: &str) -> Result<DenseVector, ErrorResponse> {
        unimplemented!()
    }
}
