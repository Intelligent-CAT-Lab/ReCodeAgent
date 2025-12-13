// Translation of cumulative_sum_test.go
// Tests for CumulativeSum function

use wspace::cumulative_sum::cumulative_sum;
use wspace::data::Float64Data;
use wspace::errors::ERR_EMPTY_INPUT;

// Helper function to create float slices for benchmarks
fn makeFloatSlice(c: usize) -> Vec<f64> {
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    lf
}

#[test]
fn ExampleCumulativeSum() {
    let data = Float64Data::from(vec![1.0, 2.1, 3.2, 4.823, 4.1, 5.8]);
    let csum: Vec<f64> = cumulative_sum(&data).unwrap().into();
    // Output: [1 3.1 6.300000000000001 11.123000000000001 15.223 21.023]
    assert_eq!(csum, vec![1.0, 3.1, 6.300000000000001, 11.123000000000001, 15.223, 21.023]);
}

#[test]
fn TestCumulativeSum() {
    let test_cases = vec![
        (vec![1.0, 2.0, 3.0], vec![1.0, 3.0, 6.0]),
        (vec![1.0, 1.1, 1.2, 2.2], vec![1.0, 2.1, 3.3, 5.5]),
        (vec![-1.0, -1.0, 2.0, -3.0], vec![-1.0, -2.0, 0.0, -3.0]),
    ];

    for (input, expected) in test_cases {
        let data = Float64Data::from(input.clone());
        let got = cumulative_sum(&data);
        assert!(got.is_ok(), "Returned an error");
        let got: Vec<f64> = got.unwrap().into();
        assert_eq!(got, expected, "CumulativeSum({:?}) => {:?} != {:?}", input, got, expected);
    }

    // Test empty slice
    let empty_data = Float64Data::from(vec![]);
    let result = cumulative_sum(&empty_data);
    assert!(result.is_err(), "Empty slice should have returned an error");
    if let Err(e) = result {
        assert_eq!(e.to_string(), ERR_EMPTY_INPUT.to_string());
    }
}

#[test]
#[ignore] // Benchmark test - run with: cargo test --test cumulative_sum_test -- --ignored --nocapture
fn BenchmarkCumulativeSumSmallFloatSlice() {
    use std::time::Instant;
    let iterations = 1000000;
    let start = Instant::now();

    for _ in 0..iterations {
        let data = Float64Data::from(makeFloatSlice(5));
        let _ = cumulative_sum(&data);
    }

    let duration = start.elapsed();
    println!("BenchmarkCumulativeSumSmallFloatSlice: {} iterations in {:?}", iterations, duration);
    println!("Average: {:?} per iteration", duration / iterations);
}

#[test]
#[ignore] // Benchmark test - run with: cargo test --test cumulative_sum_test -- --ignored --nocapture
fn BenchmarkCumulativeSumLargeFloatSlice() {
    use std::time::Instant;
    let lf = makeFloatSlice(100000);
    let data = Float64Data::from(lf);
    let iterations = 10000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = cumulative_sum(&data);
    }

    let duration = start.elapsed();
    println!("BenchmarkCumulativeSumLargeFloatSlice: {} iterations in {:?}", iterations, duration);
    println!("Average: {:?} per iteration", duration / iterations);
}
