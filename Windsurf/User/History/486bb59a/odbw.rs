use serde::{Deserialize, Serialize};
use ndarray::Array1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingExample {
    pub input_text: String,
    pub target_vdsl: String,
    pub difficulty: f64, // 0.0..1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingData {
    pub examples: Vec<TrainingExample>,
}

impl TrainingData {
    pub fn new() -> Self {
        Self {
            examples: Vec::new(),
        }
    }
    
    pub fn add_example(&mut self, input: &str, target: &str, difficulty: f64) {
        self.examples.push(TrainingExample {
            input_text: input.to_string(),
            target_vdsl: target.to_string(),
            difficulty: difficulty.clamp(0.0, 1.0),
        });
    }
    
    pub fn load_from_json(&mut self, json_str: &str) -> Result<(), serde_json::Error> {
        let data: TrainingData = serde_json::from_str(json_str)?;
        self.examples = data.examples;
        Ok(())
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    pub fn get_training_pairs(&self) -> Vec<(String, String)> {
        self.examples.iter()
            .map(|ex| (ex.input_text.clone(), ex.target_vdsl.clone()))
            .collect()
    }
    
    pub fn filter_by_difficulty(&self, min_difficulty: f64, max_difficulty: f64) -> TrainingData {
        let filtered_examples: Vec<TrainingExample> = self.examples.iter()
            .filter(|ex| ex.difficulty >= min_difficulty && ex.difficulty <= max_difficulty)
            .cloned()
            .collect();
        
        TrainingData {
            examples: filtered_examples,
        }
    }
    
    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.examples.shuffle(&mut rng);
    }
    
    pub fn split(&self, train_ratio: f64) -> (TrainingData, TrainingData) {
        let split_point = (self.examples.len() as f64 * train_ratio) as usize;
        
        let train_examples = self.examples[..split_point].to_vec();
        let test_examples = self.examples[split_point..].to_vec();
        
        (
            TrainingData { examples: train_examples },
            TrainingData { examples: test_examples },
        )
    }
}

impl Default for TrainingData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub input_size: usize,
    pub hidden_size: usize,
    pub output_size: usize,
    pub learning_rate: f64,
    pub epochs: usize,
    pub batch_size: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            input_size: 128,
            hidden_size: 256,
            output_size: 64,
            learning_rate: 0.01,
            epochs: 1000,
            batch_size: 32,
        }
    }
}

pub fn create_sample_training_data() -> TrainingData {
    let mut data = TrainingData::new();
    
    data.add_example(
        "create a cube",
        "0: 0 0 0\n1: 10 0 0\n2: 10 10 0\n3: 0 10 0\n4: 0 0 10\n5: 10 0 10\n6: 10 10 10\n7: 0 10 10\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\nl: 4 5\nl: 5 6\nl: 6 7\nl: 7 4\nl: 0 4\nl: 1 5\nl: 2 6\nl: 3 7",
        0.1
    );
    
    data.add_example(
        "make a square",
        "0: 0 0 0\n1: 10 0 0\n2: 10 10 0\n3: 0 10 0\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0",
        0.1
    );
    
    data.add_example(
        "draw a line",
        "0: 0 0 0\n1: 10 0 0\nl: 0 1",
        0.05
    );
    
    data.add_example(
        "create a large cube with size 20",
        "0: 0 0 0\n1: 20 0 0\n2: 20 20 0\n3: 0 20 0\n4: 0 0 20\n5: 20 0 20\n6: 20 20 20\n7: 0 20 20\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\nl: 4 5\nl: 5 6\nl: 6 7\nl: 7 4\nl: 0 4\nl: 1 5\nl: 2 6\nl: 3 7",
        0.2
    );
    
    data.add_example(
        "build a small box",
        "0: 0 0 0\n1: 5 0 0\n2: 5 5 0\n3: 0 5 0\n4: 0 0 5\n5: 5 0 5\n6: 5 5 5\n7: 0 5 5\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\nl: 4 5\nl: 5 6\nl: 6 7\nl: 7 4\nl: 0 4\nl: 1 5\nl: 2 6\nl: 3 7",
        0.15
    );
    
    data
}
