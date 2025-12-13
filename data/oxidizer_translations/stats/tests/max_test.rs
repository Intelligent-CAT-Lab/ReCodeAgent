// Translation of max_test.go
// Tests for Max function

use wspace::data::Float64Data;
use wspace::load::LoadRawData;
use wspace::max::max;

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
fn ExampleMax() {
    let d = vec![1.1, 2.3, 3.2, 4.0, 4.01, 5.09];
    let data = LoadRawData(&d);
    let a = max(data.clone()).expect("Max failed");
    assert_eq!(a, 5.09);
    // Output: 5.09
}

#[test]
fn TestMax() {
    // Test cases: input and expected output
    let test_cases = vec![
        (vec![1.0, 2.0, 3.0, 4.0, 5.0], 5.0),
        (vec![10.5, 3.0, 5.0, 7.0, 9.0], 10.5),
        (vec![-20.0, -1.0, -5.5], -1.0),
        (vec![-1.0], -1.0),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let got = max(data.clone());
        assert!(got.is_ok(), "Returned an error");
        let got = got.unwrap();
        assert_eq!(got, expected, "Max({:?}) => {} != {}", input, got, expected);
    }

    // Test empty slice
    let empty_data = Float64Data::from(vec![]);
    let result = max(empty_data.clone());
    assert!(result.is_err(), "Empty slice didn't return an error");
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    // Note: Rust doesn't have built-in benchmark support in stable.
    // These are just regular tests that exercise the code paths.
    // For actual benchmarking, use the criterion crate or nightly bench features.

    #[test]
    fn BenchmarkMaxSmallFloatSlice() {
        for _ in 0..100 {
            let data = Float64Data::from(makeFloatSlice(5));
            let _ = max(data.clone());
        }
    }

    #[test]
    fn BenchmarkMaxLargeFloatSlice() {
        let lf = makeFloatSlice(100000);
        let data = Float64Data::from(lf);
        for _ in 0..10 {
            let _ = max(data.clone());
        }
    }
}
