use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::mean::mean;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::legacy::BOUNDS_ERR;
use crate::util::sorted_copy;
use crate::errors::StatsError;
use crate::data::Float64Data;
use std::cmp::Ordering;
#[cfg(not(feature = "mock"))]
pub fn percentile(input: Float64Data, percent: f64) -> Result<f64> {
    let length = input.len();
    if length == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    if length == 1 {
        return Ok(input.0[0]);
    }
    if percent <= 0.0 || percent > 100.0 {
        return Err(BOUNDS_ERR.clone().into());
    }
    let mut c = sorted_copy(input);
    let index = (percent / 100.0) * c.0.len() as f64;
    if index == index.trunc() {
        let i = index as usize;
        Ok(c.0[i - 1])
    } else if index > 1.0 {
        let i = index.trunc() as usize;
        let data = Float64Data(vec![c.0[i - 1], c.0[i]]);
        mean(data).map_err(|_| anyhow::anyhow!("Error calculating mean"))
    } else {
        Err(BOUNDS_ERR.clone().into())
    }
}
#[cfg(feature = "mock")]
pub fn percentile(input: Float64Data, percent: f64) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_percentile__ground_truth"]
        fn percentile__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        Float64Data,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        Float64Data,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(input, percent);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(percentile__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn percentile__with_callees_mocked(input: Float64Data, percent: f64) -> Result<f64> {
    let length = input.len();
    if length == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    if length == 1 {
        return Ok(input.0[0]);
    }
    if percent <= 0.0 || percent > 100.0 {
        return Err(BOUNDS_ERR.clone().into());
    }
    let mut c = sorted_copy(input);
    let index = (percent / 100.0) * c.0.len() as f64;
    if index == index.trunc() {
        let i = index as usize;
        Ok(c.0[i - 1])
    } else if index > 1.0 {
        let i = index.trunc() as usize;
        let data = Float64Data(vec![c.0[i - 1], c.0[i]]);
        mean(data).map_err(|_| anyhow::anyhow!("Error calculating mean"))
    } else {
        Err(BOUNDS_ERR.clone().into())
    }
}

use std::cmp;
#[cfg(not(feature = "mock"))]
pub fn percentile_nearest_rank(input: Float64Data, percent: f64) -> Result<f64, Error> {
    let il = input.len();
    if il == 0 {
        return Err(Error::from(EMPTY_INPUT_ERR.clone()));
    }
    if percent < 0.0 || percent > 100.0 {
        return Err(Error::from(BOUNDS_ERR.clone()));
    }
    let mut c = sorted_copy(input);
    if percent == 100.0 {
        return Ok(c.0[il - 1]);
    }
    let or = (il as f64 * percent / 100.0).ceil() as usize;
    if or == 0 { Ok(c.0[0]) } else { Ok(c.0[or - 1]) }
}
#[cfg(feature = "mock")]
pub fn percentile_nearest_rank(input: Float64Data, percent: f64) -> Result<f64, Error> {
    extern "C" {
        #[link_name = "stats_percentile_nearest_rank__ground_truth"]
        fn percentile_nearest_rank__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        Float64Data,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        Float64Data,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(input, percent);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(percentile_nearest_rank__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn percentile_nearest_rank__with_callees_mocked(
    input: Float64Data,
    percent: f64,
) -> Result<f64, Error> {
    let il = input.len();
    if il == 0 {
        return Err(Error::from(EMPTY_INPUT_ERR.clone()));
    }
    if percent < 0.0 || percent > 100.0 {
        return Err(Error::from(BOUNDS_ERR.clone()));
    }
    let mut c = sorted_copy(input);
    if percent == 100.0 {
        return Ok(c.0[il - 1]);
    }
    let or = (il as f64 * percent / 100.0).ceil() as usize;
    if or == 0 { Ok(c.0[0]) } else { Ok(c.0[or - 1]) }
}

