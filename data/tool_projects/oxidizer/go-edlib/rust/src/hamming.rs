use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::internal::utils::utils::equal;
#[cfg(not(feature = "mock"))]
pub fn hamming_distance(str1: &str, str2: &str) -> Result<usize> {
    let mut str1_chars: Vec<char> = str1.chars().collect();
    let mut str2_chars: Vec<char> = str2.chars().collect();
    if str1_chars.len() != str2_chars.len() {
        return Err(Error::msg("Strings have unequal lengths"));
    } else if equal(&str1_chars, &str2_chars)? {
        return Ok(0);
    }
    let mut counter = 0;
    for (a, b) in str1_chars.iter().zip(str2_chars.iter()) {
        if a != b {
            counter += 1;
        }
    }
    Ok(counter)
}
#[cfg(feature = "mock")]
pub fn hamming_distance(str1: &str, str2: &str) -> Result<usize> {
    extern "C" {
        #[link_name = "go_edlib_hamming_distance__ground_truth"]
        fn hamming_distance__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(usize);
    let input_state_in = InputStateIn(str1, str2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(hamming_distance__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn hamming_distance__with_callees_mocked(str1: &str, str2: &str) -> Result<usize> {
    let mut str1_chars: Vec<char> = str1.chars().collect();
    let mut str2_chars: Vec<char> = str2.chars().collect();
    if str1_chars.len() != str2_chars.len() {
        return Err(Error::msg("Strings have unequal lengths"));
    } else if equal(&str1_chars, &str2_chars)? {
        return Ok(0);
    }
    let mut counter = 0;
    for (a, b) in str1_chars.iter().zip(str2_chars.iter()) {
        if a != b {
            counter += 1;
        }
    }
    Ok(counter)
}

