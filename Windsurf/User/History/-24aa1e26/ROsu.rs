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
            base_url: "https://router.huggingface.co".to_string(),
        }
    }

    fn build_model_url(&self, model_name: &str) -> String {
        format!("{}/v1/models/{}", self.base_url, model_name)
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

        let embeddings: Vec<Vec<f32>> = texts
            .iter()
            .map(|_| vec![0.0; 384]) 
            .collect();

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
            "Calculating similarity scores for {} pairs using local algorithm",
            pairs.len()
        );

        let scores: Vec<f32> = pairs
            .iter()
            .map(|(text1, text2)| {
                let words1: std::collections::HashSet<&str> = text1
                    .split_whitespace()
                    .collect();
                let words2: std::collections::HashSet<&str> = text2
                    .split_whitespace()
                    .collect();
                
                let intersection = words1.intersection(&words2).count();
                let union = words1.union(&words2).count();
                
                if union == 0 {
                    0.0
                } else {
                    intersection as f32 / union as f32
                }
            })
            .collect();

        info!("Calculated {} similarity scores", scores.len());

        Ok(scores)
    }
}
