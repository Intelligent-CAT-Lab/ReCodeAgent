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

