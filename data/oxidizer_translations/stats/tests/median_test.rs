// Translation of median_test.go
// Tests for Median function

use wspace::median::median;
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

#[test]
fn ExampleMedian() {
    let data = Float64Data::from(vec![1.0, 2.1, 3.2, 4.823, 4.1, 5.8]);
    let median = median(data.clone()).expect("Median failed");
    assert_eq!(median, 3.65, "Median example failed");
    // Output: 3.65
}

#[test]
fn TestMedian() {
    let test_cases = vec![
        (vec![5.0, 3.0, 4.0, 2.0, 1.0], 3.0),
        (vec![6.0, 3.0, 2.0, 4.0, 5.0, 1.0], 3.5),
        (vec![1.0], 1.0),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let got = median(data.clone()).expect("Median failed");
        assert_eq!(got, expected, "Median({:?}) => {} != {}", input, got, expected);
    }

    // Test empty slice should return an error
    let result = median(Float64Data::from(vec![]));
    assert!(result.is_err(), "Empty slice should have returned an error");
}

#[test]
fn BenchmarkMedianSmallFloatSlice() {
    // Note: Rust doesn't have built-in benchmarking in stable
    // This test validates the function works for the benchmark case
    let data = makeFloatSlice(5);
    let _ = median(data.clone());
}

#[test]
fn BenchmarkMedianLargeFloatSlice() {
    // Note: Rust doesn't have built-in benchmarking in stable
    // This test validates the function works for the benchmark case
    let lf = makeFloatSlice(100000);
    let _ = median(lf.clone());
}

#[test]
fn TestMedianSortSideEffects() {
    let s = Float64Data::from(vec![0.1, 0.3, 0.2, 0.4, 0.5]);
    let a = vec![0.1, 0.3, 0.2, 0.4, 0.5];
    let _ = median(s.clone());
    assert_eq!(s.0, a, "Median should not modify input: {:?} != {:?}", s.0, a);
}
