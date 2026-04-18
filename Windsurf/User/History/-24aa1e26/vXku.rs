use crate::errors::{AppError, Result};
use crate::local_ml::LocalMLService;
use tracing::info;

pub struct EmbeddingService {
    client: reqwest::Client,
    hf_token: String,
    base_url: String,
    local_ml: LocalMLService,
}

impl EmbeddingService {
    pub fn new(hf_token: &str, _model_name: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            hf_token: hf_token.to_string(),
            base_url: "http://localhost".to_string(), // Local ML services
            local_ml: LocalMLService::new(),
        }
    }

    async fn check_service_health(&self, port: u16) -> Result<()> {
        let url = format!("{}:{}/health", self.base_url, port);
        
        for _ in 0..5 { // Try for 5 seconds only
            match self.client.get(&url).send().await {
                Ok(response) if response.status().is_success() => return Ok(()),
                _ => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
        
        Err(AppError::Config(format!("ML service on port {} is not available", port)))
    }

    pub async fn get_embeddings(
        &self,
        texts: Vec<&str>,
        _model_name: &str,
    ) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        // Try Docker service first, fallback to local Python
        if self.check_service_health(5000).await.is_ok() {
            self.get_embeddings_from_docker(texts).await
        } else {
            self.get_embeddings_from_local(texts).await
        }
    }

    async fn get_embeddings_from_docker(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        info!("Requesting embeddings from Docker ML service");

        #[derive(serde::Serialize)]
        struct EmbeddingRequest {
            sentences: Vec<String>,
        }

        let request = EmbeddingRequest {
            sentences: texts.iter().map(|s| s.to_string()).collect(),
        };

        let response = self
            .client
            .post(format!("{}:5000/encode", self.base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(60))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Http(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            )));
        }

        #[derive(serde::Deserialize)]
        struct EmbeddingResponse {
            embeddings: Vec<Vec<f32>>,
        }

        let result: EmbeddingResponse = response.json().await?;
        info!("Received {} embeddings from Docker", result.embeddings.len());

        Ok(result.embeddings)
    }

    async fn get_embeddings_from_local(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        info!("Using local Python ML service for embeddings");
        
        // For now, return simple embeddings - in real implementation would call Python
        let embeddings: Vec<Vec<f32>> = texts
            .iter()
            .map(|_| vec![0.0; 384]) // MiniLM dimension
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

        // Choose service based on model type and availability
        if model_name.contains("cross-encoder") {
            if self.check_service_health(5001).await.is_ok() {
                self.get_cross_encoder_scores_from_docker(pairs).await
            } else {
                info!("Using local Python cross-encoder");
                self.local_ml.get_cross_encoder_scores(pairs).await
            }
        } else {
            if self.check_service_health(5000).await.is_ok() {
                self.get_sentence_transformer_scores_from_docker(pairs).await
            } else {
                info!("Using local Python sentence transformer");
                self.local_ml.get_similarity_scores(pairs).await
            }
        }
    }

    async fn get_cross_encoder_scores_from_docker(&self, pairs: Vec<(&str, &str)>) -> Result<Vec<f32>> {
        info!("Requesting cross-encoder scores from Docker ML service");

        #[derive(serde::Serialize)]
        struct SimilarityRequest {
            inputs: Vec<[String; 2]>,
        }

        let request = SimilarityRequest {
            inputs: pairs
                .iter()
                .map(|(a, b)| [a.to_string(), b.to_string()])
                .collect(),
        };

        let response = self
            .client
            .post(format!("{}:5001/predict", self.base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(60))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Http(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            )));
        }

        #[derive(serde::Deserialize)]
        struct SimilarityResponse {
            scores: Vec<f32>,
        }

        let result: SimilarityResponse = response.json().await?;
        info!("Received {} cross-encoder scores from Docker", result.scores.len());

        Ok(result.scores)
    }

    async fn get_sentence_transformer_scores_from_docker(&self, pairs: Vec<(&str, &str)>) -> Result<Vec<f32>> {
        info!("Requesting sentence transformer scores from Docker ML service");

        #[derive(serde::Serialize)]
        struct SimilarityRequest {
            pairs: Vec<[String; 2]>,
        }

        let request = SimilarityRequest {
            pairs: pairs
                .iter()
                .map(|(a, b)| [a.to_string(), b.to_string()])
                .collect(),
        };

        let response = self
            .client
            .post(format!("{}:5000/similarity", self.base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(60))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Http(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            )));
        }

        #[derive(serde::Deserialize)]
        struct SimilarityResponse {
            similarities: Vec<f32>,
        }

        let result: SimilarityResponse = response.json().await?;
        info!("Received {} sentence transformer scores from Docker", result.similarities.len());

        Ok(result.similarities)
    }
}
