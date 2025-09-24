use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub(crate) fn normalize(input: Float64Data) -> Result<Float64Data> {
    let sum = input.sum()?;
    let normalized = Float64Data(input.0.into_iter().map(|x| x / sum).collect());
    Ok(normalized)
}
#[cfg(feature = "mock")]
pub(crate) fn normalize(input: Float64Data) -> Result<Float64Data> {
    extern "C" {
        #[link_name = "stats_normalize__ground_truth"]
        fn normalize__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Float64Data);
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(normalize__foreign(ser(&input_state_in)))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
pub(crate) fn normalize__with_callees_mocked(input: Float64Data) -> Result<Float64Data> {
    let sum = input.sum()?;
    let normalized = Float64Data(input.0.into_iter().map(|x| x / sum).collect());
    Ok(normalized)
}

use std::f64::consts::LN_2;
#[cfg(not(feature = "mock"))]
pub fn entropy(input: Float64Data) -> Result<f64> {
    let input = normalize(input)?;
    let mut result = 0.0;
    for i in 0..input.len() {
        let v = input.get(i);
        if v != 0.0 {
            result += v * v.ln();
        }
    }
    Ok(-result)
}
#[cfg(feature = "mock")]
pub fn entropy(input: Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_entropy__ground_truth"]
        fn entropy__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(entropy__foreign(ser(&input_state_in)))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
pub fn entropy__with_callees_mocked(input: Float64Data) -> Result<f64> {
    let input = normalize(input)?;
    let mut result = 0.0;
    for i in 0..input.len() {
        let v = input.get(i);
        if v != 0.0 {
            result += v * v.ln();
        }
    }
    Ok(-result)
}

