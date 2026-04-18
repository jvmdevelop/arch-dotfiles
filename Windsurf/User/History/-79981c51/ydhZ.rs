use crate::errors::{AppError, Result};
use burn::{
    config::Config,
    module::Module,
    nn::{
        attention::MhaInput,
        cache::AutoregressiveCache,
        loss::CrossEntropyLossConfig,
        transformer::{TransformerConfig, TransformerModel, TransformerModelConfig},
        Embedding, EmbeddingConfig, Linear, LinearConfig, ReLU,
    },
    tensor::{
        backend::Backend,
        Int, Tensor,
    },
    train::{TrainOutput, TrainStep, ValidStep},
};
use ndarray::{Array1, Array2};
use std::collections::HashMap;

// Text tokenizer for legal documens
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

// Legal text similarity model using Burn
#[derive(Module, Debug)]
pub struct LegalSimilarityModel<B: Backend> {
    embedding: Embedding<B>,
    transformer: TransformerModel<B, ReLU>,
    projection: Linear<B>,
    similarity_head: Linear<B>,
}

#[derive(Config, Debug)]
pub struct LegalSimilarityModelConfig {
    pub vocab_size: usize,
    pub d_model: usize,
    pub nhead: usize,
    pub num_encoder_layers: usize,
    pub num_decoder_layers: usize,
    pub dim_feedforward: usize,
    pub dropout: f64,
    pub max_seq_length: usize,
}

impl LegalSimilarityModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> LegalSimilarityModel<B> {
        let embedding_config = EmbeddingConfig::new(self.vocab_size, self.d_model);
        let embedding = embedding_config.init(device);
        
        let transformer_config = TransformerConfig::new(
            self.d_model,
            self.nhead,
            self.num_encoder_layers,
            self.num_decoder_layers,
            self.dim_feedforward,
            self.dropout,
        );
        let transformer = transformer_config.init(device);
        
        let projection_config = LinearConfig::new(self.d_model, self.d_model);
        let projection = projection_config.init(device);
        
        let similarity_config = LinearConfig::new(self.d_model, 1);
        let similarity_head = similarity_config.init(device);
        
        LegalSimilarityModel {
            embedding,
            transformer,
            projection,
            similarity_head,
        }
    }
}

impl<B: Backend> LegalSimilarityModel<B> {
    pub fn forward(&self, tokens1: Tensor<B, 2, Int>, tokens2: Tensor<B, 2, Int>) -> Tensor<B, 1> {
        // Embed tokens
        let emb1 = self.embedding.forward(tokens1);
        let emb2 = self.embedding.forward(tokens2);
        
        // Apply transformer
        let encoded1 = self.transformer.forward(emb1, None);
        let encoded2 = self.transformer.forward(emb2, None);
        
        // Global average pooling
        let pooled1 = encoded1.squeeze::<1>(0).mean_dim(0);
        let pooled2 = encoded2.squeeze::<1>(0).mean_dim(0);
        
        // Project to similarity space
        let proj1 = self.projection.forward(pooled1);
        let proj2 = self.projection.forward(pooled2);
        
        // Compute similarity
        let combined = proj1.clone() * proj2; // Element-wise multiplication
        let similarity = self.similarity_head.forward(combined);
        
        similarity.squeeze::<1>(0)
    }
    
    pub fn forward_similarity(&self, text_pairs: Vec<(&str, &str)>, tokenizer: &LegalTokenizer, device: &B::Device) -> Vec<f32> {
        let mut similarities = Vec::new();
        
        for (text1, text2) in text_pairs {
            let tokens1 = tokenizer.tokenize(text1);
            let tokens2 = tokenizer.tokenize(text2);
            
            // Convert to tensors
            let tensor1 = Tensor::<B, 2, Int>::from_data(
                burn::tensor::Data::new(tokens1, burn::tensor::Shape::new([1, tokens1.len()])),
                device,
            );
            let tensor2 = Tensor::<B, 2, Int>::from_data(
                burn::tensor::Data::new(tokens2, burn::tensor::Shape::new([1, tokens2.len()])),
                device,
            );
            
            // Forward pass
            let similarity = self.forward(tensor1, tensor2);
            let similarity_val = similarity.into_scalar();
            
            similarities.push(similarity_val);
        }
        
        similarities
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
