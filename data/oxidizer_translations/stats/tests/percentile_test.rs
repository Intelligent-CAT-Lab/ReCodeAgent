// Translation of percentile_test.go
// Tests for Percentile and PercentileNearestRank functions

use wspace::data::Float64Data;
use wspace::percentile::{percentile, percentile_nearest_rank};
use wspace::errors::{ERR_BOUNDS, ERR_EMPTY_INPUT};

// Helper function to create a float slice with sequential values
fn makeFloatSlice(c: usize) -> Vec<f64> {
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    lf
}

#[test]
fn TestPercentile() {
    let m = percentile(Float64Data::from(vec![43.0, 54.0, 56.0, 61.0, 62.0, 66.0]), 90.0).unwrap();
    assert_eq!(m, 64.0, "{:.1} != {:.1}", m, 64.0);

    let m = percentile(Float64Data::from(vec![43.0]), 90.0).unwrap();
    assert_eq!(m, 43.0, "{:.1} != {:.1}", m, 43.0);

    let m = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]), 50.0).unwrap();
    assert_eq!(m, 5.0, "{:.1} != {:.1}", m, 5.0);

    let m = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]), 99.9).unwrap();
    assert_eq!(m, 9.5, "{:.1} != {:.1}", m, 9.5);

    let m = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]), 100.0).unwrap();
    assert_eq!(m, 10.0, "{:.1} != {:.1}", m, 10.0);

    let err = percentile(Float64Data::from(vec![]), 99.9).unwrap_err();
    assert_eq!(err.to_string(), ERR_EMPTY_INPUT.to_string(), "Empty slice didn't return expected error; got {:?}", err);

    let err = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 0.0).unwrap_err();
    assert_eq!(err.to_string(), ERR_BOUNDS.to_string(), "Zero percent didn't return expected error; got {:?}", err);

    let err = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 0.13).unwrap_err();
    assert_eq!(err.to_string(), ERR_BOUNDS.to_string(), "Too low percent didn't return expected error; got {:?}", err);

    let err = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 101.0).unwrap_err();
    assert_eq!(err.to_string(), ERR_BOUNDS.to_string(), "Too high percent didn't return expected error; got {:?}", err);
}

#[test]
fn TestPercentileSortSideEffects() {
    let s = Float64Data::from(vec![43.0, 54.0, 56.0, 44.0, 62.0, 66.0]);
    let a = Float64Data::from(vec![43.0, 54.0, 56.0, 44.0, 62.0, 66.0]);
    let _ = percentile(s.clone(), 90.0);
    assert_eq!(s, a, "{:?} != {:?}", s, a);
}

#[test]
#[ignore] // Benchmark test - run with: cargo test --test percentile_test -- --ignored --nocapture
fn BenchmarkPercentileSmallFloatSlice() {
    use std::time::Instant;
    let iterations = 1_000_000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = percentile(Float64Data::from(makeFloatSlice(5)), 50.0);
    }

    let duration = start.elapsed();
    println!("BenchmarkPercentileSmallFloatSlice: {} iterations in {:?} ({:?} per iteration)",
             iterations, duration, duration / iterations);
}

#[test]
#[ignore] // Benchmark test - run with: cargo test --test percentile_test -- --ignored --nocapture
fn BenchmarkPercentileLargeFloatSlice() {
    use std::time::Instant;
    let lf = Float64Data::from(makeFloatSlice(100000));
    let iterations = 10_000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = percentile(lf.clone(), 50.0);
    }

    let duration = start.elapsed();
    println!("BenchmarkPercentileLargeFloatSlice: {} iterations in {:?} ({:?} per iteration)",
             iterations, duration, duration / iterations);
}

#[test]
fn TestPercentileNearestRank() {
    let f1 = Float64Data::from(vec![35.0, 20.0, 15.0, 40.0, 50.0]);
    let f2 = Float64Data::from(vec![20.0, 6.0, 7.0, 8.0, 8.0, 10.0, 13.0, 15.0, 16.0, 3.0]);
    let f3 = Float64Data::from(makeFloatSlice(101));

    let test_cases = vec![
        (&f1, 30.0, 20.0),
        (&f1, 40.0, 20.0),
        (&f1, 50.0, 35.0),
        (&f1, 75.0, 40.0),
        (&f1, 95.0, 50.0),
        (&f1, 99.0, 50.0),
        (&f1, 99.9, 50.0),
        (&f1, 100.0, 50.0),
        (&f2, 25.0, 7.0),
        (&f2, 50.0, 8.0),
        (&f2, 75.0, 15.0),
        (&f2, 100.0, 20.0),
        (&f3, 1.0, 100.0),
        (&f3, 99.0, 9900.0),
        (&f3, 100.0, 10000.0),
        (&f3, 0.0, 0.0),
    ];

    for (sample, percent, result) in test_cases {
        let got = percentile_nearest_rank(sample.clone(), percent);
        if let Err(_) = got {
            panic!("Should not have returned an error");
        }
        let got = got.unwrap();
        assert_eq!(got, result, "{} != {}", got, result);
    }

    let err = percentile_nearest_rank(Float64Data::from(vec![]), 50.0);
    assert!(err.is_err(), "Should have returned an empty slice error");

    let err = percentile_nearest_rank(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), -0.01);
    assert!(err.is_err(), "Should have returned an percentage must be above 0 error");

    let err = percentile_nearest_rank(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 110.0);
    assert!(err.is_err(), "Should have returned an percentage must not be above 100 error");
}

#[test]
#[ignore] // Benchmark test - run with: cargo test --test percentile_test -- --ignored --nocapture
fn BenchmarkPercentileNearestRankSmallFloatSlice() {
    use std::time::Instant;
    let iterations = 1_000_000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = percentile_nearest_rank(Float64Data::from(makeFloatSlice(5)), 50.0);
    }

    let duration = start.elapsed();
    println!("BenchmarkPercentileNearestRankSmallFloatSlice: {} iterations in {:?} ({:?} per iteration)",
             iterations, duration, duration / iterations);
}

#[test]
#[ignore] // Benchmark test - run with: cargo test --test percentile_test -- --ignored --nocapture
fn BenchmarkPercentileNearestRankLargeFloatSlice() {
    use std::time::Instant;
    let lf = Float64Data::from(makeFloatSlice(100000));
    let iterations = 10_000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = percentile_nearest_rank(lf.clone(), 50.0);
    }

    let duration = start.elapsed();
    println!("BenchmarkPercentileNearestRankLargeFloatSlice: {} iterations in {:?} ({:?} per iteration)",
             iterations, duration, duration / iterations);
}
