use crate::errors::{AppError, Result};
use crate::rust_ml::RustMLService;
use tracing::info;

pub struct EmbeddingService {
    client: reqwest::Client,
    hf_token: String,
    rust_ml: RustMLService,
}

impl EmbeddingService {
    pub fn new(hf_token: &str, _model_name: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            hf_token: hf_token.to_string(),
            rust_ml: RustMLService::new(),
        }
    }

    pub async fn get_embeddings(
        &self,
        texts: Vec<&str>,
        _model_name: &str,
    ) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        info!("Computing embeddings for {} texts using Rust ML", texts.len());
        
        let embeddings = self.rust_ml.get_embeddings(texts);
        info!("Computed {} embeddings", embeddings.len());

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

        // Choose service based on model type
        if model_name.contains("cross-encoder") {
            self.rust_ml.get_cross_encoder_scores(pairs).await
        } else {
            self.rust_ml.get_similarity_scores(pairs).await
        }
    }
}
