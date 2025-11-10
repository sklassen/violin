use vio::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_variance() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = variance(&data).unwrap();
    assert!((result - 2.5).abs() < 1e-9);
}

#[wasm_bindgen_test]
fn test_pearson_correlation_positive() {
    let data1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let data2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
    let result = pearson_correlation(&data1, &data2).unwrap();
    assert!((result - 1.0).abs() < 1e-9);
}

#[wasm_bindgen_test]
fn test_pearson_correlation_zero() {
    let data1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let data2 = vec![1.0, -1.0, 1.0, -1.0, 1.0];
    let result = pearson_correlation(&data1, &data2).unwrap();
    assert!(result.abs() < 1e-9);
}
