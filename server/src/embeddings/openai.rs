use super::*;

const MODELS: [&str; 3] = [
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
        if !MODELS.contains(&model) {
            return Err(ErrorResponse {
                code: StatusCode::BAD_REQUEST,
                message: "Please provide a supported model.".to_string(),
                solution: Some(format!("Available models: {MODELS:?}.",)),
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
        let body = json!({
            "model": self.model,
            "input": text,
        });

        let response = Client::new()
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", &format!("Bearer {}", &self.secret))
            .json(&body)
            .send()
            .await
            .map_err(|_| ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to send the request to OpenAI.".to_string(),
                solution: None,
            })?;

        let json: Value = response.json().await.unwrap();
        if let Some(embedding) = json["data"][0]["embedding"].as_array() {
            let embedding: Vec<f32> = embedding
                .iter()
                .map(|value| value.as_f64().unwrap() as f32)
                .collect();

            return Ok(embedding);
        }

        Err(ErrorResponse {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to generate an embedding with OpenAI.".to_string(),
            solution: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_generate() {
        dotenv().ok();
        let model = EmbeddingOpenAI::new("text-embedding-ada-002").unwrap();
        let embedding = model.generate("Hello, world!").await.unwrap();
        assert_eq!(embedding.len(), 1536);
    }
}
