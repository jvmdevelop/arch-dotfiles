use burn::{
    Tensor,
    config::Config,
    module::Module,
    nn::{
        Dropout, DropoutConfig, Linear, LinearConfig,
        conv::{Conv2d, Conv2dConfig},
        pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
        ReLU,
    },
    prelude::Backend,
    tensor::TensorData,
};
use image::DynamicImage;

#[derive(Debug, Config)]
pub struct ModelConfig {
    pub voxel_size: usize,
    pub max_voxels: usize,
}

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    conv3: Conv2d<B>,
    conv4: Conv2d<B>,
    pool: AdaptiveAvgPool2d,
    dropout: Dropout,
    fc_voxel: Linear<B>,
    voxel_size: usize,
}

impl<B: Backend> Model<B> {
    pub fn new(config: ModelConfig, device: &B::Device) -> Self {
        Self {
            conv1: Conv2dConfig::new([3, 32], [3, 3]).init(device),
            conv2: Conv2dConfig::new([32, 64], [3, 3]).init(device),
            conv3: Conv2dConfig::new([64, 128], [3, 3]).init(device),
            conv4: Conv2dConfig::new([128, 256], [3, 3]).init(device),
            pool: AdaptiveAvgPool2dConfig::new([4, 4]).init(),
            dropout: DropoutConfig::new(0.3).init(),
            fc_voxel: LinearConfig::new(256 * 4 * 4, config.voxel_size * config.voxel_size * config.voxel_size).init(device),
            voxel_size: config.voxel_size,
        }
    }

    pub fn forward(&self, image: Tensor<B, 4>) -> Tensor<B, 3> {
        let mut x = self.conv1.forward(image.clone());
        x = ReLU::new().forward(x);
        x = self.conv2.forward(x);
        x = ReLU::new().forward(x);
        x = self.conv3.forward(x);
        x = ReLU::new().forward(x);
        x = self.conv4.forward(x);
        x = ReLU::new().forward(x);
        x = self.pool.forward(x);
        x = self.dropout.forward(x);

        let features = x.flatten(1, 3);
        let voxel_logits = self.fc_voxel.forward(features);
        let voxel_shape = [voxel_logits.dims()[0], self.voxel_size, self.voxel_size, self.voxel_size];
        let voxel_reshaped = voxel_logits.reshape(voxel_shape);
        
        voxel_reshaped.sigmoid()
    }
    
    pub fn voxel_size(&self) -> usize {
        self.voxel_size
    }
}
