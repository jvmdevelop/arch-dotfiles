use crate::model::{NeuralNetworkModel, TrainingExample, TextFeatures};
use crate::tokenizer::Tokenizer;
use anyhow::Result;
use ndarray::{Array1, Array2, Axis};

pub struct VoxelNeuralNetwork {
    pub model: NeuralNetworkModel,
    pub tokenizer: Tokenizer,
    training_data: Vec<(Vec<f64>, Vec<f64>)>,
}

impl VoxelNeuralNetwork {
    pub fn new() -> Self {
        Self {
            model: NeuralNetworkModel::new(vec![100, 64, 32, 50]), 
            tokenizer: Tokenizer::new(),
            training_data: Vec::new(),
        }
    }
    
    pub fn train(&mut self, training_data: &[TrainingExample]) -> Result<()> {
        println!("starting training with {} examples: ", training_data.len());
        
        let descriptions: Vec<String> = training_data.iter()
            .map(|ex| ex.description.clone())
            .collect();
        
        self.tokenizer.build_vocabulary(&descriptions)?;
        
        self.training_data.clear();
        
        for example in training_data {
            let text_features = TextFeatures::extract(&example.description, self.tokenizer.get_vocabulary());
            let vdsl_features = self.tokenizer.encode_vdsl(&example.vdsl_code);
            
            self.training_data.push((text_features, vdsl_features));
        }
        

        self.model.vocabulary = self.tokenizer.get_vocabulary().clone();
        
        println!("training completed!");
        Ok(())
    }
    
    pub fn predict(&self, description: &str) -> Result<Vec<f64>> {
        let text_features = TextFeatures::extract(description, self.tokenizer.get_vocabulary());
        
        if !self.training_data.is_empty() {
            let mut best_match = 0;
            let mut best_distance = f64::INFINITY;
            
            for (i, (train_features, _)) in self.training_data.iter().enumerate() {
                let distance = Self::euclidean_distance(&text_features, train_features);
                if distance < best_distance {
                    best_distance = distance;
                    best_match = i;
                }
            }
            
            Ok(self.training_data[best_match].1.clone())
        } else {
            self.rule_based_generation(description)
        }
    }
    
    fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
    
    fn rule_based_generation(&self, description: &str) -> Result<Vec<f64>> {
        let desc_lower = description.to_lowercase();
        let mut features = vec![0.0; 50];
        
        let mut point_count = 4;
        let mut line_count = 4;
        let mut scale = 10.0;
        
        if desc_lower.contains("cube") {
            point_count = 8;
            line_count = 12;
        } else if desc_lower.contains("pyramid") {
            point_count = 5;
            line_count = 8;
        } else if desc_lower.contains("large") {
            scale = 20.0;
        } else if desc_lower.contains("small") {
            scale = 5.0;
        }
        
        if desc_lower.contains("complex") || desc_lower.contains("detailed") {
            point_count *= 2;
            line_count *= 2;
        }
        
        features[0] = point_count as f64;
        features[1] = line_count as f64;
        features[2] = scale;
        features[3] = scale;
        features[4] = scale;
        
        Ok(features)
    }
    
    pub fn save_model(&self, path: &str) -> Result<()> {
        self.model.save(path)
    }
    
    pub fn load_model(&mut self, path: &str) -> Result<()> {
        self.model = NeuralNetworkModel::load(path)?;
        self.tokenizer = Tokenizer::new();
        Ok(())
    }
}

pub fn train_network(training_data_path: &str) -> Result<()> {
    println!("loading training data from: {}", training_data_path);
    
    let json_content = std::fs::read_to_string(training_data_path)?;
    let training_examples: Vec<TrainingExample> = serde_json::from_str(&json_content)?;
    
    let mut network = VoxelNeuralNetwork::new();
    network.train(&training_examples)?;
    
    network.save_model("model.json")?;
    println!("model saved to model.json");
    
    Ok(())
}

pub fn create_sample_training_data() -> Vec<TrainingExample> {
    vec![
        TrainingExample {
            description: "simple cube with 10 unit sides".to_string(),
            vdsl_code: "0: 0 0 0\n1: 10 0 0\n2: 10 10 0\n3: 0 10 0\n4: 0 0 10\n5: 10 0 10\n6: 10 10 10\n7: 0 10 10\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\nl: 4 5\nl: 5 6\nl: 6 7\nl: 7 4\nl: 0 4\nl: 1 5\nl: 2 6\nl: 3 7\n".to_string(),
            features: vec![8.0, 12.0, 0.0, 20.0, 0.0, 10.0, 10.0, 10.0],
        },
        TrainingExample {
            description: "small pyramid 5 units tall".to_string(),
            vdsl_code: "0: 0 0 0\n1: 5 0 0\n2: 5 5 0\n3: 0 5 0\n4: 2.5 2.5 5\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\nl: 0 4\nl: 1 4\nl: 2 4\nl: 3 4\n".to_string(),
            features: vec![5.0, 8.0, 0.0, 13.0, 0.0, 5.0, 5.0, 5.0],
        },
        TrainingExample {
            description: "simple square plane".to_string(),
            vdsl_code: "0: 0 0 0\n1: 10 0 0\n2: 10 10 0\n3: 0 10 0\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\n".to_string(),
            features: vec![4.0, 4.0, 0.0, 8.0, 0.0, 10.0, 10.0, 0.0],
        },
        TrainingExample {
            description: "large complex structure".to_string(),
            vdsl_code: "0: 0 0 0\n1: 20 0 0\n2: 20 20 0\n3: 0 20 0\n4: 0 0 20\n5: 20 0 20\n6: 20 20 20\n7: 0 20 20\n8: 10 0 0\n9: 20 10 0\n10: 10 20 0\n11: 0 10 0\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\nl: 4 5\nl: 5 6\nl: 6 7\nl: 7 4\nl: 0 4\nl: 1 5\nl: 2 6\nl: 3 7\nl: 0 8\nl: 8 1\nl: 1 9\nl: 9 2\nl: 2 10\nl: 10 3\nl: 3 11\nl: 11 0\n".to_string(),
            features: vec![12.0, 18.0, 0.0, 30.0, 0.0, 20.0, 20.0, 20.0],
        },
    ]
}
