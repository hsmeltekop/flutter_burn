use burn::backend::wgpu::AutoGraphicsApi;
use burn::backend::{Autodiff, Wgpu};

use burn::optim::AdamConfig;
use test_burn::model::ModelConfig;
use test_burn::training::{train, TrainingConfig};

fn main() {
    type MyBackend = Wgpu<AutoGraphicsApi, f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();
    let artifact_dir = "../guide";
    train::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );
}
