mod batching;
mod data;
mod model;
mod training;

use burn::{
    backend::{NdArray, ndarray::NdArrayDevice},
};

use training::train::{TrainingConfig, train};
use data::data_generator::write_and_generate_data;

fn main() {
    println!("Generating data...");
    write_and_generate_data(1000);
    
    println!("Initializing training...");
    let device = NdArrayDevice::Cpu;
    
    let config = TrainingConfig::new(model::model::RideModelConfig::new());
    
    train(config, device);
}
