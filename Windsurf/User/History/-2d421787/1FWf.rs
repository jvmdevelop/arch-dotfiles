use burn::{
    config::Config,
    data::dataset::Dataset,
    tensor::{backend::Backend, Tensor},
};

use crate::{
    data::data_loader::RideLoader,
    model::model::{RideModel, RideModelConfig},
};

#[derive(Config)]
pub struct TrainingConfig {
    pub model: RideModelConfig,
    #[config(default = 10)]
    pub num_epochs: usize,
    #[config(default = 32)]
    pub batch_size: usize,
    #[config(default = 1)]
    pub num_workers: usize,
    #[config(default = 0.1)]
    pub learning_rate: f64,
}

pub fn train<B: Backend<Device = burn::backend::NdArray, FloatElem = f32>>(config: TrainingConfig, device: B::Device) {
    let model = config.model.init::<B>(&device);

    let train_loader = RideLoader::from_csv("data/data.csv");
    
    println!("Starting training for {} epochs", config.num_epochs);

    for epoch in 1..=config.num_epochs {
        let model_train = model.clone();
        let mut loss_sum = 0.0;
        let mut batch_count = 0;

        for batch_idx in 0..(train_loader.len() / config.batch_size) {
            let batch = create_batch(&train_loader, batch_idx, config.batch_size, &device);
            
            let output = model_train.forward(batch.features.clone());
            let loss = binary_cross_entropy(output.clone(), batch.targets.clone());
            
            loss_sum += loss.into_scalar();
            batch_count += 1;

            if batch_idx % 10 == 0 {
                println!("Epoch: {}, Batch: {}, Loss: {:.4}", epoch, batch_idx, loss.into_scalar());
            }
        }

        let avg_loss = loss_sum / batch_count as f32;
        println!("Epoch: {} completed, Average Loss: {:.4}", epoch, avg_loss);
    }

    println!("Training completed");
}

fn create_batch<B: Backend<Device = burn::backend::NdArray, FloatElem = f32>>(
    loader: &RideLoader,
    batch_idx: usize,
    batch_size: usize,
    device: &B::Device,
) -> RideBatch<B> {
    let start_idx = batch_idx * batch_size;
    let mut features = Vec::new();
    let mut targets = Vec::new();

    for i in start_idx..(start_idx + batch_size).min(loader.len()) {
        if let Some(item) = loader.get(i) {
            features.push(item.features);
            targets.push(item.target);
        }
    }

    RideBatch {
        features: Tensor::stack(features, 0),
        targets: Tensor::stack(targets, 0),
    }
}

#[derive(Debug, Clone)]
pub struct RideBatch<B: Backend> {
    pub features: Tensor<B, 2>,
    pub targets: Tensor<B, 2>,
}

fn binary_cross_entropy<B: Backend>(output: Tensor<B, 2>, targets: Tensor<B, 2>) -> Tensor<B, 2> {
    let epsilon = 1e-7f32;
    let output_dims = output.dims();
    let output_device = output.device();
    
    let output_clamped = output.clamp(epsilon, 1.0 - epsilon);
    let ones = Tensor::ones(output_dims, &output_device);
    
    let term1 = targets.clone() * output_clamped.clone().log();
    let term2 = (ones.clone() - targets) * (ones - output_clamped).log();
    
    let loss = -(term1 + term2);
    loss.mean_dim(1)
}
