use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalDocument {
    pub id: String,
    pub title: String,
    pub text: String,
    pub status: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPair {
    pub doc1_id: String,
    pub doc2_id: String,
    pub doc1_title: String,
    pub doc2_title: String,
    pub similarity_score: f32,
    pub doc1_status: Option<String>,
    pub doc2_status: Option<String>,
    pub is_contradiction: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub total_documents: usize,
    pub total_pairs_analyzed: usize,
    pub similar_pairs: Vec<DocumentPair>,
    pub contradictions: Vec<DocumentPair>,
    pub analyzed_at: String,
}

impl LegalDocument {
    pub fn new(
        id: String,
        title: String,
        text: String,
        status: Option<String>,
        url: String,
    ) -> Self {
        Self {
            id,
            title,
            text,
            status,
            url,
        }
    }

    pub fn preview(&self, max_chars: usize) -> String {
        if self.text.len() <= max_chars {
            self.text.clone()
        } else {
            format!("{}...", &self.text[..max_chars])
        }
    }
}

impl DocumentPair {
    pub fn new(
        doc1: &LegalDocument,
        doc2: &LegalDocument,
        similarity_score: f32,
        contradiction_threshold: f32,
    ) -> Self {
        let is_contradiction = Self::check_contradiction(&doc1.status, &doc2.status, similarity_score, contradiction_threshold);

        Self {
            doc1_id: doc1.id.clone(),
            doc2_id: doc2.id.clone(),
            doc1_title: doc1.title.clone(),
            doc2_title: doc2.title.clone(),
            similarity_score,
            doc1_status: doc1.status.clone(),
            doc2_status: doc2.status.clone(),
            is_contradiction,
        }
    }

    fn check_contradiction(
        status1: &Option<String>,
        status2: &Option<String>,
        similarity_score: f32,
        threshold: f32,
    ) -> bool {
        if similarity_score < threshold {
            return false;
        }

        match (status1, status2) {
            (Some(s1), Some(s2)) => {
                let s1_lower = s1.to_lowercase();
                let s2_lower = s2.to_lowercase();
                
                (s1_lower.contains("действ") && s2_lower.contains("утрат")) ||
                (s1_lower.contains("утрат") && s2_lower.contains("действ")) ||
                (s1_lower.contains("отмен") && s2_lower.contains("действ")) ||
                (s1_lower.contains("действ") && s2_lower.contains("отмен"))
            }
            _ => false,
        }
    }

    pub fn is_similar(&self, threshold: f32) -> bool {
        self.similarity_score >= threshold
    }
}
