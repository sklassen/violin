use wasm_bindgen::prelude::*;
use serde::Serialize;
use statrs::statistics::{Data, Min, Max, OrderStatistics};
use statrs::statistics::Distribution as StatrsDistribution;
use statrs::distribution::{Normal, Continuous};
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::{Normal as RandNormal, Uniform as RandUniform, Exp};


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

#[derive(Serialize)]
pub struct TestData {
    skewed_normal_1: Vec<f64>,
    skewed_normal_2: Vec<f64>,
    skewed_normal_3: Vec<f64>,
    jump_diffusion_1: Vec<f64>,
    jump_diffusion_2: Vec<f64>,
    jump_diffusion_3: Vec<f64>,
}

#[wasm_bindgen]
pub fn generate_test_data() -> Result<JsValue, JsValue> {
    let mut rng = thread_rng();
    let n_samples = 300;

    // 1. Three random normal distributions with skew
    let normal_pos_skew = RandNormal::new(2.0, 1.0).unwrap();
    let exp_pos = Exp::new(1.0).unwrap();
    let skewed_normal_1: Vec<f64> = (0..n_samples).map(|_| normal_pos_skew.sample(&mut rng) + exp_pos.sample(&mut rng)).collect();

    let normal_neg_skew = RandNormal::new(-2.0, 1.0).unwrap();
    let exp_neg = Exp::new(1.0).unwrap();
    let skewed_normal_2: Vec<f64> = (0..n_samples).map(|_| normal_neg_skew.sample(&mut rng) - exp_neg.sample(&mut rng)).collect();

    let skewed_normal_3 = RandNormal::new(0.0, 1.5).unwrap().sample_iter(&mut rng).take(n_samples).collect();

    // 2. Three dual normal distributions (bimodal)
    let normal1a = RandNormal::new(-3.0, 1.0).unwrap();
    let normal1b = RandNormal::new(3.0, 1.0).unwrap();
    let uniform1 = RandUniform::new(0.0, 1.0);
    let jump_diffusion_1 = (0..n_samples).map(|_| {
        if uniform1.sample(&mut rng) < 0.5 { normal1a.sample(&mut rng) } else { normal1b.sample(&mut rng) }
    }).collect();

    let normal2a = RandNormal::new(-4.0, 0.8).unwrap();
    let normal2b = RandNormal::new(1.0, 1.5).unwrap();
    let uniform2 = RandUniform::new(0.0, 1.0);
    let jump_diffusion_2 = (0..n_samples).map(|_| {
        if uniform2.sample(&mut rng) < 0.7 { normal2a.sample(&mut rng) } else { normal2b.sample(&mut rng) }
    }).collect();

    let normal3a = RandNormal::new(0.0, 0.5).unwrap();
    let normal3b = RandNormal::new(0.0, 3.5).unwrap();
    let uniform3 = RandUniform::new(0.0, 1.0);
     let jump_diffusion_3 = (0..n_samples).map(|_| {
        if uniform3.sample(&mut rng) < 0.2 { normal3a.sample(&mut rng) } else { normal3b.sample(&mut rng) }
    }).collect();


    let test_data = TestData {
        skewed_normal_1,
        skewed_normal_2,
        skewed_normal_3,
        jump_diffusion_1,
        jump_diffusion_2,
        jump_diffusion_3,
    };

    Ok(serde_wasm_bindgen::to_value(&test_data).unwrap())
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
    fn test_generate_test_data() {
        let result = generate_test_data();
        assert!(result.is_ok());
    }
}