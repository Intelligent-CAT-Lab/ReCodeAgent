#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::f64::NAN;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::variance::population_variance;
//Translated from: github.com/montanaflynn/stats.StandardDeviationPopulation
pub fn standard_deviation_population(input: Float64Data) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Err(anyhow!("Input data is empty"));
    }

    let variance = population_variance(input)?;
    Ok(variance.sqrt())
}

//Translated from: github.com/montanaflynn/stats.StandardDeviation
pub fn standard_deviation(input: Float64Data) -> Result<f64, anyhow::Error> {
    standard_deviation_population(input)
}
use crate::median::median;
use crate::util::copyslice;
//Translated from: github.com/montanaflynn/stats.MedianAbsoluteDeviationPopulation
pub fn median_absolute_deviation_population(input: Float64Data) -> Result<f64, anyhow::Error> {
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
use std::f64;
use crate::variance::sample_variance;
//Translated from: github.com/montanaflynn/stats.StandardDeviationSample
pub fn standard_deviation_sample(input: Float64Data) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Ok(f64::NAN);
    }

    let variance = sample_variance(input)?;
    let std_dev = variance.sqrt();

    Ok(std_dev)
}

// Stubbed entry point to mirror translated API; intentionally unimplemented.
pub fn median_absolute_deviation(_input: Float64Data) -> Result<f64, anyhow::Error> {
    todo!("stubbed median_absolute_deviation")
}
