mod neural_network;
mod tokenizer;
mod vdl_generator;
mod model;

use neural_network::NeuralNetwork;
use tokenizer::Tokenizer;
use vdl_generator::VdlGenerator;
use model::TrainingData;

fn main() {
    println!("RVoxel AI - VDSL Generation from Text");
    
    // Initialize components
    let tokenizer = Tokenizer::new();
    let mut network = NeuralNetwork::new(128, 256, 64); // input, hidden, output
    let generator = VdlGenerator::new(tokenizer, network);
    
    // Example usage
    let input_text = "create a simple cube with size 10";
    
    match generator.generate_vdsl(input_text) {
        Ok(vdsl_output) => {
            println!("Generated VDSL:");
            println!("{}", vdsl_output);
        }
        Err(e) => {
            eprintln!("Error generating VDSL: {}", e);
        }
    }
}
