use burn::{
    data::dataset::Dataset,
    prelude::Backend,
    tensor::Tensor,
};

use super::data_generator::Data;

pub struct RideLoader {
    data: Vec<Data>,
}

impl RideLoader {
    pub fn from_csv(path: &str) -> Self {
        let data = read_data(path);
        Self { data }
    }
}

pub struct RideItem<B: Backend> {
    pub features: Tensor<B, 3>,
    pub target: Tensor<B, 1>,
}

impl<B: Backend> Dataset<RideItem<B>> for RideLoader {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<RideItem<B>> {
        let data = self.data.get(index)?;

        Some(RideItem {
            features: Tensor::from_floats([[[data.distance, data.price, data.user_rating]]], &B::Device::default()),
            target: Tensor::from_floats([data.accepted as f32], &B::Device::default()),
        })
    }
}

pub fn read_data(path: &str) -> Vec<Data> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    reader
        .deserialize()
        .collect::<Result<Vec<Data>, _>>()
        .unwrap()
}
