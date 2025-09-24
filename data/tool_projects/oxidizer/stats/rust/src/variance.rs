use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::mean::mean;
use crate::legacy::EMPTY_INPUT_ERR;
use std::convert::TryInto;
use crate::errors::StatsError;
use crate::data::Float64Data;
use crate::legacy::SIZE_ERR;
#[cfg(not(feature = "mock"))]
pub fn covariance_population(data1: Float64Data, data2: Float64Data) -> Result<f64> {
    let l1: usize = data1.len();
    let l2: usize = data2.len();
    if l1 == 0 || l2 == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    if l1 != l2 {
        return Err(anyhow!(SIZE_ERR.err.clone()));
    }
    let m1 = mean(data1.clone())?;
    let m2 = mean(data2.clone())?;
    let mut s: f64 = 0.0;
    for i in 0..l1 {
        let delta1 = data1.get(i) - m1;
        let delta2 = data2.get(i) - m2;
        s += delta1 * delta2;
    }
    let result = s / (l1 as f64);
    Ok(result)
}
#[cfg(feature = "mock")]
pub fn covariance_population(data1: Float64Data, data2: Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_covariance_population__ground_truth"]
        fn covariance_population__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data, Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data, Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(data1, data2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(covariance_population__foreign(ser(&params[0]), ser(&params[1])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 2usize);
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
pub fn covariance_population__with_callees_mocked(
    data1: Float64Data,
    data2: Float64Data,
) -> Result<f64> {
    let l1: usize = data1.len();
    let l2: usize = data2.len();
    if l1 == 0 || l2 == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    if l1 != l2 {
        return Err(anyhow!(SIZE_ERR.err.clone()));
    }
    let m1 = mean(data1.clone())?;
    let m2 = mean(data2.clone())?;
    let mut s: f64 = 0.0;
    for i in 0..l1 {
        let delta1 = data1.get(i) - m1;
        let delta2 = data2.get(i) - m2;
        s += delta1 * delta2;
    }
    let result = s / (l1 as f64);
    Ok(result)
}

#[cfg(not(feature = "mock"))]
pub fn variance(input: Float64Data, sample: i32) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Ok(f64::NAN);
    }
    let m = mean(input.clone())?;
    let mut variance: f64 = 0.0;
    for n in &input.0 {
        variance += (n - m) * (n - m);
    }
    let denominator = (input.len() as f64) - (sample as f64);
    let variance = variance / denominator;
    Ok(variance)
}
#[cfg(feature = "mock")]
pub fn variance(input: Float64Data, sample: i32) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats__variance__ground_truth"]
        fn variance__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(input, sample);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(variance__foreign(ser(&params[0]), ser(&params[1])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 2usize);
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
pub fn variance__with_callees_mocked(
    input: Float64Data,
    sample: i32,
) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Ok(f64::NAN);
    }
    let m = mean(input.clone())?;
    let mut variance: f64 = 0.0;
    for n in &input.0 {
        variance += (n - m) * (n - m);
    }
    let denominator = (input.len() as f64) - (sample as f64);
    let variance = variance / denominator;
    Ok(variance)
}

#[cfg(not(feature = "mock"))]
pub fn population_variance(input: Float64Data) -> Result<f64, anyhow::Error> {
    let v = variance(input, 0)?;
    Ok(v)
}
#[cfg(feature = "mock")]
pub fn population_variance(input: Float64Data) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_population_variance__ground_truth"]
        fn population_variance__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(population_variance__foreign(ser(&input_state_in)))
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
pub fn population_variance__with_callees_mocked(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    let v = variance(input, 0)?;
    Ok(v)
}

#[cfg(not(feature = "mock"))]
pub fn sample_variance(input: Float64Data) -> Result<f64, anyhow::Error> {
    let v = variance(input, 1)?;
    Ok(v)
}
#[cfg(feature = "mock")]
pub fn sample_variance(input: Float64Data) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_sample_variance__ground_truth"]
        fn sample_variance__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(sample_variance__foreign(ser(&input_state_in)))
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
pub fn sample_variance__with_callees_mocked(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    let v = variance(input, 1)?;
    Ok(v)
}

