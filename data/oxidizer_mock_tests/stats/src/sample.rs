use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::legacy::EMPTY_INPUT_ERR;
use crate::legacy::BOUNDS_ERR;
use crate::errors::StatsError;
use crate::data::Float64Data;
use rand::prelude::*;
use crate::util::unixnano;
use rand::seq::SliceRandom;
use rand::thread_rng;
#[cfg(not(feature = "mock"))]
pub fn sample(
    input: &Float64Data,
    take_num: usize,
    replacement: bool,
) -> Result<Float64Data> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let length = input.len();
    if replacement {
        let mut result = Float64Data::default();
        let mut rng = thread_rng();
        for _ in 0..take_num {
            let idx = rng.gen_range(0..length);
            result.0.push(input.0[idx]);
        }
        Ok(result)
    } else if !replacement && take_num <= length {
        let mut rng = thread_rng();
        let mut permutation = (0..length).collect::<Vec<_>>();
        permutation.shuffle(&mut rng);
        let result = Float64Data(
            permutation[..take_num].iter().map(|&idx| input.0[idx]).collect(),
        );
        Ok(result)
    } else {
        Err(BOUNDS_ERR.clone().into())
    }
}
#[cfg(feature = "mock")]
pub fn sample(
    input: &Float64Data,
    take_num: usize,
    replacement: bool,
) -> Result<Float64Data> {
    extern "C" {
        #[link_name = "stats_sample__ground_truth"]
        fn sample__foreign(_: JSONObject, _: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Float64Data, usize, bool);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Float64Data>, usize, bool);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Float64Data);
    let input_state_in = InputStateIn(input, take_num, replacement);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(sample__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn sample__with_callees_mocked(
    input: &Float64Data,
    take_num: usize,
    replacement: bool,
) -> Result<Float64Data> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let length = input.len();
    if replacement {
        let mut result = Float64Data::default();
        let mut rng = thread_rng();
        for _ in 0..take_num {
            let idx = rng.gen_range(0..length);
            result.0.push(input.0[idx]);
        }
        Ok(result)
    } else if !replacement && take_num <= length {
        let mut rng = thread_rng();
        let mut permutation = (0..length).collect::<Vec<_>>();
        permutation.shuffle(&mut rng);
        let result = Float64Data(
            permutation[..take_num].iter().map(|&idx| input.0[idx]).collect(),
        );
        Ok(result)
    } else {
        Err(BOUNDS_ERR.clone().into())
    }
}
#[cfg(test)]
mod stats_sample_harness {
    use super::*;
    #[test]
    fn sample__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.Sample.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Float64Data>, usize, bool);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(Float64Data);
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
                    {
                        (sample__with_callees_mocked(
                            &*input_state.0,
                            input_state.1,
                            input_state.2,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (sample(&*input_state.0, input_state.1, input_state.2)).unwrap() }
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
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn sample__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Float64Data>, usize, bool);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Float64Data);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.Sample.json",
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
