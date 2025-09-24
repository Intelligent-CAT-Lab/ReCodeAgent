use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::legacy::EMPTY_INPUT_ERR;
use std::f64::NAN;
use crate::errors::StatsError;
use crate::data::Float64Data;
use crate::variance::population_variance;
#[cfg(not(feature = "mock"))]
pub fn standard_deviation_population(input: Float64Data) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Err(anyhow!("Input data is empty"));
    }
    let variance = population_variance(input)?;
    Ok(variance.sqrt())
}
#[cfg(feature = "mock")]
pub fn standard_deviation_population(input: Float64Data) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_standard_deviation_population__ground_truth"]
        fn standard_deviation_population__foreign(_: JSONObject) -> JSONObject;
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
        de::<
            ForeignExecution,
        >(standard_deviation_population__foreign(ser(&input_state_in)))
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
pub fn standard_deviation_population__with_callees_mocked(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Err(anyhow!("Input data is empty"));
    }
    let variance = population_variance(input)?;
    Ok(variance.sqrt())
}

#[cfg(not(feature = "mock"))]
pub fn standard_deviation(input: Float64Data) -> Result<f64, anyhow::Error> {
    standard_deviation_population(input)
}
#[cfg(feature = "mock")]
pub fn standard_deviation(input: Float64Data) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_standard_deviation__ground_truth"]
        fn standard_deviation__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(standard_deviation__foreign(ser(&input_state_in)))
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
pub fn standard_deviation__with_callees_mocked(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    standard_deviation_population(input)
}

use crate::util::copyslice;
use crate::median::median;
#[cfg(not(feature = "mock"))]
pub fn median_absolute_deviation_population(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Ok(NAN);
    }
    let mut i = copyslice(input);
    let m = median(i.clone())?;
    for value in i.0.iter_mut() {
        *value = (*value - m).abs();
    }
    median(Float64Data(i.0))
}
#[cfg(feature = "mock")]
pub fn median_absolute_deviation_population(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_median_absolute_deviation_population__ground_truth"]
        fn median_absolute_deviation_population__foreign(_: JSONObject) -> JSONObject;
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
        de::<
            ForeignExecution,
        >(median_absolute_deviation_population__foreign(ser(&input_state_in)))
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
pub fn median_absolute_deviation_population__with_callees_mocked(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Ok(NAN);
    }
    let mut i = copyslice(input);
    let m = median(i.clone())?;
    for value in i.0.iter_mut() {
        *value = (*value - m).abs();
    }
    median(Float64Data(i.0))
}

use crate::variance::sample_variance;
use std::f64;
#[cfg(not(feature = "mock"))]
pub fn standard_deviation_sample(input: Float64Data) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Ok(f64::NAN);
    }
    let variance = sample_variance(input)?;
    let std_dev = variance.sqrt();
    Ok(std_dev)
}
#[cfg(feature = "mock")]
pub fn standard_deviation_sample(input: Float64Data) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_standard_deviation_sample__ground_truth"]
        fn standard_deviation_sample__foreign(_: JSONObject) -> JSONObject;
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
        de::<ForeignExecution>(standard_deviation_sample__foreign(ser(&input_state_in)))
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
pub fn standard_deviation_sample__with_callees_mocked(
    input: Float64Data,
) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Ok(f64::NAN);
    }
    let variance = sample_variance(input)?;
    let std_dev = variance.sqrt();
    Ok(std_dev)
}

