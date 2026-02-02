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
#[cfg(test)]
mod stats_median_harness {
    use super::*;
    #[test]
    fn median__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.Median.json",
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
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
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
                    { (median__with_callees_mocked(input_state.0)).unwrap() }
                    #[cfg(not(feature = "mock"))] { (median(input_state.0)).unwrap() }
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
    fn median__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Float64Data);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.Median.json",
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
