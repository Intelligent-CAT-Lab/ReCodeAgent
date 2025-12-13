// Translation of mode_test.go
// Tests for Mode function

use wspace::mode::mode;
use wspace::data::Float64Data;

// Helper functions imported from data_test pattern
fn makeFloatSlice(c: usize) -> Float64Data {
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    Float64Data::from(lf)
}

fn makeRandFloatSlice(c: usize) -> Float64Data {
    // Note: In GO this uses rand.Seed but doesn't actually randomize the values
    // It just creates the same sequence as makeFloatSlice
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    Float64Data::from(lf)
}

#[test]
fn TestMode() {
    let test_cases = vec![
        (vec![2.0, 2.0, 2.0, 2.0], vec![2.0]),
        (vec![5.0, 3.0, 4.0, 2.0, 1.0], vec![]),
        (vec![5.0, 5.0, 3.0, 3.0, 4.0, 4.0, 2.0, 2.0, 1.0, 1.0], vec![]),
        (vec![5.0, 5.0, 3.0, 4.0, 2.0, 1.0], vec![5.0]),
        (vec![5.0, 5.0, 3.0, 3.0, 4.0, 2.0, 1.0], vec![3.0, 5.0]),
        (vec![1.0], vec![1.0]),
        (vec![-50.0, -46.325, -46.325, -0.87, 1.0, 2.1122, 3.20, 5.0, 15.0, 15.0, 15.0001], vec![-46.325, 15.0]),
        (vec![1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0, 4.0, 5.0, 3.0, 6.0, 7.0, 5.0, 0.0, 8.0, 8.0, 7.0, 6.0, 9.0, 9.0], vec![4.0]),
        (vec![76.0, 76.0, 110.0, 76.0, 76.0, 76.0, 76.0, 119.0, 76.0, 76.0, 76.0, 76.0, 31.0, 31.0, 31.0, 31.0, 83.0, 83.0, 83.0, 78.0, 78.0, 78.0, 78.0, 78.0, 78.0, 78.0, 78.0], vec![76.0]),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let got = mode(data.clone()).expect("Mode failed");
        assert_eq!(got, expected, "Mode({:?}) => {:?} != {:?}", input, got, expected);
    }

    // Test empty slice should return an error
    let result = mode(Float64Data::from(vec![]));
    assert!(result.is_err(), "Empty slice should have returned an error");
}

#[test]
fn BenchmarkModeSmallFloatSlice() {
    // Note: Rust doesn't have built-in benchmarking in stable
    // This test validates the function works for the benchmark case
    let data = makeFloatSlice(5);
    let _ = mode(data.clone());
}

#[test]
fn BenchmarkModeSmallRandFloatSlice() {
    // Note: Rust doesn't have built-in benchmarking in stable
    // This test validates the function works for the benchmark case
    let lf = makeRandFloatSlice(5);
    let _ = mode(lf.clone());
}

#[test]
fn BenchmarkModeLargeFloatSlice() {
    // Note: Rust doesn't have built-in benchmarking in stable
    // This test validates the function works for the benchmark case
    let lf = makeFloatSlice(100000);
    let _ = mode(lf.clone());
}

#[test]
fn BenchmarkModeLargeRandFloatSlice() {
    // Note: Rust doesn't have built-in benchmarking in stable
    // This test validates the function works for the benchmark case
    let lf = makeRandFloatSlice(100000);
    let _ = mode(lf.clone());
}
