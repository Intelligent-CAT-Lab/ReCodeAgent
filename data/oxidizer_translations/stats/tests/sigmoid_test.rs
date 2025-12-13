use wspace::sigmoid::sigmoid;
use wspace::data::Float64Data;
use wspace::errors::ERR_EMPTY_INPUT;

#[test]
fn ExampleSigmoid() {
    let s = sigmoid(Float64Data::from(vec![3.0, 1.0, 2.1])).unwrap();
    println!("{:?}", s);
    // Output: [0.9525741268224334, 0.7310585786300049, 0.8909031788043871]

    // Verify the expected output
    assert_eq!(s[0], 0.9525741268224334);
    assert_eq!(s[1], 0.7310585786300049);
    assert_eq!(s[2], 0.8909031788043871);
}

#[test]
fn TestSigmoidEmptyInput() {
    let result = sigmoid(Float64Data::from(vec![]));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), ERR_EMPTY_INPUT.to_string());
}

#[test]
fn TestSigmoid() {
    let sm = sigmoid(Float64Data::from(vec![-0.54761371, 17.04850603, 4.86054302])).unwrap();

    let a = 0.3664182235138545;
    assert_eq!(sm[0], a, "{} != {}", sm[0], a);

    let a = 0.9999999605608187;
    assert_eq!(sm[1], a, "{} != {}", sm[1], a);

    let a = 0.9923132671908277;
    assert_eq!(sm[2], a, "{} != {}", sm[2], a);
}
