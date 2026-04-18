use burn::{
    prelude::Backend,
    tensor::{Tensor, TensorData},
};
use image::{DynamicImage, ImageFormat};

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn preprocess_image<B: Backend>(
        image: &DynamicImage,
        device: &B::Device,
        target_size: (u32, u32),
    ) -> Tensor<B, 4> {
        let resized = image.resize_exact(target_size.0, target_size.1, image::imageops::FilterType::Lanczos3);
        let rgb_image = resized.to_rgb8();
        
        let (width, height) = rgb_image.dimensions();
        let mut data = Vec::with_capacity((width * height * 3) as usize);
        
        for pixel in rgb_image.pixels() {
            let [r, g, b] = pixel.0;
            data.push(r as f32 / 255.0);
            data.push(g as f32 / 255.0);
            data.push(b as f32 / 255.0);
        }
        
        let tensor_data = TensorData::new(data, [1, 3, height as usize, width as usize]);
        Tensor::<B, 4>::from_data(tensor_data.convert(), device)
    }
    
    pub fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        Ok(img)
    }
}
