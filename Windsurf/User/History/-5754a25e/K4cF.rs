mod data_processor;
mod model;
mod trainer;
mod voxel_generator;
mod inference;
mod lightweight_trainer;

use burn::{
    prelude::Backend,
    tensor::Tensor,
};
use model::{Model, ModelConfig};
use inference::InferenceEngine;
use data_processor::ImageProcessor;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("rvoxel-ai: Lightweight Image-to-Voxel Generator");
    
    // Example usage
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: {} <input_image> <output_vdsl> [threshold]", args[0]);
        println!("Example: {} input.png output.vdsl 0.5", args[0]);
        return Ok(());
    }
    
    let input_path = &args[1];
    let output_path = &args[2];
    let threshold: f32 = args.get(3).unwrap_or(&"0.5".to_string()).parse()?;
    
    if !Path::new(input_path).exists() {
        eprintln!("Error: Input image '{}' does not exist", input_path);
        return Ok(());
    }
    
    // Initialize model (using CPU backend for lightweight operation)
    type MyBackend = burn::backend::Wgpu<burn::backend::Autodiff<burn::backend::Cpu>>;
    let device = burn::backend::WgpuDevice::default();
    
    let model_config = ModelConfig {
        voxel_size: 32,
        max_voxels: 1000,
    };
    
    let model = Model::<MyBackend>::new(model_config, &device);
    let inference_engine = InferenceEngine::new(model, device, threshold);
    
    println!("Processing image: {}", input_path);
    println!("Threshold: {}", threshold);
    
    match inference_engine.generate_and_save_vdsl(input_path, output_path, "generated_object") {
        Ok(_) => {
            println!("Successfully generated voxel file: {}", output_path);
            println!("You can now use rvoxel-dsl to convert this to 3D formats.");
        }
        Err(e) => {
            eprintln!("Error generating voxel file: {}", e);
        }
    }
    
    Ok(())
}
