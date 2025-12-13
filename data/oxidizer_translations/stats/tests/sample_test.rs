use wspace::data::Float64Data;
use wspace::sample::{sample, stable_sample};
use std::collections::HashMap;

#[test]
fn test_sample() {
    let result = sample(&Float64Data::from(vec![]), 10, false);
    if result.is_ok() {
        panic!("should return an error");
    }

    let result = sample(&Float64Data::from(vec![0.1, 0.2]), 10, false);
    if result.is_ok() {
        panic!("should return an error");
    }
}

#[test]
fn test_sample_without_replacement() {
    let arr = Float64Data::from(vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    let result = sample(&arr, 5, false).unwrap();
    let mut checks: HashMap<String, bool> = HashMap::new();
    for res in result.iter() {
        let key = res.to_string();
        if checks.contains_key(&key) {
            panic!("{} already seen", res);
        }
        checks.insert(key, true);
    }
}

#[test]
fn test_sample_with_replacement() {
    let arr = Float64Data::from(vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    let numsamples = 100;
    let result = sample(&arr, numsamples, true).unwrap();
    if result.len() != numsamples {
        panic!("{} != {}", result.len(), numsamples);
    }
}

#[test]
fn test_stable_sample() {
    let result = stable_sample(&Float64Data::from(vec![]), 10);
    assert!(result.is_err(), "should return EmptyInputError when sampling an empty data");
    let result = stable_sample(&Float64Data::from(vec![1.0, 2.0]), 10);
    assert!(result.is_err(), "should return BoundsErr when sampling size exceeds the maximum element size of data");
    let arr = vec![1.0, 3.0, 2.0, -1.0, 5.0];
    let mut locations: HashMap<String, usize> = HashMap::new();
    locations.insert("1".to_string(), 0);
    locations.insert("3".to_string(), 1);
    locations.insert("2".to_string(), 2);
    locations.insert("-1".to_string(), 3);
    locations.insert("5".to_string(), 4);
    let ret = stable_sample(&Float64Data::from(arr), 3).unwrap();
    if ret.len() != 3 {
        panic!("returned wrong sample size");
    }
    for i in 1..3 {
        let curr_key = ret.0[i].to_string();
        let prev_key = ret.0[i-1].to_string();
        if locations[&curr_key] < locations[&prev_key] {
            panic!("doesn't keep order");
        }
    }
}
