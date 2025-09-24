use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::mean::mean;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::errors::StatsError;
use anyhow::bail;
use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub fn auto_correlation(data: Float64Data, lags: usize) -> Result<f64> {
    if data.0.is_empty() {
        bail!("{}", EMPTY_INPUT_ERR.err);
    }
    let mean = mean(data.clone())?;
    let mut result = 0.0;
    let mut q = 0.0;
    for _ in 0..lags {
        let mut v = (data.0[0] - mean).powi(2);
        for i in 1..data.0.len() {
            let delta0 = data.0[i - 1] - mean;
            let delta1 = data.0[i] - mean;
            q += (delta0 * delta1 - q) / (i as f64 + 1.0);
            v += (delta1.powi(2) - v) / (i as f64 + 1.0);
        }
        result = q / v;
    }
    Ok(result)
}
#[cfg(feature = "mock")]
pub fn auto_correlation(data: Float64Data, lags: usize) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_auto_correlation__ground_truth"]
        fn auto_correlation__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(data, lags);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(auto_correlation__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn auto_correlation__with_callees_mocked(
    data: Float64Data,
    lags: usize,
) -> Result<f64> {
    if data.0.is_empty() {
        bail!("{}", EMPTY_INPUT_ERR.err);
    }
    let mean = mean(data.clone())?;
    let mut result = 0.0;
    let mut q = 0.0;
    for _ in 0..lags {
        let mut v = (data.0[0] - mean).powi(2);
        for i in 1..data.0.len() {
            let delta0 = data.0[i - 1] - mean;
            let delta1 = data.0[i] - mean;
            q += (delta0 * delta1 - q) / (i as f64 + 1.0);
            v += (delta1.powi(2) - v) / (i as f64 + 1.0);
        }
        result = q / v;
    }
    Ok(result)
}

use crate::legacy::SIZE_ERR;
use crate::variance::covariance_population;
use crate::deviation::standard_deviation_population;
#[cfg(not(feature = "mock"))]
pub fn correlation(data1: Float64Data, data2: Float64Data) -> Result<f64> {
    let l1 = data1.len();
    let l2 = data2.len();
    if l1 == 0 || l2 == 0 {
        bail!(EMPTY_INPUT_ERR.err.clone());
    }
    if l1 != l2 {
        bail!(SIZE_ERR.err.clone());
    }
    let sdev1 = standard_deviation_population(data1.clone())?;
    let sdev2 = standard_deviation_population(data2.clone())?;
    if sdev1 == 0.0 || sdev2 == 0.0 {
        return Ok(0.0);
    }
    let covp = covariance_population(data1, data2)?;
    Ok(covp / (sdev1 * sdev2))
}
#[cfg(feature = "mock")]
pub fn correlation(data1: Float64Data, data2: Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_correlation__ground_truth"]
        fn correlation__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(correlation__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn correlation__with_callees_mocked(
    data1: Float64Data,
    data2: Float64Data,
) -> Result<f64> {
    let l1 = data1.len();
    let l2 = data2.len();
    if l1 == 0 || l2 == 0 {
        bail!(EMPTY_INPUT_ERR.err.clone());
    }
    if l1 != l2 {
        bail!(SIZE_ERR.err.clone());
    }
    let sdev1 = standard_deviation_population(data1.clone())?;
    let sdev2 = standard_deviation_population(data2.clone())?;
    if sdev1 == 0.0 || sdev2 == 0.0 {
        return Ok(0.0);
    }
    let covp = covariance_population(data1, data2)?;
    Ok(covp / (sdev1 * sdev2))
}

