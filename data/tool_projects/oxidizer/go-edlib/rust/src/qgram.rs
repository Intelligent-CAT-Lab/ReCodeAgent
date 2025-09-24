use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashMap;
#[cfg(not(feature = "mock"))]
pub fn qgram_distance_custom_ngram(
    splitted_str1: &HashMap<String, i32>,
    splitted_str2: &HashMap<String, i32>,
) -> i32 {
    let mut union: HashMap<String, i32> = HashMap::new();
    for key in splitted_str1.keys().chain(splitted_str2.keys()) {
        union.entry(key.clone()).or_insert(0);
    }
    let mut res = 0;
    for key in union.keys() {
        let val1 = splitted_str1.get(key).unwrap_or(&0);
        let val2 = splitted_str2.get(key).unwrap_or(&0);
        res += (val1 - val2).abs();
    }
    res
}
#[cfg(feature = "mock")]
pub fn qgram_distance_custom_ngram(
    splitted_str1: &HashMap<String, i32>,
    splitted_str2: &HashMap<String, i32>,
) -> i32 {
    extern "C" {
        #[link_name = "go_edlib_qgram_distance_custom_ngram__ground_truth"]
        fn qgram_distance_custom_ngram__foreign(
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a HashMap<String, i32>, &'b HashMap<String, i32>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<HashMap<String, i32>>, Box<HashMap<String, i32>>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(splitted_str1, splitted_str2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(qgram_distance_custom_ngram__foreign(ser(&params[0]), ser(&params[1])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 2usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub fn qgram_distance_custom_ngram__with_callees_mocked(
    splitted_str1: &HashMap<String, i32>,
    splitted_str2: &HashMap<String, i32>,
) -> i32 {
    let mut union: HashMap<String, i32> = HashMap::new();
    for key in splitted_str1.keys().chain(splitted_str2.keys()) {
        union.entry(key.clone()).or_insert(0);
    }
    let mut res = 0;
    for key in union.keys() {
        let val1 = splitted_str1.get(key).unwrap_or(&0);
        let val2 = splitted_str2.get(key).unwrap_or(&0);
        res += (val1 - val2).abs();
    }
    res
}

use crate::shingle::shingle;
use std::str;
#[cfg(not(feature = "mock"))]
pub fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    let splitted_str1: HashMap<String, i32> = shingle(str1, split_length)
        .into_iter()
        .map(|(k, v)| (k, v as i32))
        .collect();
    let splitted_str2: HashMap<String, i32> = shingle(str2, split_length)
        .into_iter()
        .map(|(k, v)| (k, v as i32))
        .collect();
    let res = qgram_distance_custom_ngram(&splitted_str1, &splitted_str2) as f32;
    1.0 - (res / (splitted_str1.len() + splitted_str2.len()) as f32)
}
#[cfg(feature = "mock")]
pub fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    extern "C" {
        #[link_name = "go_edlib_qgram_similarity__ground_truth"]
        fn qgram_similarity__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    let input_state_in = InputStateIn(str1, str2, split_length);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(qgram_similarity__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 3usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub fn qgram_similarity__with_callees_mocked(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> f32 {
    let splitted_str1: HashMap<String, i32> = shingle(str1, split_length)
        .into_iter()
        .map(|(k, v)| (k, v as i32))
        .collect();
    let splitted_str2: HashMap<String, i32> = shingle(str2, split_length)
        .into_iter()
        .map(|(k, v)| (k, v as i32))
        .collect();
    let res = qgram_distance_custom_ngram(&splitted_str1, &splitted_str2) as f32;
    1.0 - (res / (splitted_str1.len() + splitted_str2.len()) as f32)
}

