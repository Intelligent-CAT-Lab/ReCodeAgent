use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashSet;
#[cfg(not(feature = "mock"))]
pub fn shingle_slice(s: &str, k: usize) -> Result<Vec<String>, Error> {
    let mut out = Vec::new();
    let mut shingles = HashSet::new();
    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() - k + 1 {
            let shingle: String = chars[i..i + k].iter().collect();
            shingles.insert(shingle);
        }
        out = shingles.into_iter().collect();
    }
    Ok(out)
}
#[cfg(feature = "mock")]
pub fn shingle_slice(s: &str, k: usize) -> Result<Vec<String>, Error> {
    extern "C" {
        #[link_name = "go_edlib_shingle_slice__ground_truth"]
        fn shingle_slice__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a str, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
    let input_state_in = InputStateIn(s, k);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(shingle_slice__foreign(ser(&params[0]), ser(&params[1])))
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
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn shingle_slice__with_callees_mocked(
    s: &str,
    k: usize,
) -> Result<Vec<String>, Error> {
    let mut out = Vec::new();
    let mut shingles = HashSet::new();
    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() - k + 1 {
            let shingle: String = chars[i..i + k].iter().collect();
            shingles.insert(shingle);
        }
        out = shingles.into_iter().collect();
    }
    Ok(out)
}

use std::collections::HashMap;
#[cfg(not(feature = "mock"))]
pub fn shingle(s: &str, k: usize) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() - k + 1 {
            let shingle: String = chars[i..i + k].iter().collect();
            *m.entry(shingle).or_insert(0) += 1;
        }
    }
    m
}
#[cfg(feature = "mock")]
pub fn shingle(s: &str, k: usize) -> HashMap<String, usize> {
    extern "C" {
        #[link_name = "go_edlib_shingle__ground_truth"]
        fn shingle__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a str, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(HashMap<String, usize>);
    let input_state_in = InputStateIn(s, k);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(shingle__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn shingle__with_callees_mocked(s: &str, k: usize) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() - k + 1 {
            let shingle: String = chars[i..i + k].iter().collect();
            *m.entry(shingle).or_insert(0) += 1;
        }
    }
    m
}

