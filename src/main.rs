// use anyhow::{Result, anyhow};
// use lambda_runtime::{handler_fn, Context, Error as LambdaError};
// use log::info;
// use serde_json::Value;
// use std::collections::HashMap;

// #[tokio::main]
// async fn main() -> Result<()> {
//     env_logger::init();
//     info!("Starting function");
//     let func = handler_fn(func);
//     lambda_runtime::run(func).await.map_err(|err| anyhow!(err.to_string()))
// }

// async fn func(event: Value, _: Context) -> Result<Value, LambdaError> {
//     info!("Received event: {:?}", event);
//     let numbers: Vec<f64> = serde_json::from_value(event).map_err(|err| LambdaError::from(err.to_string()))?;
//     let mut stats = HashMap::new();

//     let sum: f64 = numbers.iter().sum();
//     let count = numbers.len() as f64;
//     let mean = sum / count;

//     let mut sorted_numbers = numbers.clone();
//     sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     let median = sorted_numbers[count as usize / 2];

//     let variance: f64 = numbers.iter().map(|value| (value - mean).powi(2)).sum::<f64>() / count;
//     let std_dev = variance.sqrt();

//     let min = numbers.iter().cloned().fold(f64::NAN, f64::min);
//     let max = numbers.iter().cloned().fold(f64::NAN, f64::max);

//     stats.insert("mean", mean);
//     stats.insert("median", median);
//     stats.insert("std_dev", std_dev);
//     stats.insert("min", min);
//     stats.insert("max", max);

//     serde_json::to_value(stats).map_err(|err| LambdaError::from(err.to_string()))
// }




use aws_xray_sdk::segment::Segment;
use aws_xray_sdk::utils::get_trace_id;
use anyhow::{Result, anyhow};
use lambda_runtime::{handler_fn, Context, Error as LambdaError};
use log::info;
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting function");
    let func = handler_fn(func);
    lambda_runtime::run(func).await.map_err(|err| anyhow!(err.to_string()))
}

async fn func(event: Value, _: Context) -> Result<Value, LambdaError> {
    let trace_id = get_trace_id();
    let mut segment = Segment::new(trace_id, "rustLambda");
    segment.begin();

    info!("Received event: {:?}", event);
    let numbers: Vec<f64> = serde_json::from_value(event).map_err(|err| LambdaError::from(err.to_string()))?;
    let mut stats = HashMap::new();

    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    let mean = sum / count;

    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = sorted_numbers[count as usize / 2];

    let variance: f64 = numbers.iter().map(|value| (value - mean).powi(2)).sum::<f64>() / count;
    let std_dev = variance.sqrt();

    let min = numbers.iter().cloned().fold(f64::NAN, f64::min);
    let max = numbers.iter().cloned().fold(f64::NAN, f64::max);

    stats.insert("mean", mean);
    stats.insert("median", median);
    stats.insert("std_dev", std_dev);
    stats.insert("min", min);
    stats.insert("max", max);

    segment.end();
    serde_json::to_value(stats).map_err(|err| LambdaError::from(err.to_string()))
}