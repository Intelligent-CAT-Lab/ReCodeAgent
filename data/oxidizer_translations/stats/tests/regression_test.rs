use wspace::regression::{Coordinate, Series, exponential_regression, linear_regression, logarithmic_regression};
use wspace::round::round;
use wspace::errors::ERR_Y_COORD;

// Test helper functions for floating point comparison
// Taken from the standard library's math/all_test.go
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

fn close(a: f64, b: f64) -> bool {
    tolerance(a, b, 1e-14)
}

fn veryclose(a: f64, b: f64) -> bool {
    tolerance(a, b, 4e-16)
}

#[test]
fn test_example_linear_regression() {
    let data = vec![
        Coordinate { x: 1.0, y: 2.3 },
        Coordinate { x: 2.0, y: 3.3 },
        Coordinate { x: 3.0, y: 3.7 },
    ];

    let r = linear_regression(&Series::from(data)).unwrap();
    assert!(close(r.0[0].y, 2.400000000000001));
    assert!(close(r.0[1].y, 3.1));
    assert!(close(r.0[2].y, 3.7999999999999994));
}

#[test]
fn test_linear_regression() {
    let data = vec![
        Coordinate { x: 1.0, y: 2.3 },
        Coordinate { x: 2.0, y: 3.3 },
        Coordinate { x: 3.0, y: 3.7 },
        Coordinate { x: 4.0, y: 4.3 },
        Coordinate { x: 5.0, y: 5.3 },
    ];

    let r = linear_regression(&Series::from(data)).unwrap();
    let mut a = 2.3800000000000026;
    assert!(
        close(r.0[0].y, a),
        "{} != {}",
        r.0[0].y,
        a
    );
    a = 3.0800000000000014;
    assert!(
        veryclose(r.0[1].y, a),
        "{} != {}",
        r.0[1].y,
        a
    );
    a = 3.7800000000000002;
    assert_eq!(r.0[2].y, a, "{} != {}", r.0[2].y, a);
    a = 4.479999999999999;
    assert!(
        veryclose(r.0[3].y, a),
        "{} != {}",
        r.0[3].y,
        a
    );
    a = 5.179999999999998;
    assert!(
        veryclose(r.0[4].y, a),
        "{} != {}",
        r.0[4].y,
        a
    );

    let err = linear_regression(&Series::from(vec![]));
    assert!(err.is_err(), "Empty slice should have returned an error");
}

#[test]
fn test_exponential_regression() {
    let data = vec![
        Coordinate { x: 1.0, y: 2.3 },
        Coordinate { x: 2.0, y: 3.3 },
        Coordinate { x: 3.0, y: 3.7 },
        Coordinate { x: 4.0, y: 4.3 },
        Coordinate { x: 5.0, y: 5.3 },
    ];

    let r = exponential_regression(&Series::from(data)).unwrap();
    let mut a = round(r[0].y, 3).unwrap();
    assert_eq!(a, 2.515, "{} != {}", r[0].y, 2.515);
    a = round(r[1].y, 3).unwrap();
    assert_eq!(a, 3.032, "{} != {}", r[1].y, 3.032);
    a = round(r[2].y, 3).unwrap();
    assert_eq!(a, 3.655, "{} != {}", r[2].y, 3.655);
    a = round(r[3].y, 3).unwrap();
    assert_eq!(a, 4.407, "{} != {}", r[3].y, 4.407);
    a = round(r[4].y, 3).unwrap();
    assert_eq!(a, 5.313, "{} != {}", r[4].y, 5.313);

    let err = exponential_regression(&Series::from(vec![]));
    assert!(err.is_err(), "Empty slice should have returned an error");
}

#[test]
fn test_exponential_regression_y_coord_err() {
    let c = vec![
        Coordinate { x: 1.0, y: -5.0 },
        Coordinate { x: 4.0, y: 25.0 },
        Coordinate { x: 6.0, y: 5.0 },
    ];
    let err = exponential_regression(&Series::from(c));
    assert!(err.is_err());
    let err_result = err.unwrap_err();
    assert_eq!(err_result.to_string(), ERR_Y_COORD.to_string());
}

#[test]
fn test_logarithmic_regression() {
    let data = vec![
        Coordinate { x: 1.0, y: 2.3 },
        Coordinate { x: 2.0, y: 3.3 },
        Coordinate { x: 3.0, y: 3.7 },
        Coordinate { x: 4.0, y: 4.3 },
        Coordinate { x: 5.0, y: 5.3 },
    ];

    let r = logarithmic_regression(Series::from(data)).unwrap();
    let mut a = 2.1520822363811702;
    assert!(
        close(r.0[0].y, a),
        "{} != {}",
        r.0[0].y,
        a
    );
    a = 3.3305559222492214;
    assert!(
        veryclose(r.0[1].y, a),
        "{} != {}",
        r.0[1].y,
        a
    );
    a = 4.019918836568674;
    assert!(
        close(r.0[2].y, a),
        "{} != {}",
        r.0[2].y,
        a
    );
    a = 4.509029608117273;
    assert!(
        close(r.0[3].y, a),
        "{} != {}",
        r.0[3].y,
        a
    );
    a = 4.888413396683663;
    assert!(
        close(r.0[4].y, a),
        "{} != {}",
        r.0[4].y,
        a
    );

    let err = logarithmic_regression(Series::from(vec![]));
    assert!(err.is_err(), "Empty slice should have returned an error");
}
