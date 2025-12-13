// Test module for round function
// Translated from Go test file: round_test.go

use wspace::round::round;

#[test]
fn test_example_round() {
    let rounded = round(1.534424, 1).unwrap();
    assert_eq!(rounded, 1.5);
    println!("{}", rounded);
}

#[test]
fn test_round() {
    let test_cases = vec![
        (0.1111, 1, 0.1),
        (-0.1111, 2, -0.11),
        (5.3253, 3, 5.325),
        (5.3258, 3, 5.326),
        (5.3253, 0, 5.0),
        (5.55, 1, 5.6),
    ];

    for (number, decimals, result) in test_cases {
        let m = round(number, decimals);
        assert!(m.is_ok(), "Returned an error");
        let m = m.unwrap();
        assert_eq!(m, result, "{} != {}", m, result);
    }

    // Test NaN error
    let err = round(f64::NAN, 2);
    assert!(err.is_err(), "Round should error on NaN");
}

// Note: Rust doesn't have a direct equivalent to Go's benchmark tests.
// The benchmark function is kept here for reference but would need
// to be run with a benchmarking framework like criterion.
#[cfg(test)]
mod benchmarks {
    use super::*;

    // This is a placeholder for benchmark functionality
    // In Rust, use `cargo bench` with criterion crate for proper benchmarks
    #[allow(dead_code)]
    fn benchmark_round() {
        for _ in 0..1000 {
            let _ = round(0.1111, 1);
        }
    }
}
