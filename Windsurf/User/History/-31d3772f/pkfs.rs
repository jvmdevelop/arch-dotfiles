use crate::neural_network::{VoxelNeuralNetwork, create_sample_training_data};
use anyhow::Result;
use std::fs;

pub fn generate_from_text(description: &str, output_path: &str) -> Result<()> {
    println!("Generating VDSL from description: \"{}\"", description);
    
    let mut network = VoxelNeuralNetwork::new();
    
    if let Ok(_) = network.load_model("model.json") {
        println!("Loaded existing model");
    } else {
        println!("No existing model found, training with sample data...");
        let sample_data = create_sample_training_data();
        network.train(&sample_data)?;
        network.save_model("model.json")?;
    }
    
    let features = network.predict(description)?;
    
    let vdsl_code = network.tokenizer.decode_to_vdsl(&features);
    
    fs::write(output_path, vdsl_code)?;
    println!("Generated VDSL saved to: {}", output_path);
    
    Ok(())
}

pub fn run_demo() -> Result<()> {
    println!("=== RVOXEL-AI DEMO ===\n");
    
    let sample_descriptions = vec![
        "simple cube with 10 unit sides",
        "small pyramid 5 units tall", 
        "large complex structure",
        "simple square plane",
        "detailed architectural model",
    ];
    
    let mut network = VoxelNeuralNetwork::new();
    
    println!("Training neural network with sample data...");
    let sample_data = create_sample_training_data();
    network.train(&sample_data)?;
    network.save_model("demo_model.json")?;
    
    println!("\n=== GENERATION RESULTS ===\n");
    
    for (i, description) in sample_descriptions.iter().enumerate() {
        println!("{}. Description: \"{}\"", i + 1, description);
        
        let features = network.predict(description)?;
        let vdsl_code = network.tokenizer.decode_to_vdsl(&features);
        
        println!("Generated VDSL:");
        println!("{}", vdsl_code);
        
        let filename = format!("demo_{}.vdsl", i + 1);
        fs::write(&filename, &vdsl_code)?;
        println!("Saved to: {}\n", filename);
        
        let lines: Vec<&str> = vdsl_code.lines().collect();
        let point_count = lines.iter().filter(|l| l.trim().chars().next().map_or(false, |c| c.is_ascii_digit())).count();
        let line_count = lines.iter().filter(|l| l.trim().starts_with('l')).count();
        
        println!("Statistics: {} points, {} lines, {} total lines\n", 
                point_count, line_count, lines.len());
    }
    
    println!("Generated files can be converted with the main character-voxel-gen project:");
    println!("cargo run -- demo_1.vdsl output1.obj");
    
    Ok(())
}

pub fn interactive_mode() -> Result<()> {
    println!("Enter descriptions to generate VDSL code (type 'quit' to exit)\n");
    
    let mut network = VoxelNeuralNetwork::new();
    
    if let Ok(_) = network.load_model("model.json") {
        println!("Loaded existing model");
    } else {
        println!("Training with sample data...");
        let sample_data = create_sample_training_data();
        network.train(&sample_data)?;
        network.save_model("model.json")?;
    }
    
    let mut counter = 1;
    
    loop {
        print!("Enter description> ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.to_lowercase() == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        println!("\nGenerating VDSL for: \"{}\"", input);
        
        let features = network.predict(input)?;
        let vdsl_code = network.tokenizer.decode_to_vdsl(&features);
        
        println!("Generated VDSL:");
        println!("{}", vdsl_code);
        
        let filename = format!("interactive_{}.vdsl", counter);
        fs::write(&filename, &vdsl_code)?;
        println!("Saved to: {}\n", filename);
        
        counter += 1;
    }
    
    println!("Goodbye!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sample_generation() {
        let result = generate_from_text("simple cube", "test_output.vdsl");
        assert!(result.is_ok());
        
        let content = fs::read_to_string("test_output.vdsl").unwrap();
        assert!(!content.is_empty());
        
        let _ = fs::remove_file("test_output.vdsl");
    }
    
    #[test]
    fn test_demo_mode() {
        let result = run_demo();
        assert!(result.is_ok());
        
        for i in 1..=5 {
            let filename = format!("demo_{}.vdsl", i);
            assert!(fs::metadata(&filename).is_ok());
            let _ = fs::remove_file(&filename);
        }
        
        let _ = fs::remove_file("demo_model.json");
    }
}
