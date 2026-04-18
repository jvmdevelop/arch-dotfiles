use crate::errors::{AppError, Result};
use tracing::info;

pub struct EmbeddingService {
    client: reqwest::Client,
    hf_token: String,
    base_url: String,
}

impl EmbeddingService {
    pub fn new(hf_token: &str, _model_name: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            hf_token: hf_token.to_string(),
            base_url: "https://api-inference.huggingface.co".to_string(),
        }
    }

    fn build_model_url(&self, model_name: &str) -> String {
        format!("{}/models/{}", self.base_url, model_name)
    }

    pub async fn get_embeddings(
        &self,
        texts: Vec<&str>,
        model_name: &str,
    ) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        info!(
            "Requesting embeddings for {} texts from {}",
            texts.len(),
            model_name
        );

        #[derive(serde::Serialize)]
        struct EmbeddingRequest {
            inputs: Vec<String>,
        }

        let request = EmbeddingRequest {
            inputs: texts.iter().map(|s| s.to_string()).collect(),
        };

        let model_url = self.build_model_url(model_name);

        let response = self
            .client
            .post(&model_url)
            .header("Authorization", format!("Bearer {}", self.hf_token))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(60))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let status_code = status.as_u16();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error body".to_string());
            return Err(AppError::HuggingFace(format!(
                "API error [{}]: {}",
                status_code, error_text
            )));
        }

        let embeddings: Vec<Vec<f32>> = response.json().await?;
        info!("Received {} embeddings", embeddings.len());

        Ok(embeddings)
    }

    pub async fn get_similarity_scores(
        &self,
        pairs: Vec<(&str, &str)>,
        model_name: &str,
    ) -> Result<Vec<f32>> {
        if pairs.is_empty() {
            return Ok(Vec::new());
        }

        info!(
            "Requesting similarity scores for {} pairs from {}",
            pairs.len(),
            model_name
        );

        #[derive(serde::Serialize)]
        struct SimilarityRequest {
            inputs: Vec<(String, String)>,
        }

        let request = SimilarityRequest {
            inputs: pairs
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect(),
        };

        let model_url = self.build_model_url(model_name);

        let response = self
            .client
            .post(&model_url)
            .header("Authorization", format!("Bearer {}", self.hf_token))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(60))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let status_code = status.as_u16();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error body".to_string());
            return Err(AppError::HuggingFace(format!(
                "API error [{}]: {}",
                status_code, error_text
            )));
        }

        let scores: Vec<f32> = response.json().await?;
        info!("Received {} similarity scores", scores.len());

        Ok(scores)
    }
}
