// Translated from GO describe_test.go
// Tests for the describe module

use wspace::describe::describe;
use wspace::errors::ERR_EMPTY_INPUT;
use wspace::mean::mean;
use wspace::deviation::standard_deviation;
use wspace::max::max;
use wspace::min::min;
use wspace::data::Float64Data;

#[test]
#[allow(non_snake_case)]
fn TestDescribeValidDataset() {
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let percentiles = vec![25.0, 50.0, 75.0];
    let result = describe(data.clone(), false, Some(&percentiles));
    assert!(result.is_ok(), "Returned an error");
}

#[test]
#[allow(non_snake_case)]
fn TestDescribeEmptyDataset() {
    let data = Float64Data::from(vec![]);
    let result = describe(data.clone(), false, None);
    assert!(result.is_err(), "Did not return empty input error");
    let err = result.unwrap_err();
    let expected_err = ERR_EMPTY_INPUT.to_string();
    assert_eq!(err.to_string(), expected_err, "Did not return empty input error");
}

#[test]
#[allow(non_snake_case)]
fn TestDescribeEmptyDatasetNaN() {
    let data = Float64Data::from(vec![]);
    let result = describe(data.clone(), true, None);
    assert!(result.is_ok(), "Returned an error");

    let describe = result.unwrap();
    assert!(describe.max.is_nan(), "Was not NaN");
    assert!(describe.mean.is_nan(), "Was not NaN");
    assert!(describe.min.is_nan(), "Was not NaN");
    assert!(describe.std.is_nan(), "Was not NaN");
}

#[test]
#[allow(non_snake_case)]
fn TestDescribeValidDatasetNaN() {
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let percentiles = vec![25.0, 50.0, 75.0];
    let result = describe(data.clone(), true, Some(&percentiles));
    assert!(result.is_ok(), "Returned an error");

    let describe = result.unwrap();
    assert!(!describe.max.is_nan(), "Was NaN");
}

#[test]
#[allow(non_snake_case)]
fn TestDescribeValues() {
    let data_set = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let percentiles = vec![25.0, 50.0, 75.0];
    let describe = describe(data_set.clone(), true, Some(&percentiles)).unwrap();

    assert_eq!(describe.count as usize, data_set.len(), "Count was not == length of dataset");
    assert_eq!(
        describe.description_percentiles.len(),
        percentiles.len(),
        "Percentiles length was not == length of input percentiles"
    );

    let max = max(data_set.clone()).unwrap();
    assert_eq!(max, describe.max, "Max was not equal to max(dataset)");

    let min = min(data_set.clone()).unwrap();
    assert_eq!(min, describe.min, "Min was not equal to min(dataset)");

    let mean_val = mean(data_set.clone()).unwrap();
    assert_eq!(mean_val, describe.mean, "Mean was not equal to mean(dataset)");

    let std = standard_deviation(data_set.clone()).unwrap();
    assert_eq!(std, describe.std, "Std was not equal to standard_deviation(dataset)");
}

#[test]
#[allow(non_snake_case)]
fn TestDescribeString() {
    let data = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let percentiles = vec![25.0, 50.0, 75.0];
    let describe = describe(data.clone(), true, Some(&percentiles)).unwrap();

    let expected = "count\t3\nmean\t2.00\nstd\t0.82\nmax\t3.00\nmin\t1.00\n25.00%\tNaN\n50.00%\t1.50\n75.00%\t2.50\nNaN OK\ttrue";
    let result = describe.String(2);
    assert_eq!(result, expected, "String output is not correct");
}
