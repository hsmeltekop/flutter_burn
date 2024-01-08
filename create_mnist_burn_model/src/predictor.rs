use std::sync::OnceLock;

use burn::{
    backend::{
        wgpu::{AutoGraphicsApi, WgpuDevice},
        Wgpu,
    },
    config::Config,
    data::{dataloader::batcher::Batcher, dataset::source::huggingface::MNISTItem},
    module::Module,
    record::{CompactRecorder, Recorder},
};
use stopwatch::Stopwatch;

use crate::{
    data::MNISTBatcher,
    model::{Model, ModelRecord},
    training::TrainingConfig,
};

pub struct Predictor {
    pub device: WgpuDevice,
    pub model: Model<Wgpu<AutoGraphicsApi, f32, i32>>,
    pub batcher: MNISTBatcher<Wgpu<AutoGraphicsApi, f32, i32>>,
}

impl Predictor {
    pub fn create_item(&self, bytes: &[u8], label: usize) -> MNISTItem {
        let mut image_array = [[0f32; 28]; 28];
        for (i, pixel) in bytes.iter().enumerate() {
            let x = i % 28;
            let y = i / 28;
            image_array[y][x] = *pixel as f32;
        }

        let item = MNISTItem {
            image: image_array,
            label,
        };
        item
    }

    pub fn predict(&self, bytes: &[u8], label: usize) -> i32 {
        let sw = Stopwatch::start_new();
        let item = self.create_item(bytes, label);
        let batch = self.batcher.batch(vec![item]);
        let output = self.model.forward(batch.images);
        let predicted = &output.argmax(1).flatten::<1>(0, 1).into_scalar();

        println!("prediction took {} ms....", sw.elapsed_ms());
        *predicted
    }
}

pub fn predictor() -> &'static Predictor {
    static PREDICTOR: OnceLock<Predictor> = OnceLock::new();
    PREDICTOR.get_or_init(|| {
        let device = burn::backend::wgpu::WgpuDevice::default();
        let mut sw = Stopwatch::start_new();
        println!("Initializing Predictor....");
        let artifact_dir = "./guide";

        let config_bytes = include_bytes!("../guide/config.json");
        let config = TrainingConfig::load_binary(config_bytes).unwrap();

        let record: ModelRecord<Wgpu<AutoGraphicsApi, f32, i32>> = CompactRecorder::new()
            .load(format!("{artifact_dir}/model").into())
            .expect("Failed to load trained model");

        let model = config
            .model
            .init_with::<Wgpu<AutoGraphicsApi, f32, i32>>(record)
            .to_device(&device);

        let batcher = MNISTBatcher::new(device.clone());

        let predictor = Predictor {
            model,
            batcher,
            device,
        };

        let dummy = &[0u8; 748];
        predictor.predict(dummy, 99);

        println!("Predictor initialized in {} ms....", sw.elapsed_ms());
        sw.stop();
        predictor
    })
}
