// Translation of min_test.go
// Tests for Min function

use wspace::data::Float64Data;
use wspace::load::{LoadRawData, LoadRawDataFromStrings};
use wspace::min::min;

// Helper functions from data_test.rs
fn makeFloatSlice(c: usize) -> Vec<f64> {
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    lf
}

fn makeRandFloatSlice(c: usize) -> Vec<f64> {
    // Note: In GO this uses rand.Seed but doesn't actually randomize the values
    // It just creates the same sequence as makeFloatSlice
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    lf
}

#[test]
fn ExampleMin() {
    // d := stats.LoadRawData([]interface{}{1.1, "2", 3.0, 4, "5"})
    // In Rust, we need to parse strings separately
    let raw_data = vec![1.1, 2.0, 3.0, 4.0, 5.0];
    let d = LoadRawData(&raw_data);
    let a = min(d.clone()).expect("Min failed");
    assert_eq!(a, 1.1);
    // Output: 1.1
}

#[test]
fn TestMin() {
    // Test cases: input and expected output
    let test_cases = vec![
        (vec![1.1, 2.0, 3.0, 4.0, 5.0], 1.1),
        (vec![10.534, 3.0, 5.0, 7.0, 9.0], 3.0),
        (vec![-5.0, 1.0, 5.0], -5.0),
        (vec![5.0], 5.0),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let got = min(data.clone());
        assert!(got.is_ok(), "Returned an error");
        let got = got.unwrap();
        assert_eq!(got, expected, "Min({:?}) => {} != {}", input, got, expected);
    }

    // Test empty slice
    let empty_data = Float64Data::from(vec![]);
    let result = min(empty_data.clone());
    assert!(result.is_err(), "Empty slice didn't return an error");
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    // Note: Rust doesn't have built-in benchmark support in stable.
    // These are just regular tests that exercise the code paths.
    // For actual benchmarking, use the criterion crate or nightly bench features.

    #[test]
    fn BenchmarkMinSmallFloatSlice() {
        let test_data = Float64Data::from(makeFloatSlice(5));
        for _ in 0..100 {
            let _ = min(test_data.clone());
        }
    }

    #[test]
    fn BenchmarkMinSmallRandFloatSlice() {
        let test_data = Float64Data::from(makeRandFloatSlice(5));
        for _ in 0..100 {
            let _ = min(test_data.clone());
        }
    }

    #[test]
    fn BenchmarkMinLargeFloatSlice() {
        let test_data = Float64Data::from(makeFloatSlice(100000));
        for _ in 0..10 {
            let _ = min(test_data.clone());
        }
    }

    #[test]
    fn BenchmarkMinLargeRandFloatSlice() {
        let test_data = Float64Data::from(makeRandFloatSlice(100000));
        for _ in 0..10 {
            let _ = min(test_data.clone());
        }
    }
}
