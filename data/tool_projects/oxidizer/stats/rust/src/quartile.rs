use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub struct Quartiles {
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Q1")]
    pub q1: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Q2")]
    pub q2: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Q3")]
    pub q3: f64,
}

use crate::legacy::EMPTY_INPUT_ERR;
use crate::util::sorted_copy;
use crate::errors::StatsError;
use crate::data::Float64Data;
use crate::median::median;
#[cfg(not(feature = "mock"))]
pub fn quartile(input: Float64Data) -> Result<Quartiles, anyhow::Error> {
    let il = input.len();
    if il == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let copy = sorted_copy(input);
    let (c1, c2) = if il % 2 == 0 {
        (il / 2, il / 2)
    } else {
        let c1 = (il - 1) / 2;
        (c1, c1 + 1)
    };
    let q1 = median(Float64Data(copy.0[..c1].to_vec()))?;
    let q2 = median(copy.clone())?;
    let q3 = median(Float64Data(copy.0[c2..].to_vec()))?;
    Ok(Quartiles { q1, q2, q3 })
}
#[cfg(feature = "mock")]
pub fn quartile(input: Float64Data) -> Result<Quartiles, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_quartile__ground_truth"]
        fn quartile__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Quartiles);
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(quartile__foreign(ser(&input_state_in)))
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
pub fn quartile__with_callees_mocked(
    input: Float64Data,
) -> Result<Quartiles, anyhow::Error> {
    let il = input.len();
    if il == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let copy = sorted_copy(input);
    let (c1, c2) = if il % 2 == 0 {
        (il / 2, il / 2)
    } else {
        let c1 = (il - 1) / 2;
        (c1, c1 + 1)
    };
    let q1 = median(Float64Data(copy.0[..c1].to_vec()))?;
    let q2 = median(copy.clone())?;
    let q3 = median(Float64Data(copy.0[c2..].to_vec()))?;
    Ok(Quartiles { q1, q2, q3 })
}

#[cfg(not(feature = "mock"))]
pub fn inter_quartile_range(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    match quartile(input) {
        Ok(qs) => {
            let iqr = qs.q3 - qs.q1;
            Ok(iqr)
        }
        Err(e) => Err(e),
    }
}
#[cfg(feature = "mock")]
pub fn inter_quartile_range(input: Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_inter_quartile_range__ground_truth"]
        fn inter_quartile_range__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(inter_quartile_range__foreign(ser(&input_state_in)))
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
pub fn inter_quartile_range__with_callees_mocked(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    match quartile(input) {
        Ok(qs) => {
            let iqr = qs.q3 - qs.q1;
            Ok(iqr)
        }
        Err(e) => Err(e),
    }
}

#[cfg(not(feature = "mock"))]
pub fn midhinge(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let qs = quartile(input)?;
    let mh = (qs.q1 + qs.q3) / 2.0;
    Ok(mh)
}
#[cfg(feature = "mock")]
pub fn midhinge(input: Float64Data) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_midhinge__ground_truth"]
        fn midhinge__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(midhinge__foreign(ser(&input_state_in)))
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
pub fn midhinge__with_callees_mocked(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let qs = quartile(input)?;
    let mh = (qs.q1 + qs.q3) / 2.0;
    Ok(mh)
}

use std::f64::NAN;
#[cfg(not(feature = "mock"))]
pub fn trimean(input: Float64Data) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let c = sorted_copy(input);
    let q = quartile(c)?;
    let trimean = (q.q1 + (q.q2 * 2.0) + q.q3) / 4.0;
    Ok(trimean)
}
#[cfg(feature = "mock")]
pub fn trimean(input: Float64Data) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_trimean__ground_truth"]
        fn trimean__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(trimean__foreign(ser(&input_state_in)))
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
pub fn trimean__with_callees_mocked(input: Float64Data) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let c = sorted_copy(input);
    let q = quartile(c)?;
    let trimean = (q.q1 + (q.q2 * 2.0) + q.q3) / 4.0;
    Ok(trimean)
}

