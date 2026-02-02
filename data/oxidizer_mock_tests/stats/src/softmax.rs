use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::legacy::EMPTY_INPUT;
use std::f64::consts::E;
use crate::errors::StatsError;
use anyhow::bail;
use crate::max::max;
use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub fn soft_max(input: Float64Data) -> Result<Vec<f64>> {
    if input.len() == 0 {
        bail!(EMPTY_INPUT.err.clone());
    }
    let c = max(input.clone())?.to_owned();
    let s: f64 = input.0.iter().map(|x| E.powf(x - c)).sum();
    let sm: Vec<f64> = input.0.iter().map(|x| E.powf(x - c) / s).collect();
    Ok(sm)
}
#[cfg(feature = "mock")]
pub fn soft_max(input: Float64Data) -> Result<Vec<f64>> {
    extern "C" {
        #[link_name = "stats_soft_max__ground_truth"]
        fn soft_max__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(soft_max__foreign(ser(&input_state_in)))
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
pub fn soft_max__with_callees_mocked(input: Float64Data) -> Result<Vec<f64>> {
    if input.len() == 0 {
        bail!(EMPTY_INPUT.err.clone());
    }
    let c = max(input.clone())?.to_owned();
    let s: f64 = input.0.iter().map(|x| E.powf(x - c)).sum();
    let sm: Vec<f64> = input.0.iter().map(|x| E.powf(x - c) / s).collect();
    Ok(sm)
}
#[cfg(test)]
mod stats_soft_max_harness {
    use super::*;
    #[test]
    fn soft_max__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.SoftMax.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Float64Data);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "Vec < crate :: interoperation_utils :: MyFloat64 >")]
            Vec<f64>,
        );
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (soft_max__with_callees_mocked(input_state.0)).unwrap() }
                    #[cfg(not(feature = "mock"))] { (soft_max(input_state.0)).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn soft_max__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Float64Data);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "Vec < crate :: interoperation_utils :: MyFloat64 >")]
            Vec<f64>,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.SoftMax.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}
