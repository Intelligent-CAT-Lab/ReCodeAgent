use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::StatsError;
use crate::data::Float64Data;
use crate::legacy::EMPTY_INPUT;
#[cfg(not(feature = "mock"))]
pub fn sigmoid(input: Float64Data) -> Result<Vec<f64>, anyhow::Error> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT.err.clone()));
    }
    let mut output = Vec::with_capacity(input.len());
    for v in input.0 {
        output.push(1.0 / (1.0 + (-v).exp()));
    }
    Ok(output)
}
#[cfg(feature = "mock")]
pub fn sigmoid(input: Float64Data) -> Result<Vec<f64>, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_sigmoid__ground_truth"]
        fn sigmoid__foreign(_: JSONObject) -> JSONObject;
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
        #[serde_as(as = "Vec < crate :: interoperation_utils :: MyFloat64 >")]
        Vec<f64>,
    );
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(sigmoid__foreign(ser(&input_state_in)))
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
pub fn sigmoid__with_callees_mocked(
    input: Float64Data,
) -> Result<Vec<f64>, anyhow::Error> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT.err.clone()));
    }
    let mut output = Vec::with_capacity(input.len());
    for v in input.0 {
        output.push(1.0 / (1.0 + (-v).exp()));
    }
    Ok(output)
}

