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
    
    let tokenizer = Tokenizer::new();
    let mut network = NeuralNetwork::new(128, 256, 64);
    let mut generator = VdlGenerator::new(tokenizer, network);
    
    // Create training data
    let mut training_data = TrainingData::new();
    
    // Add examples
    training_data.add_example(
        "create a cube",
        "0: 0 0 0\n1: 10 0 0\n2: 10 10 0\n3: 0 10 0\n4: 0 0 10\n5: 10 0 10\n6: 10 10 10\n7: 0 10 10\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0\nl: 4 5\nl: 5 6\nl: 6 7\nl: 7 4\nl: 0 4\nl: 1 5\nl: 2 6\nl: 3 7",
        0.1
    );
    
    training_data.add_example(
        "make a square",
        "0: 0 0 0\n1: 10 0 0\n2: 10 10 0\n3: 0 10 0\nl: 0 1\nl: 1 2\nl: 2 3\nl: 3 0",
        0.1
    );
    
    training_data.add_example(
        "draw a line",
        "0: 0 0 0\n1: 10 0 0\nl: 0 1",
        0.05
    );
    
    // Train the model
    println!("Training model...");
    if let Err(e) = generator.train(&training_data, 100) {
        eprintln!("Training error: {}", e);
        return;
    }
    println!("Training completed!");
    
    // Test generation
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
