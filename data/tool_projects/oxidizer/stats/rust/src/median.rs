use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::mean::mean;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::util::sorted_copy;
use crate::errors::StatsError;
use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub fn median(input: Float64Data) -> Result<f64, anyhow::Error> {
    let c = sorted_copy(input);
    let l = c.0.len();
    if l == 0 {
        return Err(anyhow!("{}", EMPTY_INPUT_ERR.err));
    } else if l % 2 == 0 {
        let mean_input = Float64Data(c.0[l / 2 - 1..l / 2 + 1].to_vec());
        let median = mean(mean_input)?;
        Ok(median)
    } else {
        Ok(c.0[l / 2])
    }
}
#[cfg(feature = "mock")]
pub fn median(input: Float64Data) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_median__ground_truth"]
        fn median__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(median__foreign(ser(&input_state_in)))
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
pub fn median__with_callees_mocked(input: Float64Data) -> Result<f64, anyhow::Error> {
    let c = sorted_copy(input);
    let l = c.0.len();
    if l == 0 {
        return Err(anyhow!("{}", EMPTY_INPUT_ERR.err));
    } else if l % 2 == 0 {
        let mean_input = Float64Data(c.0[l / 2 - 1..l / 2 + 1].to_vec());
        let median = mean(mean_input)?;
        Ok(median)
    } else {
        Ok(c.0[l / 2])
    }
}

