
pub mod pnf;
pub use pnf::*;

pub mod distributions;
pub use distributions::*;

pub mod violin;
pub use violin::*;


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