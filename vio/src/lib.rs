use wasm_bindgen::prelude::*;
use serde::Serialize;
use statrs::statistics::{Data, Min, Max, OrderStatistics};
use statrs::statistics::Distribution as StatrsDistribution;
use statrs::distribution::{Normal, Continuous};
use rand::distributions::Distribution;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand_distr::{Normal as RandNormal, Uniform as RandUniform, SkewNormal};
use num_complex::Complex;
use rustfft::{FftPlanner, num_traits::Zero};

pub mod pnf;
pub use pnf::*;

pub mod distributions;
pub use distributions::*;

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


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_calculate_violin_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = calculate_violin_data(data);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_generate_uniform_data() {
        let result = generate_uniform_data(0.0, 1.0, 0.5, 100, 0);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_generate_normal_data() {
        let result = generate_normal_data(0.0, 1.0, 0.5, 100, 0);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_generate_skewed_data() {
        let result = generate_skewed_data(0.0, 1.0, 5.0, 0.5, 100, 0);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_generate_bimodal_data() {
        let result = generate_bimodal_data(0.0, 1.0, 5.0, 1.0, 0.5, 0.5, 100, 0);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_generate_fractal_data() {
        let result = generate_fractal_data(0.7, 100, 0);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_calculate_pnf_data() {
        let data = vec![65.0, 66.0, 67.0, 66.0, 65.0, 64.0, 63.0, 62.0, 61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0];
        let result = calculate_pnf_data(data, 1.0, 3);
        assert!(result.is_ok());
    }
}