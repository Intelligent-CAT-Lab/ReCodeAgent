use wspace::softmax::soft_max;
use wspace::errors::ERR_EMPTY_INPUT;
use wspace::data::Float64Data;

fn tolerance(a: f64, b: f64, e: f64) -> bool {
    (a - b).abs() < e
}

fn close(a: f64, b: f64) -> bool {
    tolerance(a, b, 1e-14)
}

#[test]
fn example_soft_max() {
    let sm = soft_max(Float64Data::from(vec![3.0, 1.0, 0.2])).unwrap();
    println!("{:?}", sm);
    // Output: [0.8360188027814407, 0.11314284146556013, 0.05083835575299916]
}

#[test]
fn test_soft_max_empty_input() {
    let result = soft_max(Float64Data::from(vec![]));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), ERR_EMPTY_INPUT.to_string());
}

#[test]
fn test_soft_max() {
    let sm = soft_max(Float64Data::from(vec![3.0, 1.0, 0.2])).unwrap();

    let a = 0.8360188027814407;
    assert!(close(sm[0], a), "{} != {}", sm[0], a);

    let a = 0.11314284146556013;
    assert!(close(sm[1], a), "{} != {}", sm[1], a);

    let a = 0.05083835575299916;
    assert!(close(sm[2], a), "{} != {}", sm[2], a);
}
