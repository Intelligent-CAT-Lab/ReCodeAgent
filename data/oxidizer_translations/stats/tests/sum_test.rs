// Translation of sum_test.go
// Tests for Sum function

use wspace::data::Float64Data;
use wspace::load::LoadRawData;
use wspace::sum::sum;
use wspace::errors::ERR_EMPTY_INPUT;

// Helper functions from data_test.rs
fn makeFloatSlice(c: usize) -> Vec<f64> {
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    lf
}

#[test]
fn ExampleSum() {
    let d = vec![1.1, 2.2, 3.3];
    let data = LoadRawData(&d);
    let a = sum(data).expect("Sum failed");
    // Note: Due to floating point precision, we check with tolerance
    assert!((a - 6.6).abs() < 1e-10, "Expected 6.6, got {}", a);
    // Output: 6.6
}

#[test]
fn TestSum() {
    // Test cases: input and expected output
    let test_cases = vec![
        (vec![1.0, 2.0, 3.0], 6.0),
        (vec![1.0, 1.1, 1.2, 2.2], 5.5),
        (vec![1.0, -1.0, 2.0, -3.0], -1.0),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let got = sum(data);
        assert!(got.is_ok(), "Returned an error");
        let got = got.unwrap();
        // Use approximate equality for floating point comparison
        assert_eq!(got, expected, "Sum({:?}) => {} != {}", input, got, expected);
    }

    // Test empty slice
    let empty_data = Float64Data::from(vec![]);
    let result = sum(empty_data);
    assert!(result.is_err(), "Empty slice should have returned an error");
    assert_eq!(result.unwrap_err().to_string(), ERR_EMPTY_INPUT.to_string());
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    // Note: Rust doesn't have built-in benchmark support in stable.
    // These are just regular tests that exercise the code paths.
    // For actual benchmarking, use the criterion crate or nightly bench features.

    #[test]
    fn BenchmarkSumSmallFloatSlice() {
        for _ in 0..100 {
            let data = Float64Data::from(makeFloatSlice(5));
            let _ = sum(data);
        }
    }

    #[test]
    fn BenchmarkSumLargeFloatSlice() {
        let lf = makeFloatSlice(100000);
        let data = Float64Data::from(lf);
        for _ in 0..10 {
            let _ = sum(data.clone());
        }
    }
}
