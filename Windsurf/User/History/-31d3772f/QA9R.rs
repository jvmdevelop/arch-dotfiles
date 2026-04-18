use crate::neural_network::NeuralNetwork;
use crate::tokenizer::Tokenizer;
use crate::model::TrainingData;
use ndarray::Array1;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VdlGenerationError {
    TokenizationError(String),
    NetworkError(String),
    InvalidInput(String),
}

impl fmt::Display for VdlGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VdlGenerationError::TokenizationError(msg) => write!(f, "Tokenization error: {}", msg),
            VdlGenerationError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            VdlGenerationError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl Error for VdlGenerationError {}

pub struct VdlGenerator {
    tokenizer: Tokenizer,
    network: NeuralNetwork,
    max_sequence_length: usize,
}

impl VdlGenerator {
    pub fn new(tokenizer: Tokenizer, network: NeuralNetwork) -> Self {
        Self {
            tokenizer,
            network,
            max_sequence_length: 128,
        }
    }
    
    pub fn generate_vdsl(&self, input_text: &str) -> Result<String, VdlGenerationError> {
        if input_text.trim().is_empty() {
            return Err(VdlGenerationError::InvalidInput("Input text cannot be empty".to_string()));
        }
        
        let input_tokens = self.tokenizer.encode_text(input_text, self.max_sequence_length);
        let input_array = Array1::from_vec(input_tokens);
        
        let output_probs = self.network.predict(&input_array);
        
        let output_tokens = self.probs_to_tokens(&output_probs);
        
        let vdsl_output = self.tokenizer.decode_to_vdsl(&output_tokens);
        
        Ok(vdsl_output)
    }
    
    pub fn train(&mut self, training_data: &TrainingData, epochs: usize) -> Result<(), VdlGenerationError> {
        let mut inputs = Vec::new();
        let mut targets = Vec::new();
        
        for example in &training_data.examples {
            let input_encoded = self.tokenizer.encode_text(&example.input_text, self.max_sequence_length);
            let target_encoded = self.tokenizer.encode_text(&example.target_vdsl, self.max_sequence_length);
            
            inputs.push(Array1::from_vec(input_encoded));
            targets.push(Array1::from_vec(target_encoded));
        }
        
        if inputs.is_empty() {
            return Err(VdlGenerationError::InvalidInput("No training data provided".to_string()));
        }
        
        self.network.train(&inputs, &targets, epochs);
        Ok(())
    }
    
    pub fn train_from_examples(&mut self, examples: Vec<(String, String)>, epochs: usize) -> Result<(), VdlGenerationError> {
        let mut inputs = Vec::new();
        let mut targets = Vec::new();
        
        for (input_text, target_vdsl) in examples {
            let input_encoded = self.tokenizer.encode_text(&input_text, self.max_sequence_length);
            let target_encoded = self.tokenizer.encode_text(&target_vdsl, self.max_sequence_length);
            
            inputs.push(Array1::from_vec(input_encoded));
            targets.push(Array1::from_vec(target_encoded));
        }
        
        if inputs.is_empty() {
            return Err(VdlGenerationError::InvalidInput("No training examples provided".to_string()));
        }
        
        self.network.train(&inputs, &targets, epochs);
        Ok(())
    }
    
    fn probs_to_tokens(&self, probs: &Array1<f64>) -> Vec<usize> {
        probs.iter()
            .enumerate()
            .map(|(i, &prob)| (i, prob))
            .filter(|(_, prob)| *prob > 0.1) 
            .map(|(i, _)| i)
            .take(20) 
            .collect()
    }
    
    pub fn generate_with_temperature(&self, input_text: &str, temperature: f64) -> Result<String, VdlGenerationError> {
        if input_text.trim().is_empty() {
            return Err(VdlGenerationError::InvalidInput("Input text cannot be empty".to_string()));
        }
        
        let input_tokens = self.tokenizer.encode_text(input_text, self.max_sequence_length);
        let input_array = Array1::from_vec(input_tokens);
        
        let output_probs = self.network.predict(&input_array);
        let temp_probs = self.apply_temperature(&output_probs, temperature);
        let output_tokens = self.sample_from_distribution(&temp_probs);
        
        let vdsl_output = self.tokenizer.decode_to_vdsl(&output_tokens);
        
        Ok(vdsl_output)
    }
    
    fn apply_temperature(&self, probs: &Array1<f64>, temperature: f64) -> Array1<f64> {
        if temperature <= 0.0 {
            return probs.clone();
        }
        
        let scaled_probs = probs.mapv(|p| p / temperature);
        let exp_probs = scaled_probs.mapv(|p| p.exp());
        let sum_exp = exp_probs.sum();
        
        if sum_exp > 0.0 {
            exp_probs / sum_exp
        } else {
            probs.clone()
        }
    }
    
    fn sample_from_distribution(&self, probs: &Array1<f64>) -> Vec<usize> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut tokens = Vec::new();
        
        for _ in 0..20 { 
            let random_val: f64 = rng.gen();
            let mut cumulative = 0.0;
            
            for (i, &prob) in probs.iter().enumerate() {
                cumulative += prob;
                if random_val <= cumulative {
                    tokens.push(i);
                    break;
                }
            }
        }
        
        tokens
    }
    
    pub fn save_model(&self) -> Result<String, Box<dyn Error>> {
        let weights = self.network.save_weights();
        Ok(serde_json::to_string_pretty(&weights)?)
    }
    
    pub fn load_model(&mut self, model_json: &str) -> Result<(), Box<dyn Error>> {
        let weights: std::collections::HashMap<String, Vec<f64>> = serde_json::from_str(model_json)?;
        self.network.load_weights(weights)?;
        Ok(())
    }
    
    pub fn set_max_sequence_length(&mut self, length: usize) {
        self.max_sequence_length = length;
    }
    
    pub fn get_tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }
    
    pub fn get_network(&self) -> &NeuralNetwork {
        &self.network
    }
}
