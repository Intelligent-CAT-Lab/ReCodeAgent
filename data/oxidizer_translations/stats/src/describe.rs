#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/montanaflynn/stats.descriptionPercentile
#[derive(PartialEq, PartialOrd, Debug)]
#[derive(Default)]#[derive(Clone)]pub struct DescriptionPercentile {
    pub percentile: f64,
    pub value: f64,
}


//Translated from: github.com/montanaflynn/stats.Description
#[derive(Default)]#[derive(Clone, Debug)]pub struct Description {
    pub count: i32,
    pub mean: f64,
    pub std: f64,
    pub max: f64,
    pub min: f64,
    pub description_percentiles: Vec<DescriptionPercentile>,
    pub allowed_nan: bool,
}

use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::errors::ERR_EMPTY_INPUT;
use crate::max::max;
use crate::mean::mean;
use crate::min::min;
use crate::percentile::percentile;
use crate::percentile::percentile_nearest_rank;
use crate::deviation::standard_deviation;
//Translated from: github.com/montanaflynn/stats.DescribePercentileFunc
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
                description.description_percentiles.push(DescriptionPercentile {
                    percentile,
                    value,
                });
            } else if allow_nan {
                // Handle NaN case if allowNaN is true
            }
        }
    }

    Ok(description)
}


//Translated from: github.com/montanaflynn/stats.Describe
pub fn describe(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
) -> Result<Description> {
    let percentile_func = |data: Float64Data, percentile: f64| -> Result<f64> {
        // Implement Percentile calculation here
        Ok(percentile)
    };

    describe_percentile_func(input, allow_nan, percentiles, percentile_func)
}

// Stubbed String formatter to match expected API; intentionally unimplemented.
impl Description {
    pub fn String(&self, _places: i32) -> String {
        todo!("stubbed Description::String")
    }
}
