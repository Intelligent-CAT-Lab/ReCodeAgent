// Translation of deviation_test.go
// Tests for StandardDeviation and MedianAbsoluteDeviation functions

use wspace::data::Float64Data;
use wspace::deviation::{
    median_absolute_deviation,
    median_absolute_deviation_population,
    standard_deviation,
    standard_deviation_population,
    standard_deviation_sample,
};
use wspace::round::round;

#[test]
fn TestMedianAbsoluteDeviation() {
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let result = median_absolute_deviation(data.clone());
    assert!(result.is_ok(), "Returned an error");
}

#[test]
fn TestMedianAbsoluteDeviationPopulation() {
    // Test case 1: [1, 2, 3]
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let s = median_absolute_deviation(data.clone());
    assert!(s.is_ok(), "Returned an error");

    let s = s.unwrap();
    let m = round(s, 2);
    assert!(m.is_ok(), "Returned an error");

    let m = m.unwrap();
    assert_eq!(m, 1.00, "{} != {}", m, 1.00);

    // Test case 2: [-2, 0, 4, 5, 7]
    let data = Float64Data::from(vec![-2.0, 0.0, 4.0, 5.0, 7.0]);
    let s = median_absolute_deviation(data.clone());
    assert!(s.is_ok(), "Returned an error");

    let s = s.unwrap();
    let m = round(s, 2);
    assert!(m.is_ok(), "Returned an error");

    let m = m.unwrap();
    assert_eq!(m, 3.00, "{} != {}", m, 3.00);

    // Test empty slice - should return NaN (via error in Rust)
    let empty_data = Float64Data::from(vec![]);
    let m = median_absolute_deviation(empty_data.clone());
    // In the Go version, empty slice returns NaN
    // In Rust version, we expect an error for empty input
    assert!(m.is_err(), "Empty slice should return an error");
}

#[test]
fn TestStandardDeviation() {
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let result = standard_deviation(data.clone());
    assert!(result.is_ok(), "Returned an error");
}

#[test]
fn TestStandardDeviationPopulation() {
    // Test case 1: [1, 2, 3]
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let s = standard_deviation_population(data.clone());
    assert!(s.is_ok(), "Returned an error");

    let s = s.unwrap();
    let m = round(s, 2);
    assert!(m.is_ok(), "Returned an error");

    let m = m.unwrap();
    assert_eq!(m, 0.82, "{} != {}", m, 0.82);

    // Test case 2: [-1, -2, -3.3]
    let data = Float64Data::from(vec![-1.0, -2.0, -3.3]);
    let s = standard_deviation_population(data.clone());
    assert!(s.is_ok(), "Returned an error");

    let s = s.unwrap();
    let m = round(s, 2);
    assert!(m.is_ok(), "Returned an error");

    let m = m.unwrap();
    assert_eq!(m, 0.94, "{} != {}", m, 0.94);

    // Test empty slice - should return NaN (via error in Rust)
    let empty_data = Float64Data::from(vec![]);
    let m = standard_deviation_population(empty_data.clone());
    // In the Go version, empty slice returns NaN
    // In Rust version, we expect an error for empty input
    assert!(m.is_err(), "Empty slice should return an error");
}

#[test]
fn TestStandardDeviationSample() {
    // Test case 1: [1, 2, 3]
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let s = standard_deviation_sample(data.clone());
    assert!(s.is_ok(), "Returned an error");

    let s = s.unwrap();
    let m = round(s, 2);
    assert!(m.is_ok(), "Returned an error");

    let m = m.unwrap();
    assert_eq!(m, 1.0, "{} != {}", m, 1.0);

    // Test case 2: [-1, -2, -3.3]
    let data = Float64Data::from(vec![-1.0, -2.0, -3.3]);
    let s = standard_deviation_sample(data.clone());
    assert!(s.is_ok(), "Returned an error");

    let s = s.unwrap();
    let m = round(s, 2);
    assert!(m.is_ok(), "Returned an error");

    let m = m.unwrap();
    assert_eq!(m, 1.15, "{} != {}", m, 1.15);

    // Test empty slice - should return NaN (via error in Rust)
    let empty_data = Float64Data::from(vec![]);
    let m = standard_deviation_sample(empty_data.clone());
    // In the Go version, empty slice returns NaN
    // In Rust version, we expect an error for empty input
    assert!(m.is_err(), "Empty slice should return an error");
}
