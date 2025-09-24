use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::StatsError;
use crate::legacy::NAN_ERR;
use std::f64;
#[cfg(not(feature = "mock"))]
pub fn round(input: f64, places: i32) -> Result<f64, Error> {
    if input.is_nan() {
        return Err(Error::from(NAN_ERR.clone()));
    }
    let mut sign = 1.0;
    let mut input = input;
    if input < 0.0 {
        sign = -1.0;
        input *= -1.0;
    }
    let precision = f64::powi(10.0, places);
    let digit = input * precision;
    let decimal = digit.fract();
    let rounded = if decimal >= 0.5 { digit.ceil() } else { digit.floor() };
    Ok(rounded / precision * sign)
}
#[cfg(feature = "mock")]
pub fn round(input: f64, places: i32) -> Result<f64, Error> {
    extern "C" {
        #[link_name = "stats_round__ground_truth"]
        fn round__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
        i32,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
        i32,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(input, places);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(round__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn round__with_callees_mocked(input: f64, places: i32) -> Result<f64, Error> {
    if input.is_nan() {
        return Err(Error::from(NAN_ERR.clone()));
    }
    let mut sign = 1.0;
    let mut input = input;
    if input < 0.0 {
        sign = -1.0;
        input *= -1.0;
    }
    let precision = f64::powi(10.0, places);
    let digit = input * precision;
    let decimal = digit.fract();
    let rounded = if decimal >= 0.5 { digit.ceil() } else { digit.floor() };
    Ok(rounded / precision * sign)
}

