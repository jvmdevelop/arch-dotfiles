use crate::errors::{AppError, Result};
use crate::rust_ml::RustMLService;
use tracing::info;

pub struct EmbeddingService {
    ml_service: RustMLService,
    hf_token: String,
}

impl EmbeddingService {
    pub fn new(hf_token: &str, _model_name: &str) -> Self {
        Self {
            ml_service: RustMLService::new(),
            hf_token: hf_token.to_string(),
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
        
        let embeddings = self.ml_service.get_embeddings(texts);
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

        info!("Computing similarity scores for {} pairs using Rust ML", pairs.len());

        let scores = if model_name.contains("cross-encoder") {
            self.ml_service.get_cross_encoder_scores(pairs)
        } else {
            self.ml_service.get_similarity_scores(pairs)
        };

        Ok(scores)
    }

    pub fn train_on_documents(&mut self, documents: &[String]) {
        info!("Training ML model on {} documents", documents.len());
        self.ml_service.train_on_documents(documents);
    }
}
