use wasm_bindgen::prelude::*;
use serde::Serialize;
use statrs::statistics::{Data, Min, Max, OrderStatistics};
use statrs::statistics::Distribution as StatrsDistribution;
use statrs::distribution::{Normal, Continuous};
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::{Normal as RandNormal, Uniform as RandUniform, SkewNormal};
use rand::Rng;

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

#[derive(Serialize, Clone, Copy, PartialEq, Debug)]
pub enum PnfDirection {
    Up,
    Down,
}

#[derive(Serialize, Debug)]
pub struct PnfColumn {
    pub direction: PnfDirection,
    pub from: f64,
    pub to: f64,
}

#[wasm_bindgen]
pub fn calculate_pnf_data(
    time_series: Vec<f64>,
    box_size: f64,
    reversal_amount: usize,
) -> Result<JsValue, JsValue> {
    if time_series.len() < 2 || box_size <= 0.0 {
        return Ok(serde_wasm_bindgen::to_value(&Vec::<PnfColumn>::new()).unwrap());
    }

    let mut columns: Vec<PnfColumn> = Vec::new();
    let reversal_distance = box_size * reversal_amount as f64;

    let mut direction: Option<PnfDirection> = None;
    let mut column_start_price = time_series[0];
    let mut last_extreme = time_series[0]; // High for up-column, low for down-column

    for &price in time_series.iter().skip(1) {
        if let Some(dir) = direction {
            match dir {
                PnfDirection::Up => {
                    if price > last_extreme {
                        last_extreme = price;
                    } else if price <= last_extreme - reversal_distance {
                        columns.push(PnfColumn { direction: PnfDirection::Up, from: column_start_price, to: last_extreme });
                        direction = Some(PnfDirection::Down);
                        column_start_price = last_extreme;
                        last_extreme = price;
                    }
                }
                PnfDirection::Down => {
                    if price < last_extreme {
                        last_extreme = price;
                    } else if price >= last_extreme + reversal_distance {
                        columns.push(PnfColumn { direction: PnfDirection::Down, from: column_start_price, to: last_extreme });
                        direction = Some(PnfDirection::Up);
                        column_start_price = last_extreme;
                        last_extreme = price;
                    }
                }
            }
        } else {
            // Establish initial direction
            if price >= column_start_price + box_size {
                direction = Some(PnfDirection::Up);
                last_extreme = price;
            } else if price <= column_start_price - box_size {
                direction = Some(PnfDirection::Down);
                last_extreme = price;
            }
        }
    }

    // Add the last, uncommitted column
    if let Some(dir) = direction {
        columns.push(PnfColumn {
            direction: dir,
            from: column_start_price,
            to: last_extreme,
        });
    }

    Ok(serde_wasm_bindgen::to_value(&columns).unwrap())
}

// Helper function for AR(1) process for simple distributions
fn generate_ar1_series<D>(dist: D, ar_coeff: f64, n_samples: usize) -> Vec<f64>
where
    D: Distribution<f64>,
{
    let mut rng = thread_rng();
    let mut series = Vec::with_capacity(n_samples);
    if n_samples == 0 {
        return series;
    }

    let mut last_value = dist.sample(&mut rng);
    series.push(last_value);

    for _ in 1..n_samples {
        let new_value = ar_coeff * last_value + (1.0 - ar_coeff) * dist.sample(&mut rng);
        series.push(new_value);
        last_value = new_value;
    }
    series
}


#[wasm_bindgen]
pub fn generate_uniform_data(min: f64, max: f64, ar_coeff: f64, n_samples: usize) -> Vec<f64> {
    let dist = RandUniform::new(min, max);
    generate_ar1_series(dist, ar_coeff, n_samples)
}

#[wasm_bindgen]
pub fn generate_normal_data(mean: f64, std_dev: f64, ar_coeff: f64, n_samples: usize) -> Vec<f64> {
    let dist = RandNormal::new(mean, std_dev).unwrap();
    generate_ar1_series(dist, ar_coeff, n_samples)
}

#[wasm_bindgen]
pub fn generate_skewed_data(mean: f64, std_dev: f64, skew: f64, ar_coeff: f64, n_samples: usize) -> Vec<f64> {
    let dist = SkewNormal::new(mean, std_dev, skew).unwrap();
    generate_ar1_series(dist, ar_coeff, n_samples)
}

#[wasm_bindgen]
pub fn generate_bimodal_data(
    mean1: f64, std_dev1: f64,
    mean2: f64, std_dev2: f64,
    weight: f64, // Weight for the first distribution
    ar_coeff: f64,
    n_samples: usize
) -> Vec<f64> {
    let mut rng = thread_rng();
    let dist1 = RandNormal::new(mean1, std_dev1).unwrap();
    let dist2 = RandNormal::new(mean2, std_dev2).unwrap();
    let uniform = RandUniform::new(0.0, 1.0);

    let mut series = Vec::with_capacity(n_samples);
    if n_samples == 0 {
        return series;
    }

    let mut last_value = if uniform.sample(&mut rng) < weight {
        dist1.sample(&mut rng)
    } else {
        dist2.sample(&mut rng)
    };
    series.push(last_value);

    for _ in 1..n_samples {
        let innovation = if uniform.sample(&mut rng) < weight {
            dist1.sample(&mut rng)
        } else {
            dist2.sample(&mut rng)
        };
        let new_value = ar_coeff * last_value + (1.0 - ar_coeff) * innovation;
        series.push(new_value);
        last_value = new_value;
    }
    series
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
        let result = generate_uniform_data(0.0, 1.0, 0.5, 100);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_generate_normal_data() {
        let result = generate_normal_data(0.0, 1.0, 0.5, 100);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_generate_skewed_data() {
        let result = generate_skewed_data(0.0, 1.0, 5.0, 0.5, 100);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_generate_bimodal_data() {
        let result = generate_bimodal_data(0.0, 1.0, 5.0, 1.0, 0.5, 0.5, 100);
        assert_eq!(result.len(), 100);
    }

    #[wasm_bindgen_test]
    fn test_calculate_pnf_data() {
        let data = vec![65.0, 66.0, 67.0, 66.0, 65.0, 64.0, 63.0, 62.0, 61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0];
        let result = calculate_pnf_data(data, 1.0, 3);
        assert!(result.is_ok());
    }
}