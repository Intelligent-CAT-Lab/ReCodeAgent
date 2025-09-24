use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::StatsError;
use crate::data::Float64Data;
use crate::legacy::EMPTY_INPUT;
#[cfg(not(feature = "mock"))]
pub fn cumulative_sum(input: &Float64Data) -> Result<Float64Data> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT.err.clone()));
    }
    let mut cum_sum = Vec::with_capacity(input.len());
    let mut sum = 0.0;
    for val in &input.0 {
        sum += val;
        cum_sum.push(sum);
    }
    Ok(Float64Data(cum_sum))
}
#[cfg(feature = "mock")]
pub fn cumulative_sum(input: &Float64Data) -> Result<Float64Data> {
    extern "C" {
        #[link_name = "stats_cumulative_sum__ground_truth"]
        fn cumulative_sum__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Float64Data>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Float64Data);
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(cumulative_sum__foreign(ser(&input_state_in)))
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
pub fn cumulative_sum__with_callees_mocked(input: &Float64Data) -> Result<Float64Data> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT.err.clone()));
    }
    let mut cum_sum = Vec::with_capacity(input.len());
    let mut sum = 0.0;
    for val in &input.0 {
        sum += val;
        cum_sum.push(sum);
    }
    Ok(Float64Data(cum_sum))
}

