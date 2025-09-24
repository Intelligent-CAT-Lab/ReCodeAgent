use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub fn sum(input: Float64Data) -> Result<f64, Error> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut sum = 0.0;
    for n in input.0 {
        sum += n;
    }
    Ok(sum)
}
#[cfg(feature = "mock")]
pub fn sum(input: Float64Data) -> Result<f64, Error> {
    extern "C" {
        #[link_name = "stats_sum__ground_truth"]
        fn sum__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(sum__foreign(ser(&input_state_in)))
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
pub fn sum__with_callees_mocked(input: Float64Data) -> Result<f64, Error> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut sum = 0.0;
    for n in input.0 {
        sum += n;
    }
    Ok(sum)
}

