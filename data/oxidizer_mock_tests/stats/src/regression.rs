use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;


#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub struct Coordinate {
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "X")]
    pub x: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Y")]
    pub y: f64,
}
#[cfg(test)]
mod stats_Coordinate_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "stats_Coordinate_roundtrip"]
        fn Coordinate__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn Coordinate__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!(
                                "(*{}).", "github.com-montanaflynn-stats.Coordinate"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-montanaflynn-stats.Coordinate"
                                ),
                            )
                    {
                        let unittests_file: std::fs::File = std::fs::File::open(
                                entry.path(),
                            )
                            .unwrap();
                        let unittests_reader = std::io::BufReader::new(unittests_file);
                        let unittests: Vec<ExecutionData> = serde_json::from_reader(
                                unittests_reader,
                            )
                            .unwrap();
                        for unittest in unittests {
                            let obj = unittest.inputs[0].clone();
                            if obj == serde_json::Value::Null {
                                continue;
                            }
                            let obj_once = serde_json::to_value(
                                    serde_json::from_value::<Coordinate>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<Coordinate>(obj_once.clone())
                                        .unwrap(),
                                )
                                .unwrap();
                            assert_json_diff::assert_json_eq!(obj_once, obj_twice);
                        }
                    }
                }
            }
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(derive_more::From, derive_more::Into)]
#[derive(Default)]
#[derive(Clone)]
pub struct Series(#[serde_as(as = "serde_with::DefaultOnNull")] pub Vec<Coordinate>);
use crate::errors::StatsError;
use crate::legacy::Y_COORD_ERR;
use crate::legacy::EMPTY_INPUT_ERR;
#[cfg(not(feature = "mock"))]
pub fn exponential_regression(s: &Series) -> Result<Vec<Coordinate>, anyhow::Error> {
    if s.0.is_empty() {
        return Err(EMPTY_INPUT_ERR.to_owned().into());
    }
    let mut sum = [0.0; 6];
    for coord in &s.0 {
        if coord.y < 0.0 {
            return Err(Y_COORD_ERR.to_owned().into());
        }
        sum[0] += coord.x;
        sum[1] += coord.y;
        sum[2] += coord.x * coord.x * coord.y;
        sum[3] += coord.y * coord.y.ln();
        sum[4] += coord.x * coord.y * coord.y.ln();
        sum[5] += coord.x * coord.y;
    }
    let denominator = sum[1] * sum[2] - sum[5] * sum[5];
    let a = f64::exp((sum[2] * sum[3] - sum[5] * sum[4]) / denominator);
    let b = (sum[1] * sum[4] - sum[5] * sum[3]) / denominator;
    let mut regressions = Vec::with_capacity(s.0.len());
    for coord in &s.0 {
        regressions
            .push(Coordinate {
                x: coord.x,
                y: a * f64::exp(b * coord.x),
            });
    }
    Ok(regressions)
}
#[cfg(feature = "mock")]
pub fn exponential_regression(s: &Series) -> Result<Vec<Coordinate>, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_exponential_regression__ground_truth"]
        fn exponential_regression__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Series);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Series>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Coordinate>);
    let input_state_in = InputStateIn(s);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(exponential_regression__foreign(ser(&input_state_in)))
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
pub fn exponential_regression__with_callees_mocked(
    s: &Series,
) -> Result<Vec<Coordinate>, anyhow::Error> {
    if s.0.is_empty() {
        return Err(EMPTY_INPUT_ERR.to_owned().into());
    }
    let mut sum = [0.0; 6];
    for coord in &s.0 {
        if coord.y < 0.0 {
            return Err(Y_COORD_ERR.to_owned().into());
        }
        sum[0] += coord.x;
        sum[1] += coord.y;
        sum[2] += coord.x * coord.x * coord.y;
        sum[3] += coord.y * coord.y.ln();
        sum[4] += coord.x * coord.y * coord.y.ln();
        sum[5] += coord.x * coord.y;
    }
    let denominator = sum[1] * sum[2] - sum[5] * sum[5];
    let a = f64::exp((sum[2] * sum[3] - sum[5] * sum[4]) / denominator);
    let b = (sum[1] * sum[4] - sum[5] * sum[3]) / denominator;
    let mut regressions = Vec::with_capacity(s.0.len());
    for coord in &s.0 {
        regressions
            .push(Coordinate {
                x: coord.x,
                y: a * f64::exp(b * coord.x),
            });
    }
    Ok(regressions)
}
#[cfg(test)]
mod stats_exponential_regression_harness {
    use super::*;
    #[test]
    fn exponential_regression__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.ExponentialRegression.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Series>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "serde_with::DefaultOnNull")]
            Vec<Coordinate>,
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
                        (exponential_regression__with_callees_mocked(&*input_state.0))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (exponential_regression(&*input_state.0)).unwrap() }
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
    fn exponential_regression__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Series>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "serde_with::DefaultOnNull")]
            Vec<Coordinate>,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.ExponentialRegression.json",
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
use std::iter::Sum;
#[cfg(not(feature = "mock"))]
pub fn linear_regression(series: &Series) -> Result<Series, Error> {
    if series.0.is_empty() {
        return Err(anyhow::Error::msg(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut sum = [0.0; 5];
    let mut i = 0;
    for coord in &series.0 {
        sum[0] += coord.x;
        sum[1] += coord.y;
        sum[2] += coord.x * coord.x;
        sum[3] += coord.x * coord.y;
        sum[4] += coord.y * coord.y;
        i += 1;
    }
    let f = i as f64;
    let gradient = (f * sum[3] - sum[0] * sum[1]) / (f * sum[2] - sum[0] * sum[0]);
    let intercept = (sum[1] / f) - (gradient * sum[0] / f);
    let regressions = series
        .0
        .iter()
        .map(|coord| Coordinate {
            x: coord.x,
            y: coord.x * gradient + intercept,
        })
        .collect();
    Ok(Series(regressions))
}
#[cfg(feature = "mock")]
pub fn linear_regression(series: &Series) -> Result<Series, Error> {
    extern "C" {
        #[link_name = "stats_linear_regression__ground_truth"]
        fn linear_regression__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Series);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Series>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Series);
    let input_state_in = InputStateIn(series);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(linear_regression__foreign(ser(&input_state_in)))
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
pub fn linear_regression__with_callees_mocked(series: &Series) -> Result<Series, Error> {
    if series.0.is_empty() {
        return Err(anyhow::Error::msg(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut sum = [0.0; 5];
    let mut i = 0;
    for coord in &series.0 {
        sum[0] += coord.x;
        sum[1] += coord.y;
        sum[2] += coord.x * coord.x;
        sum[3] += coord.x * coord.y;
        sum[4] += coord.y * coord.y;
        i += 1;
    }
    let f = i as f64;
    let gradient = (f * sum[3] - sum[0] * sum[1]) / (f * sum[2] - sum[0] * sum[0]);
    let intercept = (sum[1] / f) - (gradient * sum[0] / f);
    let regressions = series
        .0
        .iter()
        .map(|coord| Coordinate {
            x: coord.x,
            y: coord.x * gradient + intercept,
        })
        .collect();
    Ok(Series(regressions))
}
#[cfg(test)]
mod stats_linear_regression_harness {
    use super::*;
    #[test]
    fn linear_regression__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.LinearRegression.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Series>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(Series);
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
                        (linear_regression__with_callees_mocked(&*input_state.0))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (linear_regression(&*input_state.0)).unwrap() }
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
    fn linear_regression__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Series>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Series);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.LinearRegression.json",
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

#[cfg(not(feature = "mock"))]
pub fn logarithmic_regression(s: Series) -> Result<Series, Error> {
    if s.0.is_empty() {
        return Err(Error::from(EMPTY_INPUT_ERR.clone()));
    }
    let mut sum = [0.0; 4];
    for coord in &s.0 {
        sum[0] += coord.x.ln();
        sum[1] += coord.y * coord.x.ln();
        sum[2] += coord.y;
        sum[3] += (coord.x.ln()).powi(2);
    }
    let f = s.0.len() as f64;
    let a = (f * sum[1] - sum[2] * sum[0]) / (f * sum[3] - sum[0] * sum[0]);
    let b = (sum[2] - a * sum[0]) / f;
    let regressions = s
        .0
        .iter()
        .map(|coord| Coordinate {
            x: coord.x,
            y: b + a * coord.x.ln(),
        })
        .collect();
    Ok(Series(regressions))
}
#[cfg(feature = "mock")]
pub fn logarithmic_regression(s: Series) -> Result<Series, Error> {
    extern "C" {
        #[link_name = "stats_logarithmic_regression__ground_truth"]
        fn logarithmic_regression__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Series);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Series);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Series);
    let input_state_in = InputStateIn(s);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(logarithmic_regression__foreign(ser(&input_state_in)))
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
pub fn logarithmic_regression__with_callees_mocked(s: Series) -> Result<Series, Error> {
    if s.0.is_empty() {
        return Err(Error::from(EMPTY_INPUT_ERR.clone()));
    }
    let mut sum = [0.0; 4];
    for coord in &s.0 {
        sum[0] += coord.x.ln();
        sum[1] += coord.y * coord.x.ln();
        sum[2] += coord.y;
        sum[3] += (coord.x.ln()).powi(2);
    }
    let f = s.0.len() as f64;
    let a = (f * sum[1] - sum[2] * sum[0]) / (f * sum[3] - sum[0] * sum[0]);
    let b = (sum[2] - a * sum[0]) / f;
    let regressions = s
        .0
        .iter()
        .map(|coord| Coordinate {
            x: coord.x,
            y: b + a * coord.x.ln(),
        })
        .collect();
    Ok(Series(regressions))
}
#[cfg(test)]
mod stats_logarithmic_regression_harness {
    use super::*;
    #[test]
    fn logarithmic_regression__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.LogarithmicRegression.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Series);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(Series);
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
                        (logarithmic_regression__with_callees_mocked(input_state.0))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (logarithmic_regression(input_state.0)).unwrap() }
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
    fn logarithmic_regression__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Series);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Series);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.LogarithmicRegression.json",
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
