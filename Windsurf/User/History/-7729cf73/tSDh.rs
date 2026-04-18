use ndarray::{Array1, Array2, ArrayD};
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    weights_input_hidden: Array2<f64>,
    biases_hidden: Array1<f64>,
    weights_hidden_output: Array2<f64>,
    biases_output: Array1<f64>,
    learning_rate: f64,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        let weights_input_hidden = Array2::random((hidden_size, input_size), Uniform::new(-0.5, 0.5));
        let biases_hidden = Array1::random(hidden_size, Uniform::new(-0.5, 0.5));
        let weights_hidden_output = Array2::random((output_size, hidden_size), Uniform::new(-0.5, 0.5));
        let biases_output = Array1::random(output_size, Uniform::new(-0.5, 0.5));
        
        Self {
            weights_input_hidden,
            biases_hidden,
            weights_hidden_output,
            biases_output,
            learning_rate: 0.01,
        }
    }
    
    pub fn forward(&self, input: &Array1<f64>) -> (Array1<f64>, Array1<f64>) {
        let hidden_input = self.weights_input_hidden.dot(input) + &self.biases_hidden;
        let hidden_output = self.relu(&hidden_input);
        
        let output_input = self.weights_hidden_output.dot(&hidden_output) + &self.biases_output;
        let output = self.softmax(&output_input);
        
        (hidden_output, output)
    }
    
    pub fn train(&mut self, inputs: &[Array1<f64>], targets: &[Array1<f64>], epochs: usize) {
        for epoch in 0..epochs {
            let mut total_error = 0.0;
            
            for (input, target) in inputs.iter().zip(targets.iter()) {
                let (hidden_output, output) = self.forward(input);
                
                let output_error = target - &output;
                total_error += output_error.iter().map(|&x| x * x).sum::<f64>();
                
                let output_delta = &output_error * self.softmax_derivative(&output);
                let hidden_error = self.weights_hidden_output.t().dot(&output_delta);
                let hidden_delta = &hidden_error * self.relu_derivative(&hidden_output);
                
                let input_outer = Array2::from_shape_fn((hidden_output.len(), input.len()), 
                    |(i, j)| hidden_output[i] * input[j]);
                self.weights_input_hidden += &(input_outer * output_delta.len() as f64 * self.learning_rate);
                self.biases_hidden += &(&hidden_delta * self.learning_rate);
                
                let hidden_outer = Array2::from_shape_fn((output.len(), hidden_output.len()),
                    |(i, j)| output[i] * hidden_output[j]);
                self.weights_hidden_output += &(hidden_outer * self.learning_rate);
                self.biases_output += &(&output_delta * self.learning_rate);
            }
            
            if epoch % 100 == 0 {
                println!("Epoch {}: Error = {:.6}", epoch, total_error / inputs.len() as f64);
            }
        }
    }
    
    pub fn predict(&self, input: &Array1<f64>) -> Array1<f64> {
        let (_, output) = self.forward(input);
        output
    }
    
    fn relu(&self, x: &Array1<f64>) -> Array1<f64> {
        x.mapv(|x| if x > 0.0 { x } else { 0.0 })
    }
    
    fn relu_derivative(&self, x: &Array1<f64>) -> Array1<f64> {
        x.mapv(|x| if x > 0.0 { 1.0 } else { 0.0 })
    }
    
    fn softmax(&self, x: &Array1<f64>) -> Array1<f64> {
        let exp_x = x.mapv(|x| x.exp());
        let sum_exp = exp_x.sum();
        exp_x / sum_exp
    }
    
    fn softmax_derivative(&self, x: &Array1<f64>) -> Array1<f64> {
        x * (1.0 - x)
    }
    
    pub fn set_learning_rate(&mut self, rate: f64) {
        self.learning_rate = rate;
    }
    
    pub fn save_weights(&self) -> HashMap<String, Vec<f64>> {
        let mut weights = HashMap::new();
        
        weights.insert("weights_input_hidden".to_string(), 
                      self.weights_input_hidden.iter().cloned().collect());
        weights.insert("biases_hidden".to_string(), 
                      self.biases_hidden.iter().cloned().collect());
        weights.insert("weights_hidden_output".to_string(), 
                      self.weights_hidden_output.iter().cloned().collect());
        weights.insert("biases_output".to_string(), 
                      self.biases_output.iter().cloned().collect());
        
        weights
    }
    
    pub fn load_weights(&mut self, weights: HashMap<String, Vec<f64>>) -> Result<(), String> {
        if let Some(w_ih) = weights.get("weights_input_hidden") {
            self.weights_input_hidden = Array1::from_vec(w_ih.clone()).into_shape((self.weights_input_hidden.nrows(), self.weights_input_hidden.ncols()))
                .map_err(|_| "Invalid shape for weights_input_hidden")?;
        }
        
        if let Some(b_h) = weights.get("biases_hidden") {
            self.biases_hidden = Array1::from_vec(b_h.clone());
        }
        
        if let Some(w_ho) = weights.get("weights_hidden_output") {
            self.weights_hidden_output = Array1::from_vec(w_ho.clone()).into_shape((self.weights_hidden_output.nrows(), self.weights_hidden_output.ncols()))
                .map_err(|_| "Invalid shape for weights_hidden_output")?;
        }
        
        if let Some(b_o) = weights.get("biases_output") {
            self.biases_output = Array1::from_vec(b_o.clone());
        }
        
        Ok(())
    }
}
