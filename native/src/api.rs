use anyhow::Result;

use crate::predictor::predictor;

pub fn init_predictor(model_path: String) -> Result<bool> {
    let _ = predictor(model_path);
    Ok(true)
}

pub fn predict(bytes: Vec<f32>) -> Result<i64> {
    let predicted = predictor("".to_string()).predict(&bytes, 4);
    Ok(predicted)
}
