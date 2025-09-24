use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashMap;
use crate::shingle::shingle;
#[cfg(not(feature = "mock"))]
pub fn sorensen_dice_coefficient(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32> {
    if str1.is_empty() && str2.is_empty() {
        return Ok(0.0);
    }
    let shingle1 = shingle(str1, split_length);
    let shingle2 = shingle(str2, split_length);
    let mut intersection = 0;
    for (shingle, _) in &shingle1 {
        if shingle2.contains_key(shingle) {
            intersection += 1;
        }
    }
    let total_shingles = shingle1.len() + shingle2.len();
    let coefficient = 2.0 * intersection as f32 / total_shingles as f32;
    Ok(coefficient)
}
#[cfg(feature = "mock")]
pub fn sorensen_dice_coefficient(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32> {
    extern "C" {
        #[link_name = "go_edlib_sorensen_dice_coefficient__ground_truth"]
        fn sorensen_dice_coefficient__foreign(
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
        >(
            sorensen_dice_coefficient__foreign(
                ser(&params[0]),
                ser(&params[1]),
                ser(&params[2]),
            ),
        )
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
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn sorensen_dice_coefficient__with_callees_mocked(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32> {
    if str1.is_empty() && str2.is_empty() {
        return Ok(0.0);
    }
    let shingle1 = shingle(str1, split_length);
    let shingle2 = shingle(str2, split_length);
    let mut intersection = 0;
    for (shingle, _) in &shingle1 {
        if shingle2.contains_key(shingle) {
            intersection += 1;
        }
    }
    let total_shingles = shingle1.len() + shingle2.len();
    let coefficient = 2.0 * intersection as f32 / total_shingles as f32;
    Ok(coefficient)
}

