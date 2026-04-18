use crate::errors::Result;
use crate::hf::EmbeddingService;
use crate::models::{AnalysisResult, DocumentPair, LegalDocument};
use tracing::info;

pub struct LegalAnalyzer {
    embedding_service: EmbeddingService,
    similarity_threshold: f32,
    contradiction_threshold: f32,
}

impl LegalAnalyzer {
    pub fn new(hf_token: &str, similarity_threshold: f32) -> Self {
        Self {
            embedding_service: EmbeddingService::new(
                hf_token,
                "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2",
            ),
            similarity_threshold,
            contradiction_threshold: 0.5,
        }
    }

    pub fn with_cross_encoder(hf_token: &str, similarity_threshold: f32) -> Self {
        Self {
            embedding_service: EmbeddingService::new(
                hf_token,
                "cross-encoder/stsb-distilroberta-base",
            ),
            similarity_threshold,
            contradiction_threshold: 0.5,
        }
    }

    pub async fn analyze(&self, documents: Vec<LegalDocument>) -> Result<AnalysisResult> {
        info!("Analyzing {} documents", documents.len());

        if documents.len() < 2 {
            return Ok(AnalysisResult {
                total_documents: documents.len(),
                total_pairs_analyzed: 0,
                similar_pairs: Vec::new(),
                contradictions: Vec::new(),
                analyzed_at: chrono::Utc::now().to_rfc3339(),
            });
        }

        let pairs = self.generate_pairs(&documents);
        info!("Generated {} document pairs", pairs.len());

        let text_pairs: Vec<(&str, &str)> = pairs
            .iter()
            .map(|(d1, d2)| (d1.text.as_str(), d2.text.as_str()))
            .collect();

        let scores = self
            .embedding_service
            .get_similarity_scores(text_pairs)
            .await?;

        let mut similar_pairs = Vec::with_capacity(pairs.len());
        let mut contradictions = Vec::with_capacity(pairs.len());

        for ((doc1, doc2), score) in pairs.iter().zip(scores.iter()) {
            let pair = DocumentPair::new(doc1, doc2, *score, self.contradiction_threshold);

            if pair.is_similar(self.similarity_threshold) {
                if pair.is_contradiction {
                    contradictions.push(pair);
                } else {
                    similar_pairs.push(pair);
                }
            }
        }

        info!(
            "Found {} similar pairs, {} contradictions",
            similar_pairs.len(),
            contradictions.len()
        );

        Ok(AnalysisResult {
            total_documents: documents.len(),
            total_pairs_analyzed: pairs.len(),
            similar_pairs,
            contradictions,
            analyzed_at: chrono::Utc::now().to_rfc3339(),
        })
    }

    fn generate_pairs<'a>(
        &'a self,
        documents: &'a [LegalDocument],
    ) -> Vec<(&'a LegalDocument, &'a LegalDocument)> {
        let mut pairs = Vec::new();

        for i in 0..documents.len() {
            for j in (i + 1)..documents.len() {
                pairs.push((&documents[i], &documents[j]));
            }
        }

        pairs
    }
}
