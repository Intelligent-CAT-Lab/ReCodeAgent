// Translation of mean_test.go
// Tests for Mean, GeometricMean, and HarmonicMean functions

use wspace::data::Float64Data;
use wspace::mean::{mean, geometric_mean, harmonic_mean};
use wspace::round::round;

// Helper function from data_test.rs
fn makeFloatSlice(c: usize) -> Vec<f64> {
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    lf
}

#[test]
fn TestMean() {
    // Test cases: input and expected output
    let test_cases = vec![
        (vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0),
        (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3.5),
        (vec![1.0], 1.0),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let got = mean(data.clone());
        assert!(got.is_ok(), "Returned an error");
        let got = got.unwrap();
        assert_eq!(got, expected, "Mean({:?}) => {} != {}", input, got, expected);
    }

    // Test empty slice
    let empty_data = Float64Data::from(vec![]);
    let result = mean(empty_data.clone());
    assert!(result.is_err(), "Empty slice should have returned an error");
}

#[test]
fn TestGeometricMean() {
    let s1 = vec![2.0, 18.0];
    let s2 = vec![10.0, 51.2, 8.0];
    let s3 = vec![1.0, 3.0, 9.0, 27.0, 81.0];

    // Test cases: input and expected output
    let test_cases = vec![
        (s1, 6.0),
        (s2, 16.0),
        (s3, 9.0),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let gm = geometric_mean(&data);
        assert!(gm.is_ok(), "Should not have returned an error");

        let gm = gm.unwrap();
        let gm = round(gm, 0).expect("Round failed");
        assert_eq!(gm, expected, "Geometric Mean {} != {}", gm, expected);
    }

    // Test empty slice
    let empty_data = Float64Data::from(vec![]);
    let result = geometric_mean(&empty_data);
    assert!(result.is_err(), "Empty slice should have returned an error");
}

#[test]
fn TestHarmonicMean() {
    let s1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let s2 = vec![10.0, -51.2, 8.0];
    let s3 = vec![1.0, 0.0, 9.0, 27.0, 81.0];

    let data1 = Float64Data::from(s1);
    let hm = harmonic_mean(data1.clone());
    assert!(hm.is_ok(), "Should not have returned an error");

    let hm = hm.unwrap();
    let hm = round(hm, 2).expect("Round failed");
    assert_eq!(hm, 2.19, "Harmonic Mean {} != {}", hm, 2.19);

    let data2 = Float64Data::from(s2);
    let result = harmonic_mean(data2);
    assert!(result.is_err(), "Should have returned Err for negative values");

    let data3 = Float64Data::from(s3);
    let result = harmonic_mean(data3);
    assert!(result.is_err(), "Should have returned Err for zero values");

    // Test empty slice
    let empty_data = Float64Data::from(vec![]);
    let result = harmonic_mean(empty_data);
    assert!(result.is_err(), "Empty slice should have returned an error");
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    // Note: Rust doesn't have built-in benchmark support in stable.
    // These are just regular tests that exercise the code paths.
    // For actual benchmarking, use the criterion crate or nightly bench features.

    #[test]
    fn BenchmarkMeanSmallFloatSlice() {
        for _ in 0..100 {
            let data = Float64Data::from(makeFloatSlice(5));
            let _ = mean(data.clone());
        }
    }

    #[test]
    fn BenchmarkMeanLargeFloatSlice() {
        let lf = makeFloatSlice(100000);
        let data = Float64Data::from(lf);
        for _ in 0..10 {
            let _ = mean(data.clone());
        }
    }
}
