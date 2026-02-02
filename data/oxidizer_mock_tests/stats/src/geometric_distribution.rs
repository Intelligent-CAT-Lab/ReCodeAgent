use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::ERR_NEGATIVE;
use crate::errors::StatsError;
use std::f64;
#[cfg(not(feature = "mock"))]
pub fn exp_geom(p: f64) -> Result<f64, anyhow::Error> {
    if p > 1.0 || p < 0.0 {
        return Err(anyhow!("{}", ERR_NEGATIVE.err));
    }
    Ok(1.0 / p)
}
#[cfg(feature = "mock")]
pub fn exp_geom(p: f64) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_exp_geom__ground_truth"]
        fn exp_geom__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(p);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(exp_geom__foreign(ser(&input_state_in)))
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
pub fn exp_geom__with_callees_mocked(p: f64) -> Result<f64, anyhow::Error> {
    if p > 1.0 || p < 0.0 {
        return Err(anyhow!("{}", ERR_NEGATIVE.err));
    }
    Ok(1.0 / p)
}
#[cfg(test)]
mod stats_exp_geom_harness {
    use super::*;
    #[test]
    fn exp_geom__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.ExpGeom.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
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
                    { (exp_geom__with_callees_mocked(input_state.0)).unwrap() }
                    #[cfg(not(feature = "mock"))] { (exp_geom(input_state.0)).unwrap() }
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
    fn exp_geom__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.ExpGeom.json",
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
use crate::errors::ERR_BOUNDS;
use std::f64::NAN;
#[cfg(not(feature = "mock"))]
pub fn prob_geom(a: i32, b: i32, p: f64) -> Result<f64> {
    mock::mock_body!(
        { if a > b || a < 1 { return Err(anyhow::Error::new(ERR_BOUNDS.clone())); } let
        mut prob = 0.0; let q = 1.0 - p; for k in (a + 1)..= (b) { prob += p * q.powi(k
        as i32 - 1); } Ok(prob) }
    );
}
#[cfg(feature = "mock")]
pub fn prob_geom(a: i32, b: i32, p: f64) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_prob_geom__ground_truth"]
        fn prob_geom__foreign(_: JSONObject, _: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        i32,
        i32,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        i32,
        i32,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(a, b, p);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(prob_geom__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn prob_geom__with_callees_mocked(a: i32, b: i32, p: f64) -> Result<f64> {
    mock::mock_body!(
        { if a > b || a < 1 { return Err(anyhow::Error::new(ERR_BOUNDS.clone())); } let
        mut prob = 0.0; let q = 1.0 - p; for k in (a + 1)..= (b) { prob += p * q.powi(k
        as i32 - 1); } Ok(prob) }
    );
}
#[cfg(test)]
mod stats_prob_geom_harness {
    use super::*;
    #[test]
    fn prob_geom__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.ProbGeom.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            i32,
            i32,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
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
                    {
                        (prob_geom__with_callees_mocked(
                            input_state.0,
                            input_state.1,
                            input_state.2,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (prob_geom(input_state.0, input_state.1, input_state.2)).unwrap() }
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
    fn prob_geom__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            i32,
            i32,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.ProbGeom.json",
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
use std::f64::consts::SQRT_2;
#[cfg(not(feature = "mock"))]
pub fn var_geom(p: f64) -> Result<f64, anyhow::Error> {
    if p > 1.0 || p < 0.0 {
        return Err(anyhow::Error::msg("p must be between 0 and 1"));
    }
    let exp = (1.0 - p) / (p * p);
    Ok(exp)
}
#[cfg(feature = "mock")]
pub fn var_geom(p: f64) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_var_geom__ground_truth"]
        fn var_geom__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(p);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(var_geom__foreign(ser(&input_state_in)))
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
pub fn var_geom__with_callees_mocked(p: f64) -> Result<f64, anyhow::Error> {
    if p > 1.0 || p < 0.0 {
        return Err(anyhow::Error::msg("p must be between 0 and 1"));
    }
    let exp = (1.0 - p) / (p * p);
    Ok(exp)
}
#[cfg(test)]
mod stats_var_geom_harness {
    use super::*;
    #[test]
    fn var_geom__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.VarGeom.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
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
                    { (var_geom__with_callees_mocked(input_state.0)).unwrap() }
                    #[cfg(not(feature = "mock"))] { (var_geom(input_state.0)).unwrap() }
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
    fn var_geom__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.VarGeom.json",
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
