use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct DescriptionPercentile {
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Percentile")]
    pub percentile: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Value")]
    pub value: f64,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Description {
    #[serde(rename = "Count")]
    pub count: i32,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Mean")]
    pub mean: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Std")]
    pub std: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Max")]
    pub max: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "DescriptionPercentiles")]
    pub description_percentiles: Vec<DescriptionPercentile>,
    #[serde(rename = "AllowedNaN")]
    pub allowed_nan: bool,
}

use crate::errors::ERR_EMPTY_INPUT;
use crate::deviation::standard_deviation;
use crate::mean::mean;
use crate::percentile::percentile_nearest_rank;
use crate::min::min;
use crate::errors::StatsError;
use crate::max::max;
use crate::data::Float64Data;
use crate::percentile::percentile;
pub fn describe_percentile_func(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
    percentile_func: fn(Float64Data, f64) -> Result<f64, anyhow::Error>,
) -> Result<Description, anyhow::Error> {
    let mut description = Description {
        allowed_nan: allow_nan,
        count: input.len() as i32,
        ..Default::default()
    };
    if description.count == 0 && !allow_nan {
        return Err(ERR_EMPTY_INPUT.clone().into());
    }
    description.std = standard_deviation(input.clone())?;
    description.max = max(input.clone())?;
    description.min = min(input.clone())?;
    description.mean = mean(input.clone())?;
    if let Some(percentiles) = percentiles {
        for &percentile in percentiles {
            if let Ok(value) = percentile_func(input.clone(), percentile) {
                description
                    .description_percentiles
                    .push(DescriptionPercentile {
                        percentile,
                        value,
                    });
            } else if allow_nan {}
        }
    }
    Ok(description)
}

#[cfg(not(feature = "mock"))]
pub fn describe(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
) -> Result<Description> {
    let percentile_func = |data: Float64Data, percentile: f64| -> Result<f64> {
        Ok(percentile)
    };
    describe_percentile_func(input, allow_nan, percentiles, percentile_func)
}
#[cfg(feature = "mock")]
pub fn describe(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
) -> Result<Description> {
    extern "C" {
        #[link_name = "stats_describe__ground_truth"]
        fn describe__foreign(_: JSONObject, _: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(
        Float64Data,
        bool,
        #[serde_as(as = "Option < & 'a [crate :: interoperation_utils :: MyFloat64] >")]
        Option<&'a [f64]>,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        Float64Data,
        bool,
        #[serde_as(
            as = "Option < Box < [crate :: interoperation_utils :: MyFloat64] > >"
        )]
        Option<Box<[f64]>>,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Description);
    let input_state_in = InputStateIn(input, allow_nan, percentiles);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(describe__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn describe__with_callees_mocked(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
) -> Result<Description> {
    let percentile_func = |data: Float64Data, percentile: f64| -> Result<f64> {
        Ok(percentile)
    };
    describe_percentile_func(input, allow_nan, percentiles, percentile_func)
}

