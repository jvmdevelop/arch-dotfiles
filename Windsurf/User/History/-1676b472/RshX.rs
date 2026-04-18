use burn::{
    prelude::Backend,
    tensor::{Tensor, backend::BackendOps},
    train::{TrainOutput, TrainStep, ValidStep},
    optim::AdamConfig,
};
use crate::model::{Model, ModelConfig};

#[derive(Debug)]
pub struct VoxelLossOutput<B: Backend> {
    pub loss: Tensor<B, 1>,
    pub voxel_pred: Tensor<B, 3>,
}

#[derive(Debug)]
pub struct VoxelLoss;

impl<B: Backend> VoxelLoss {
    pub fn forward(
        &self,
        voxel_pred: Tensor<B, 3>,
        voxel_target: Tensor<B, 3>,
    ) -> VoxelLossOutput<B> {
        let bce_loss = binary_cross_entropy(voxel_pred.clone(), voxel_target.clone());
        
        VoxelLossOutput {
            loss: bce_loss,
            voxel_pred,
        }
    }
}

pub fn binary_cross_entropy<B: Backend>(
    pred: Tensor<B, 3>,
    target: Tensor<B, 3>,
) -> Tensor<B, 1> {
    let eps = 1e-8;
    let pred_clamped = pred.clamp(eps, 1.0 - eps);
    
    let term1 = target.clone() * pred_clamped.clone().log();
    let term2 = (target.clone() * -1.0 + 1.0) * (pred_clamped.clone() * -1.0 + 1.0).log();
    
    let bce = -(term1 + term2).sum() / pred.dims().iter().product::<usize>() as f32;
    bce
}

impl<B: Backend> TrainStep<Tensor<B, 4>, Tensor<B, 3>, VoxelLossOutput<B>> for Model<B> {
    fn step(&self, voxel_pred: Tensor<B, 4>, voxel_target: Tensor<B, 3>, loss_fn: VoxelLoss) -> TrainOutput<VoxelLossOutput<B>> {
        let voxel_pred = self.forward(voxel_pred);
        let VoxelLossOutput { loss, .. } = loss_fn.forward(voxel_pred.clone(), voxel_target);
        
        TrainOutput::new(self, loss, VoxelLossOutput {
            loss,
            voxel_pred,
        })
    }
}

impl<B: Backend> ValidStep<Tensor<B, 4>, Tensor<B, 3>, VoxelLossOutput<B>> for Model<B> {
    fn step(&self, voxel_pred: Tensor<B, 4>, voxel_target: Tensor<B, 3>, loss_fn: VoxelLoss) -> VoxelLossOutput<B> {
        let voxel_pred = self.forward(voxel_pred);
        loss_fn.forward(voxel_pred, voxel_target)
    }
}

pub struct TrainingConfig {
    pub learning_rate: f64,
    pub epochs: usize,
    pub batch_size: usize,
    pub voxel_size: usize,
    pub threshold: f32,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            epochs: 50,
            batch_size: 4,
            voxel_size: 32,
            threshold: 0.5,
        }
    }
}
