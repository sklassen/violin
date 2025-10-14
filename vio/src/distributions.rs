use wasm_bindgen::prelude::*;
use rand::distributions::Distribution;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand_distr::{Normal as RandNormal, Uniform as RandUniform, SkewNormal};
use num_complex::Complex;
use rustfft::{FftPlanner, num_traits::Zero};

// Helper function for AR(1) process for simple distributions
fn generate_ar1_series<D, R>(dist: D, ar_coeff: f64, n_samples: usize, rng: &mut R) -> Vec<f64>
where
    D: Distribution<f64>,
    R: Rng + ?Sized,
{
    let mut series = Vec::with_capacity(n_samples);
    if n_samples == 0 {
        return series;
    }

    let mut last_value = dist.sample(rng);
    series.push(last_value);

    for _ in 1..n_samples {
        let new_value = ar_coeff * last_value + (1.0 - ar_coeff) * dist.sample(rng);
        series.push(new_value);
        last_value = new_value;
    }
    series
}


#[wasm_bindgen]
pub fn generate_uniform_data(min: f64, max: f64, ar_coeff: f64, n_samples: usize, seed: u32) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let dist = RandUniform::new(min, max);
    generate_ar1_series(dist, ar_coeff, n_samples, &mut rng)
}

#[wasm_bindgen]
pub fn generate_normal_data(mean: f64, std_dev: f64, ar_coeff: f64, n_samples: usize, seed: u32) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let dist = RandNormal::new(mean, std_dev).unwrap();
    generate_ar1_series(dist, ar_coeff, n_samples, &mut rng)
}

#[wasm_bindgen]
pub fn generate_skewed_data(mean: f64, std_dev: f64, skew: f64, ar_coeff: f64, n_samples: usize, seed: u32) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let dist = SkewNormal::new(mean, std_dev, skew).unwrap();
    generate_ar1_series(dist, ar_coeff, n_samples, &mut rng)
}

#[wasm_bindgen]
pub fn generate_bimodal_data(
    mean1: f64, std_dev1: f64,
    mean2: f64, std_dev2: f64,
    weight: f64, // Weight for the first distribution
    ar_coeff: f64,
    n_samples: usize,
    seed: u32
) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
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

#[wasm_bindgen]
pub fn generate_fractal_data(hurst: f64, n_samples: usize, seed: u32) -> Vec<f64> {
    if n_samples == 0 {
        return Vec::new();
    }
    if n_samples == 1 {
        return vec![0.0];
    }

    let n = n_samples - 1;
    let m = 2 * n;

    // Step 1: Calculate the autocovariance function (ACF) of fGn
    let mut acf = vec![0.0; n + 1];
    for i in 0..=n {
        let k = i as f64;
        acf[i] = 0.5 * ((k + 1.0).powf(2.0 * hurst) - 2.0 * k.powf(2.0 * hurst) + (k - 1.0).abs().powf(2.0 * hurst));
    }

    // Step 2: Create the first row of the circulant covariance matrix
    let mut circulant_row = vec![0.0; m];
    circulant_row[0] = acf[0];
    for i in 1..n {
        circulant_row[i] = acf[i];
        circulant_row[m - i] = acf[i];
    }
    circulant_row[n] = acf[n];

    // Step 3: Use FFT to find the eigenvalues
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(m);
    let mut buffer: Vec<Complex<f64>> = circulant_row.into_iter().map(|v| Complex::new(v, 0.0)).collect();
    fft.process(&mut buffer);
    let eigenvalues = buffer;

    // Step 4: Generate complex random numbers
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let normal = RandNormal::new(0.0, 1.0).unwrap();
    let mut z = vec![Complex::zero(); m];
    z[0] = Complex::new(normal.sample(&mut rng) * (eigenvalues[0].re * m as f64).sqrt(), 0.0);
    z[n] = Complex::new(normal.sample(&mut rng) * (eigenvalues[n].re * m as f64).sqrt(), 0.0);
    for i in 1..n {
        let re = normal.sample(&mut rng);
        let im = normal.sample(&mut rng);
        z[i] = Complex::new(re, im) * (0.5 * m as f64 * eigenvalues[i].re).sqrt();
        z[m - i] = z[i].conj();
    }

    // Step 5 & 6: Perform inverse FFT
    let ifft = planner.plan_fft_inverse(m);
    ifft.process(&mut z);
    let fgn: Vec<f64> = z.into_iter().take(n).map(|v| v.re / m as f64).collect();

    // Step 7: Calculate cumulative sum for fBm
    let mut fbm = vec![0.0; n_samples];
    for i in 0..n {
        fbm[i+1] = fbm[i] + fgn[i];
    }
    fbm
}
