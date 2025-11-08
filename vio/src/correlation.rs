use wasm_bindgen::prelude::*;

fn rank(data: &mut [f64]) {
    let mut ranks: Vec<(usize, &f64)> = data.iter().enumerate().collect();
    ranks.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    let mut rank_values: Vec<f64> = vec![0.0; data.len()];
    for (i, &(original_index, _)) in ranks.iter().enumerate() {
        rank_values[original_index] = i as f64 + 1.0;
    }
    for i in 0..data.len() {
        data[i] = rank_values[i];
    }
}

#[wasm_bindgen]
pub fn pearson_correlation(data1: &[f64], data2: &[f64]) -> Result<f64, JsValue> {
    if data1.len() != data2.len() {
        return Err(JsValue::from_str("Input data must have the same length."));
    }
    let n = data1.len();
    if n < 2 {
        return Ok(0.0);
    }

    let mut mean1 = 0.0;
    let mut mean2 = 0.0;
    let mut m2_1 = 0.0;
    let mut m2_2 = 0.0;
    let mut c = 0.0;

    for i in 0..n {
        let x = data1[i];
        let y = data2[i];
        let k = (i + 1) as f64;

        let dx = x - mean1;
        let dy = y - mean2;

        mean1 += dx / k;
        mean2 += dy / k;

        c += dx * (y - mean2);
        m2_1 += dx * (x - mean1);
        m2_2 += dy * (y - mean2);
    }

    let sample_var1 = m2_1 / ((n - 1) as f64);
    let sample_var2 = m2_2 / ((n - 1) as f64);
    let sample_cov = c / ((n - 1) as f64);

    let std_dev1 = sample_var1.sqrt();
    let std_dev2 = sample_var2.sqrt();

    if std_dev1 == 0.0 || std_dev2 == 0.0 {
        return Ok(0.0);
    }

    Ok(sample_cov / (std_dev1 * std_dev2))
}

#[wasm_bindgen]
pub fn spearman_correlation(data1: &mut [f64], data2: &mut [f64]) -> Result<f64, JsValue> {
    if data1.len() != data2.len() {
        return Err(JsValue::from_str("Input data must have the same length."));
    }
    rank(data1);
    rank(data2);
    pearson_correlation(data1, data2)
}

#[wasm_bindgen]
pub fn kendall_correlation(data1: &[f64], data2: &[f64]) -> Result<f64, JsValue> {
    if data1.len() != data2.len() {
        return Err(JsValue::from_str("Input data must have the same length."));
    }
    let mut concordant_pairs = 0;
    let mut discordant_pairs = 0;
    for i in 0..data1.len() {
        for j in (i + 1)..data1.len() {
            let data1_diff = data1[i] - data1[j];
            let data2_diff = data2[i] - data2[j];
            if data1_diff * data2_diff > 0.0 {
                concordant_pairs += 1;
            } else if data1_diff * data2_diff < 0.0 {
                discordant_pairs += 1;
            }
        }
    }
    Ok((concordant_pairs - discordant_pairs) as f64 / (concordant_pairs + discordant_pairs) as f64)
}

#[wasm_bindgen]
pub fn variance(data: &[f64]) -> Result<f64, JsValue> {
    let n = data.len();
    if n < 2 {
        return Ok(0.0);
    }

    let mut mean = 0.0;
    let mut m2 = 0.0;

    for i in 0..n {
        let x = data[i];
        let k = (i + 1) as f64;
        let dx = x - mean;
        mean += dx / k;
        m2 += dx * (x - mean);
    }

    Ok(m2 / ((n - 1) as f64))
}
