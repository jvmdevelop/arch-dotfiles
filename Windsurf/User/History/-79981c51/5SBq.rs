use crate::errors::Result;
use ndarray::{Array1, Array2};
use std::collections::HashMap;

// Text tokenizer for legal documents
pub struct LegalTokenizer {
    vocab: HashMap<String, usize>,
    vocab_reverse: HashMap<usize, String>,
    max_length: usize,
}

impl LegalTokenizer {
    pub fn new() -> Self {
        let mut vocab = HashMap::new();
        let mut vocab_reverse = HashMap::new();
        
        // Basic legal vocabulary
        let legal_terms = vec![
            "закон", "статья", "кодекс", "конституция", "указ", "постановление",
            "документ", "норма", "право", "обязанность", "ответственность",
            "гражданин", "государство", "орган", "власть", "суд", "правосудие",
            "договор", "соглашение", "регламент", "инструкция", "правило",
            "пункт", "часть", "раздел", "глава", "параграф", "подпункт",
            "действие", "применение", "нарушение", "соблюдение", "исполнение",
            "контроль", "надзор", "проверка", "ответственность", "взыскание",
            "штраф", "наказание", "мера", "санкция", "прекращение", "отмена",
            // Add common words
            "и", "в", "на", "с", "по", "о", "от", "до", "для", "под", "над",
            "при", "об", "без", "к", "у", "за", "через", "между", "во", "со",
            "из", "относительно", "вследствие", "благодаря", "несмотря",
            "не", "ни", "нет", "да", "или", "либо", "то", "ли", "же", "бы",
            "если", "когда", "пока", "как", "что", "чтобы", "зачем", "почему",
            "где", "куда", "откуда", "когда", "какой", "который", "чей",
            "быть", "иметь", "делать", "сказать", "дать", "взять", "стать",
            "может", "должен", "следует", "необходимо", "важно", "нужно",
        ];
        
        for (i, term) in legal_terms.iter().enumerate() {
            vocab.insert(term.to_string(), i);
            vocab_reverse.insert(i, term.to_string());
        }
        
        // Add special tokens
        let special_tokens = ["<pad>", "<unk>", "<s>", "</s>"];
        for (i, token) in special_tokens.iter().enumerate() {
            vocab.insert(token.to_string(), legal_terms.len() + i);
            vocab_reverse.insert(legal_terms.len() + i, token.to_string());
        }
        
        Self {
            vocab,
            vocab_reverse,
            max_length: 512,
        }
    }
    
    pub fn tokenize(&self, text: &str) -> Vec<usize> {
        let words: Vec<&str> = text
            .to_lowercase()
            .split_whitespace()
            .collect();
            
        let mut tokens = Vec::new();
        for word in words {
            if let Some(&token_id) = self.vocab.get(word) {
                tokens.push(token_id);
            } else {
                // Use <unk> for unknown words
                if let Some(&unk_id) = self.vocab.get("<unk>") {
                    tokens.push(unk_id);
                }
            }
            
            if tokens.len() >= self.max_length {
                break;
            }
        }
        
        // Pad or truncate to max_length
        if let Some(&pad_id) = self.vocab.get("<pad>") {
            while tokens.len() < self.max_length {
                tokens.push(pad_id);
            }
        }
        
        tokens
    }
    
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }
}

// Simple cosine similarity fallback
pub fn cosine_similarity(vec1: &Array1<f32>, vec2: &Array1<f32>) -> f32 {
    let dot_product = vec1.dot(vec2);
    let norm1 = vec1.dot(vec1).sqrt();
    let norm2 = vec2.dot(vec2).sqrt();
    
    if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot_product / (norm1 * norm2)
    }
}

// Text embedding using simple TF-IDF-like approach
pub fn embed_text(text: &str, vocab: &HashMap<String, usize>) -> Array1<f32> {
    let mut embedding = Array1::zeros(vocab.len());
    let words: Vec<&str> = text.to_lowercase().split_whitespace().collect();
    
    // Simple word frequency
    for word in &words {
        if let Some(&idx) = vocab.get(word) {
            embedding[idx] += 1.0;
        }
    }
    
    // Normalize
    let norm = embedding.dot(&embedding).sqrt();
    if norm > 0.0 {
        embedding = embedding / norm;
    }
    
    embedding
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenizer() {
        let tokenizer = LegalTokenizer::new();
        let tokens = tokenizer.tokenize("закон и право");
        assert!(!tokens.is_empty());
    }
    
    #[test]
    fn test_cosine_similarity() {
        let vec1 = Array1::from(vec![1.0, 0.0, 0.0]);
        let vec2 = Array1::from(vec![1.0, 0.0, 0.0]);
        let sim = cosine_similarity(&vec1, &vec2);
        assert!((sim - 1.0).abs() < 1e-6);
    }
}
