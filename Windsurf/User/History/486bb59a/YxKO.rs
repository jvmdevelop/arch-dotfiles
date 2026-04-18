use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingExample {
    pub description: String,
    pub vdsl_code: String,
    pub features: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoxelFeatures {
    pub num_points: usize,
    pub num_lines: usize,
    pub bounding_box: (f64, f64, f64, f64, f64, f64), // min_x, max_x, min_y, max_y, min_z, max_z
    pub complexity: f64,
    pub symmetry: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkModel {
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<f64>,
    pub architecture: Vec<usize>,
    pub vocabulary: HashMap<String, usize>,
    pub feature_mapping: HashMap<String, f64>,
}

impl NeuralNetworkModel {
    pub fn new(architecture: Vec<usize>) -> Self {
        Self {
            weights: Vec::new(),
            biases: Vec::new(),
            architecture,
            vocabulary: HashMap::new(),
            feature_mapping: HashMap::new(),
        }
    }
    
    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let model: NeuralNetworkModel = serde_json::from_str(&json)?;
        Ok(model)
    }
}

#[derive(Debug, Clone)]
pub struct TextFeatures {
    pub word_count: usize,
    pub unique_words: usize,
    pub avg_word_length: f64,
    pub contains_numbers: bool,
    pub contains_dimensions: bool,
    pub contains_shapes: Vec<String>,
    pub complexity_score: f64,
}

impl TextFeatures {
    pub fn extract(text: &str, vocabulary: &HashMap<String, usize>) -> Vec<f64> {
        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .collect();
        
        let word_count = words.len();
        let unique_words: std::collections::HashSet<_> = words.iter().collect();
        let unique_count = unique_words.len();
        
        let avg_word_length = if word_count > 0 {
            words.iter().map(|w| w.len()).sum::<usize>() as f64 / word_count as f64
        } else {
            0.0
        };
        
        let contains_numbers = text.chars().any(|c| c.is_ascii_digit());
        let contains_dimensions = text_lower.contains("dimension") || 
                                 text_lower.contains("size") ||
                                 text_lower.contains("scale");
        
        let shapes = vec!["cube", "sphere", "pyramid", "cylinder", "plane"];
        let contains_shapes: Vec<String> = shapes.iter()
            .filter(|&&shape| text_lower.contains(shape))
            .map(|&s| s.to_string())
            .collect();
        
        let complexity_score = (unique_count as f64 / word_count.max(1) as f64) * 
                              (1.0 + contains_shapes.len() as f64 * 0.5) +
                              if contains_numbers { 0.3 } else { 0.0 } +
                              if contains_dimensions { 0.2 } else { 0.0 };
        
        // Create feature vector
        let mut features = vec![
            word_count as f64,
            unique_count as f64,
            avg_word_length,
            if contains_numbers { 1.0 } else { 0.0 },
            if contains_dimensions { 1.0 } else { 0.0 },
            contains_shapes.len() as f64,
            complexity_score,
        ];
        
        // Add vocabulary-based features
        for (word, &index) in vocabulary {
            if index < 50 { // Limit vocabulary size for features
                let count = words.iter().filter(|&&w| w == word).count() as f64;
                features.push(count);
            }
        }
        
        // Pad or truncate to fixed size
        while features.len() < 100 {
            features.push(0.0);
        }
        features.truncate(100);
        
        features
    }
}
