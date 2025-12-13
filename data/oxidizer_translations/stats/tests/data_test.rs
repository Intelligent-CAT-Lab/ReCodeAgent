// Translation of data_test.go
// Tests for Float64Data methods

use wspace::data::{Float64Data, LoadRawData};
use wspace::errors::StatsError;

// Test data matching GO test data
// In GO, data1 is a package-level variable that gets modified by TestInterfaceMethods
// After the Swap(0, 2) call, data1 becomes {5, -10.001, -10, 1.1, 2, 3, 4.20, 5}
// So most tests actually use this swapped version

fn data1_original() -> Float64Data {
    Float64Data::from(vec![-10.0, -10.001, 5.0, 1.1, 2.0, 3.0, 4.20, 5.0])
}

fn data1() -> Float64Data {
    // This is the data1 after Swap(0, 2) is called in TestInterfaceMethods
    // All subsequent tests in GO use this swapped version
    Float64Data::from(vec![5.0, -10.001, -10.0, 1.1, 2.0, 3.0, 4.20, 5.0])
}

fn data2() -> Float64Data {
    Float64Data::from(vec![-9.0, -9.001, 4.0, 0.1, 1.0, 2.0, 3.20, 5.0])
}

// Helper functions for tests

fn getFunctionName(_i: &str) -> &str {
    // In Rust we can't get function name dynamically like in GO
    // So we'll just pass the name as a string
    _i
}

fn veryclose(a: f64, b: f64) -> bool {
    tolerance(a, b, 4e-16)
}

fn tolerance(a: f64, b: f64, e: f64) -> bool {
    // Multiplying by e here can underflow denormal values to zero.
    // Check a==b so that at least if a and b are small and identical
    // we say they match.
    if a == b {
        return true;
    }
    let mut d = a - b;
    if d < 0.0 {
        d = -d;
    }

    // note: b is correct (expected) value, a is actual value.
    // make error tolerance a fraction of b, not a.
    let mut e = e;
    if b != 0.0 {
        e = e * b;
        if e < 0.0 {
            e = -e;
        }
    }
    d < e
}

fn checkResult(result: f64, err: Option<&str>, name: &str, expected: f64) -> Result<(), String> {
    if let Some(e) = err {
        return Err(format!("{} returned an error: {}", name, e));
    }
    if !veryclose(result, expected) {
        return Err(format!("{} => {} != {}", name, result, expected));
    }
    Ok(())
}

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
fn TestInterfaceMethods() {
    let mut data1 = data1_original();

    // Test Get
    let a = data1.get(1);
    assert_eq!(a, -10.001, "get(1) => {} != {}", a, -10.001);

    // Test Len
    let l = data1.len();
    assert_eq!(l, 8, "len() => {} != {}", l, 8);

    // Test Less
    let b = data1.Less(0, 5);
    assert!(b, "Less() => {} != {}", b, true);

    // Test Swap
    data1.Swap(0, 2);
    assert_eq!(data1.get(0), 5.0, "After Swap, get(0) => {} != {}", data1.get(0), 5.0);
}

#[test]
fn TestHelperMethods() {
    let data1 = data1();

    // Test Min
    let m = data1.Min().expect("Min failed");
    assert_eq!(m, -10.001, "Min() => {} != {}", m, -10.001);

    // Test Max
    let m = data1.Max().expect("Max failed");
    assert_eq!(m, 5.0, "Max() => {} != {}", m, 5.0);

    // Test Sum
    let m = data1.sum().expect("Sum failed");
    assert!(veryclose(m, 0.2990000000000004), "Sum() => {} != {}", m, 0.2990000000000004);

    // Test CumulativeSum
    let cs = data1.CumulativeSum().expect("CumulativeSum failed");
    let want = vec![5.0, -5.0009999999999994, -15.001, -13.901, -11.901, -8.901, -4.701, 0.2990000000000004];
    assert_eq!(cs.len(), want.len(), "CumulativeSum length mismatch");
    for (i, (got, expected)) in cs.iter().zip(want.iter()).enumerate() {
        assert!(veryclose(*got, *expected), "CumulativeSum[{}] => {} != {}", i, got, expected);
    }

    // Test Mean
    let m = data1.Mean().expect("Mean failed");
    assert_eq!(m, 0.03737500000000005, "Mean() => {} != {}", m, 0.03737500000000005);

    // Test GeometricMean
    let m = data1.GeometricMean().expect("GeometricMean failed");
    assert_eq!(m, 4.028070682618703, "GeometricMean() => {} != {}", m, 4.028070682618703);

    // Test HarmonicMean
    let m = data1.HarmonicMean().expect("HarmonicMean failed");
    assert!(m.is_nan(), "HarmonicMean() => {} != NaN", m);

    // Test Median
    let m = data1.Median().expect("Median failed");
    assert_eq!(m, 2.5, "Median() => {} != {}", m, 2.5);

    // Test Mode
    let mo = data1.Mode().expect("Mode failed");
    assert_eq!(mo.len(), 1, "Mode length mismatch");
    assert_eq!(mo[0], 5.0, "Mode() => {:?} != [5.0]", mo);

    // Test InterQuartileRange
    let iqr = data1.InterQuartileRange().expect("InterQuartileRange failed");
    assert_eq!(iqr, 9.05, "InterQuartileRange() => {} != {}", iqr, 9.05);
}

fn assertFloat64<F>(mut func: F, expected: f64, test_name: &str)
where
    F: FnMut() -> Result<f64, StatsError>,
{
    let res = func();
    match res {
        Ok(value) => {
            if let Err(e) = checkResult(value, None, test_name, expected) {
                panic!("{}", e);
            }
        }
        Err(e) => {
            panic!("{} returned an error: {}", test_name, e);
        }
    }
}

#[test]
fn TestMedianAbsoluteDeviationMethods() {
    let data1 = data1();
    assertFloat64(
        || data1.MedianAbsoluteDeviation(),
        2.1,
        "MedianAbsoluteDeviation"
    );
    assertFloat64(
        || data1.MedianAbsoluteDeviationPopulation(),
        2.1,
        "MedianAbsoluteDeviationPopulation"
    );
}

#[test]
fn TestStandardDeviationMethods() {
    let data1 = data1();
    assertFloat64(
        || data1.StandardDeviation(),
        5.935684731720091,
        "StandardDeviation"
    );
    assertFloat64(
        || data1.StandardDeviationPopulation(),
        5.935684731720091,
        "StandardDeviationPopulation"
    );
    assertFloat64(
        || data1.StandardDeviationSample(),
        6.345513892000508,
        "StandardDeviationSample"
    );
}

#[test]
fn TestVarianceMethods() {
    let data1 = data1();
    assertFloat64(
        || data1.Variance(),
        35.232353234375005,
        "Variance"
    );
    assertFloat64(
        || data1.PopulationVariance(),
        35.232353234375005,
        "PopulationVariance"
    );
    assertFloat64(
        || data1.SampleVariance(),
        40.26554655357143,
        "SampleVariance"
    );
}

fn assertPercentiles<F>(mut func: F, i: f64, expected: f64, test_name: &str)
where
    F: FnMut(f64) -> Result<f64, StatsError>,
{
    let res = func(i);
    match res {
        Ok(value) => {
            if let Err(e) = checkResult(value, None, test_name, expected) {
                panic!("{}", e);
            }
        }
        Err(e) => {
            panic!("{} returned an error: {}", test_name, e);
        }
    }
}

#[test]
fn TestPercentileMethods() {
    let data1 = data1();
    assertPercentiles(
        |p| data1.Percentile(p),
        75.0,
        4.2,
        "Percentile"
    );
    assertPercentiles(
        |p| data1.PercentileNearestRank(p),
        75.0,
        4.2,
        "PercentileNearestRank"
    );
}

fn assertOtherDataMethods<F>(mut func: F, d: &Float64Data, expected: f64, test_name: &str)
where
    F: FnMut(&Float64Data) -> Result<f64, StatsError>,
{
    let res = func(d);
    match res {
        Ok(value) => {
            if let Err(e) = checkResult(value, None, test_name, expected) {
                panic!("{}", e);
            }
        }
        Err(e) => {
            panic!("{} returned an error: {}", test_name, e);
        }
    }
}

#[test]
fn TestOtherDataMethods() {
    let data1 = data1();
    let data2 = data2();

    assertOtherDataMethods(
        |d| data1.Correlation(d),
        &data2,
        0.20875473597605448,
        "Correlation"
    );
    assertOtherDataMethods(
        |d| data1.Pearson(d),
        &data2,
        0.20875473597605448,
        "Pearson"
    );
    assertOtherDataMethods(
        |d| data1.Midhinge(d),
        &data2,
        -0.42500000000000004,
        "Midhinge"
    );
    assertOtherDataMethods(
        |d| data1.Trimean(d),
        &data2,
        0.5375,
        "Trimean"
    );
    assertOtherDataMethods(
        |d| data1.Covariance(d),
        &data2,
        7.3814215535714265,
        "Covariance"
    );
    assertOtherDataMethods(
        |d| data1.CovariancePopulation(d),
        &data2,
        6.458743859374998,
        "CovariancePopulation"
    );
}

#[test]
fn TestAutoCorrelationMethod() {
    let data1 = data1();
    let result = data1.AutoCorrelation(1);
    assert!(result.is_ok(), "Float64Data.AutoCorrelation returned an error");
}

#[test]
fn TestSampleMethod() {
    let data1 = data1();
    // Test Sample method
    let result = data1.Sample(5, true);
    assert!(result.is_ok(), "Sample returned an error");
}

#[test]
fn TestQuartileMethods() {
    let data1 = data1();
    let data2 = data2();

    // Test QuartileOutliers method
    let result = data1.QuartileOutliers();
    assert!(result.is_ok(), "QuartileOutliers returned an error");

    // Test Quartile method
    let result = data1.Quartile(&data2);
    assert!(result.is_ok(), "Quartile returned an error");
}

#[test]
fn TestSigmoidMethod() {
    let d = LoadRawData(&[3.0, 1.0, 2.1]);
    let a = vec![0.9525741268224334, 0.7310585786300049, 0.8909031788043871];
    let s = d.Sigmoid().expect("Sigmoid failed");
    assert_eq!(s.len(), a.len(), "Sigmoid length mismatch");
    for (i, (got, expected)) in s.iter().zip(a.iter()).enumerate() {
        assert_eq!(*got, *expected, "Sigmoid[{}] => {} != {}", i, got, expected);
    }
}

#[test]
fn TestSoftMaxMethod() {
    let d = LoadRawData(&[3.0, 1.0, 0.2]);
    let a = vec![0.8360188027814407, 0.11314284146556013, 0.05083835575299916];
    let s = d.SoftMax().expect("SoftMax failed");
    assert_eq!(s.len(), a.len(), "SoftMax length mismatch");
    for (i, (got, expected)) in s.iter().zip(a.iter()).enumerate() {
        assert_eq!(*got, *expected, "SoftMax[{}] => {} != {}", i, got, expected);
    }
}

#[test]
fn TestEntropyMethod() {
    let d = LoadRawData(&[3.0, 1.0, 0.2]);
    let a = 0.7270013625470586;
    let e = d.Entropy().expect("Entropy failed");
    assert_eq!(e, a, "Entropy() => {} != {}", e, a);
}

// Benchmarks are not directly supported in stable Rust in the same way as GO
// These would need to use the criterion crate or nightly bench features
// For now, we'll skip the benchmark tests as they're not part of the core test suite

#[test]
fn TestQuartilesMethods() {
    let data1 = data1();
    let result = data1.Quartiles();
    assert!(result.is_ok(), "Quartiles returned an error");
}

// Note: BenchmarkRegularAPI and BenchmarkMethodsAPI are omitted
// because stable Rust doesn't support benchmarks in the same way.
// Use the criterion crate for benchmarking if needed.
