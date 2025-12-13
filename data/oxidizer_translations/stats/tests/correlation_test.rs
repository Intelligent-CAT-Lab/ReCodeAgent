use wspace::correlation::{auto_correlation, correlation, Pearson};
use wspace::data::Float64Data;
use wspace::round::round;
use wspace::errors::{ERR_EMPTY_INPUT, ERR_SIZE};

// Helper function for approximate float comparisons
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

fn veryclose(a: f64, b: f64) -> bool {
    tolerance(a, b, 4e-16)
}

#[test]
fn ExampleCorrelation() {
    let s1 = Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let s2 = Float64Data::from(vec![1.0, 2.0, 3.0, 5.0, 6.0]);
    let a = correlation(s1.clone(), s2.clone()).unwrap();
    let rounded = round(a, 5).unwrap();
    assert_eq!(rounded, 0.99124);
    // Output: 0.99124
}

#[test]
fn TestCorrelation() {
    let s1 = Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let s2 = Float64Data::from(vec![10.0, -51.2, 8.0]);
    let s3 = Float64Data::from(vec![1.0, 2.0, 3.0, 5.0, 6.0]);
    let s4 = Float64Data::from(vec![]);
    let s5 = Float64Data::from(vec![0.0, 0.0, 0.0]);

    struct TestCase {
        name: &'static str,
        input: (Float64Data, Float64Data),
        output: f64,
        err: Option<String>,
    }

    let test_cases = vec![
        TestCase {
            name: "Empty Slice Error",
            input: (s4.clone(), s4.clone()),
            output: f64::NAN,
            err: Some(ERR_EMPTY_INPUT.to_string()),
        },
        TestCase {
            name: "Different Length Error",
            input: (s1.clone(), s2.clone()),
            output: f64::NAN,
            err: Some(ERR_SIZE.to_string()),
        },
        TestCase {
            name: "Correlation Value",
            input: (s1.clone(), s3.clone()),
            output: 0.9912407071619302,
            err: None,
        },
        TestCase {
            name: "Same Input Value",
            input: (s5.clone(), s5.clone()),
            output: 0.00,
            err: None,
        },
    ];

    for tc in test_cases {
        let a = correlation(tc.input.0.clone(), tc.input.1.clone());
        match a {
            Err(e) => {
                if let Some(expected_err) = &tc.err {
                    assert_eq!(
                        e.to_string(),
                        expected_err.as_str(),
                        "{}: Should have returned error {}",
                        tc.name,
                        expected_err
                    );
                } else {
                    panic!("{}: Unexpected error: {}", tc.name, e);
                }
            }
            Ok(result) => {
                if tc.err.is_some() {
                    panic!("{}: Should have returned an error", tc.name);
                }
                if !veryclose(result, tc.output) {
                    panic!(
                        "{}: Result {:.08} should be {:.08}",
                        tc.name, result, tc.output
                    );
                }
            }
        }

        let a2 = Pearson(tc.input.0.clone(), tc.input.1.clone());
        match a2 {
            Err(e) => {
                if let Some(expected_err) = &tc.err {
                    assert_eq!(
                        e.to_string(),
                        expected_err.as_str(),
                        "{}: Should have returned error {}",
                        tc.name,
                        expected_err
                    );
                } else {
                    panic!("{}: Unexpected error: {}", tc.name, e);
                }
            }
            Ok(result) => {
                if tc.err.is_some() {
                    panic!("{}: Should have returned an error", tc.name);
                }
                if !veryclose(result, tc.output) {
                    panic!(
                        "{}: Result {:.08} should be {:.08}",
                        tc.name, result, tc.output
                    );
                }
            }
        }
    }
}

#[test]
fn ExampleAutoCorrelation() {
    let s1 = Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let a = auto_correlation(s1.clone(), 1).unwrap();
    assert_eq!(a, 0.4);
    // Output: 0.4
}

#[test]
fn TestAutoCorrelation() {
    let s1 = Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let s2 = Float64Data::from(vec![]);

    let a = auto_correlation(s1.clone(), 1);
    match a {
        Err(_) => panic!("Should not have returned an error"),
        Ok(result) => {
            if result != 0.4 {
                panic!("Should have returned 0.4");
            }
        }
    }

    let result = auto_correlation(s2.clone(), 1);
    match result {
        Err(e) => {
            assert_eq!(
                e.to_string(),
                ERR_EMPTY_INPUT.to_string(),
                "Should have returned empty input error"
            );
        }
        Ok(_) => panic!("Should have returned empty input error"),
    }
}
