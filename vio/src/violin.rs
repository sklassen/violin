use wasm_bindgen::prelude::*;
use serde::Serialize;
use statrs::statistics::{Data, Min, Max, OrderStatistics};
use statrs::statistics::Distribution as StatrsDistribution;
use statrs::distribution::{Normal, Continuous};

#[derive(Serialize)]
pub struct ViolinData {
    kde_points: Vec<(f64, f64)>,
    q1: f64,
    median: f64,
    q3: f64,
    min: f64,
    max: f64,
    iqr: f64,
    upper_whisker: f64,
    lower_whisker: f64,
}

#[wasm_bindgen]
pub fn calculate_violin_data(raw_data: Vec<f64>) -> Result<JsValue, JsValue> {
    if raw_data.len() < 4 {
        return Err(JsValue::from_str("Input data must have at least 4 elements."));
    }

    let mut data = Data::new(raw_data);
    let q1 = data.quantile(0.25);
    let median = data.quantile(0.5);
    let q3 = data.quantile(0.75);
    let iqr = q3 - q1;

    let min = data.min();
    let max = data.max();

    // Standard box plot whisker calculation
    let upper_whisker = (q3 + 1.5 * iqr).min(max);
    let lower_whisker = (q1 - 1.5 * iqr).max(min);

    // Kernel Density Estimation
    let n = data.len() as f64;
    let std_dev = data.std_dev().unwrap_or(1.0);

    // Silverman's rule of thumb for bandwidth selection
    let h = 1.06 * std_dev * n.powf(-0.2);

    let kde_normal = Normal::new(0.0, h).unwrap();

    let mut kde_points = Vec::new();
    let num_points = 100; // Number of points to evaluate the KDE at
    let step = (max - min) / (num_points - 1) as f64;

    for i in 0..num_points {
        let x = min + (i as f64 * step);
        let mut density = 0.0;
        for &sample in data.iter() {
            density += kde_normal.pdf(x - sample);
        }
        density /= n;
        kde_points.push((x, density));
    }

    let result = ViolinData {
        kde_points,
        q1,
        median,
        q3,
        min,
        max,
        iqr,
        upper_whisker,
        lower_whisker,
    };

    Ok(serde_wasm_bindgen::to_value(&result).unwrap())
}
