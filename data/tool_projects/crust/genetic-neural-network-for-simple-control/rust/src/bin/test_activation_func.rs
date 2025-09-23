
use Genetic_neural_network_for_simple_control::activation_fnc::{
    selectTangActivationFunction, 
    selectSigmActivationFunction};

fn compare_results(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}

fn test_sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}
#[test]
pub fn testActivationFncTanh() {
    println!("\x1b[1m=======TEST ACTIVAION FNC TANH STARTED=======\x1b[0m");

    let data_check1: f32 = 1.0;
    let data_check2:f32 = 23.5;
    let data_check3: f32 = 3.333;

    let mut func_ptr_tanh: Option<fn(f32) -> f32> = None;
    selectTangActivationFunction(&mut func_ptr_tanh);
    let func = func_ptr_tanh.expect("Tanh function not set");

    let flag1 = compare_results(data_check1.tanh() * 5.0, func(data_check1));
    let flag2 = compare_results(data_check2.tanh() * 5.0, func(data_check2));
    let flag3 = compare_results(data_check3.tanh() * 5.0, func(data_check3));

    selectSigmActivationFunction(&mut None); // dummy call to match C code

    assert!(flag1 && flag2 && flag3, "Test Activation Function Tanh failed");
    println!("\x1b[1m\x1b[32m=======TEST ACTIVAION FNC TANH SUCCESSFUL=======\x1b[0m");
    
}

#[test]
pub fn testActivationFncSigm()  {
    println!("\x1b[1m=======TEST ACTIVAION FNC SIGM STARTED=======\x1b[0m");

    let data_check1 = 1.0;
    let data_check2 = 23.5;
    let data_check3 = 3.333;

    let mut func_ptr_sigm: Option<fn(f32) -> f32> = None;
    selectSigmActivationFunction(&mut func_ptr_sigm);
    let func = func_ptr_sigm.expect("Sigmoid function not set");

    let flag1 = compare_results(test_sigmoid(data_check1), func(data_check1));
    let flag2 = compare_results(test_sigmoid(data_check2), func(data_check2));
    let flag3 = compare_results(test_sigmoid(data_check3), func(data_check3));

    assert!(flag1 && flag2 && flag3, "Test Activation Function Sigmoid failed");
    println!("\x1b[1m\x1b[32m=======TEST ACTIVAION FNC SIGM SUCCESSFUL=======\x1b[0m");
    
}
fn main(){}
