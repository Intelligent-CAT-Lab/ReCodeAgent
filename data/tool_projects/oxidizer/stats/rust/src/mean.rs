use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub fn mean(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(anyhow!("Input is empty"));
    }
    let sum = input.sum()?;
    let result = sum / (input.len() as f64);
    Ok(result)
}
#[cfg(feature = "mock")]
pub fn mean(input: Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_mean__ground_truth"]
        fn mean__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(mean__foreign(ser(&input_state_in)))
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
pub fn mean__with_callees_mocked(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(anyhow!("Input is empty"));
    }
    let sum = input.sum()?;
    let result = sum / (input.len() as f64);
    Ok(result)
}

use std::f64;
#[cfg(not(feature = "mock"))]
pub fn geometric_mean(input: &Float64Data) -> Result<f64> {
    let l = input.len();
    if l == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut p = 1.0;
    for n in &input.0 {
        p *= n;
    }
    let geometric_mean = p.powf(1.0 / (l as f64));
    Ok(geometric_mean)
}
#[cfg(feature = "mock")]
pub fn geometric_mean(input: &Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_geometric_mean__ground_truth"]
        fn geometric_mean__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Float64Data>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(geometric_mean__foreign(ser(&input_state_in)))
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
pub fn geometric_mean__with_callees_mocked(input: &Float64Data) -> Result<f64> {
    let l = input.len();
    if l == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut p = 1.0;
    for n in &input.0 {
        p *= n;
    }
    let geometric_mean = p.powf(1.0 / (l as f64));
    Ok(geometric_mean)
}

use crate::legacy::ZERO_ERR;
use crate::legacy::NEGATIVE_ERR;
#[cfg(not(feature = "mock"))]
pub fn harmonic_mean(input: Float64Data) -> Result<f64> {
    let l = input.len();
    if l == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let mut p = 0.0;
    for n in input.0 {
        if n < 0.0 {
            return Err(NEGATIVE_ERR.clone().into());
        } else if n == 0.0 {
            return Err(ZERO_ERR.clone().into());
        }
        p += 1.0 / n;
    }
    Ok(l as f64 / p)
}
#[cfg(feature = "mock")]
pub fn harmonic_mean(input: Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_harmonic_mean__ground_truth"]
        fn harmonic_mean__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(harmonic_mean__foreign(ser(&input_state_in)))
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
pub fn harmonic_mean__with_callees_mocked(input: Float64Data) -> Result<f64> {
    let l = input.len();
    if l == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let mut p = 0.0;
    for n in input.0 {
        if n < 0.0 {
            return Err(NEGATIVE_ERR.clone().into());
        } else if n == 0.0 {
            return Err(ZERO_ERR.clone().into());
        }
        p += 1.0 / n;
    }
    Ok(l as f64 / p)
}

