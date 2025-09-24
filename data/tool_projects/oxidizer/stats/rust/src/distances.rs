use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::legacy::SIZE_ERR;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub(crate) fn validate_data(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<()> {
    if data_point_x.0.is_empty() || data_point_y.0.is_empty() {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    if data_point_x.0.len() != data_point_y.0.len() {
        return Err(SIZE_ERR.clone().into());
    }
    Ok(())
}
#[cfg(feature = "mock")]
pub(crate) fn validate_data(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<()> {
    extern "C" {
        #[link_name = "stats_validate_data__ground_truth"]
        fn validate_data__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data, Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data, Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(data_point_x, data_point_y);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(validate_data__foreign(ser(&params[0]), ser(&params[1])))
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
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub(crate) fn validate_data__with_callees_mocked(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<()> {
    if data_point_x.0.is_empty() || data_point_y.0.is_empty() {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    if data_point_x.0.len() != data_point_y.0.len() {
        return Err(SIZE_ERR.clone().into());
    }
    Ok(())
}

use std::f64::NAN;
#[cfg(not(feature = "mock"))]
pub fn chebyshev_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance = 0.0;
    for i in 0..data_point_y.0.len() {
        let temp_distance = (data_point_x.0[i] - data_point_y.0[i]).abs();
        if distance < temp_distance {
            distance = temp_distance;
        }
    }
    Ok(distance)
}
#[cfg(feature = "mock")]
pub fn chebyshev_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64, Error> {
    extern "C" {
        #[link_name = "stats_chebyshev_distance__ground_truth"]
        fn chebyshev_distance__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
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
    let input_state_in = InputStateIn(data_point_x, data_point_y);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(chebyshev_distance__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn chebyshev_distance__with_callees_mocked(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance = 0.0;
    for i in 0..data_point_y.0.len() {
        let temp_distance = (data_point_x.0[i] - data_point_y.0[i]).abs();
        if distance < temp_distance {
            distance = temp_distance;
        }
    }
    Ok(distance)
}

use std::ops::Sub;
#[cfg(not(feature = "mock"))]
pub fn euclidean_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance: f64 = 0.0;
    for (x, y) in data_point_x.0.iter().zip(data_point_y.0.iter()) {
        distance += (x - y).powi(2);
    }
    Ok(distance.sqrt())
}
#[cfg(feature = "mock")]
pub fn euclidean_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64, Error> {
    extern "C" {
        #[link_name = "stats_euclidean_distance__ground_truth"]
        fn euclidean_distance__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
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
    let input_state_in = InputStateIn(data_point_x, data_point_y);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(euclidean_distance__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn euclidean_distance__with_callees_mocked(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance: f64 = 0.0;
    for (x, y) in data_point_x.0.iter().zip(data_point_y.0.iter()) {
        distance += (x - y).powi(2);
    }
    Ok(distance.sqrt())
}

use std::cmp::PartialEq;
#[cfg(not(feature = "mock"))]
pub fn manhattan_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance = 0.0;
    let data_point_x: Vec<f64> = data_point_x.into();
    let data_point_y: Vec<f64> = data_point_y.into();
    if data_point_x.len() != data_point_y.len() {
        return Err(anyhow::anyhow!("Input data points have different lengths"));
    }
    for (x, y) in data_point_x.iter().zip(data_point_y.iter()) {
        distance += (x - y).abs();
    }
    Ok(distance)
}
#[cfg(feature = "mock")]
pub fn manhattan_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64> {
    extern "C" {
        #[link_name = "stats_manhattan_distance__ground_truth"]
        fn manhattan_distance__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
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
    let input_state_in = InputStateIn(data_point_x, data_point_y);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(manhattan_distance__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn manhattan_distance__with_callees_mocked(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
) -> Result<f64> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance = 0.0;
    let data_point_x: Vec<f64> = data_point_x.into();
    let data_point_y: Vec<f64> = data_point_y.into();
    if data_point_x.len() != data_point_y.len() {
        return Err(anyhow::anyhow!("Input data points have different lengths"));
    }
    for (x, y) in data_point_x.iter().zip(data_point_y.iter()) {
        distance += (x - y).abs();
    }
    Ok(distance)
}

use crate::legacy::INF_VALUE;
use std::f64;
#[cfg(not(feature = "mock"))]
pub fn minkowski_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
    lambda: f64,
) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance = 0.0;
    for i in 0..data_point_y.0.len() {
        distance += f64::powf(f64::abs(data_point_x.0[i] - data_point_y.0[i]), lambda);
    }
    distance = f64::powf(distance, 1.0 / lambda);
    if distance.is_infinite() {
        return Err(INF_VALUE.clone().into());
    }
    Ok(distance)
}
#[cfg(feature = "mock")]
pub fn minkowski_distance(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
    lambda: f64,
) -> Result<f64, Error> {
    extern "C" {
        #[link_name = "stats_minkowski_distance__ground_truth"]
        fn minkowski_distance__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        Float64Data,
        Float64Data,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        Float64Data,
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
    let input_state_in = InputStateIn(data_point_x, data_point_y, lambda);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(minkowski_distance__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn minkowski_distance__with_callees_mocked(
    data_point_x: Float64Data,
    data_point_y: Float64Data,
    lambda: f64,
) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;
    let mut distance = 0.0;
    for i in 0..data_point_y.0.len() {
        distance += f64::powf(f64::abs(data_point_x.0[i] - data_point_y.0[i]), lambda);
    }
    distance = f64::powf(distance, 1.0 / lambda);
    if distance.is_infinite() {
        return Err(INF_VALUE.clone().into());
    }
    Ok(distance)
}

