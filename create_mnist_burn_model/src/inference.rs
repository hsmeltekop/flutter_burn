use crate::{data::MNISTBatcher, training::TrainingConfig};
use burn::data::dataset::source::huggingface::MNISTItem;
use burn::{
    config::Config,
    data::dataloader::batcher::Batcher,
    module::Module,
    record::{CompactRecorder, Recorder},
    tensor::backend::Backend,
};
use stopwatch::Stopwatch;

pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, item: MNISTItem) {
    let sw = Stopwatch::start_new();
    // do something that takes some time

    // let config =
    //     TrainingConfig::load(format!("{artifact_dir}/config.json")).expect("A config exists");

    let config_bytes = include_bytes!("../guide/config.json");
    let config = TrainingConfig::load_binary(config_bytes).unwrap();

    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into())
        .expect("Failed to load trained model");

    let model = config.model.init_with::<B>(record).to_device(&device);

    println!("Loading model took {}ms", sw.elapsed_ms());

    let sw = Stopwatch::start_new();

    let label = item.label;
    let batcher = MNISTBatcher::new(device);
    let batch = batcher.batch(vec![item]);
    let output = model.forward(batch.images);
    let predicted = &output.argmax(1).flatten::<1>(0, 1).into_scalar();

    println!("Predicted {} Expected {}", predicted, label);
    println!("Making prediction took {}ms", sw.elapsed_ms());
}
