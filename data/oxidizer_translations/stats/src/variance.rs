#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::convert::TryInto;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::legacy::SIZE_ERR;
use crate::mean::mean;
//Translated from: github.com/montanaflynn/stats.CovariancePopulation
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

// Stub for sample covariance (divides by n-1 instead of n)
pub fn covariance(_data1: Float64Data, _data2: Float64Data) -> Result<f64> {
    todo!("stubbed covariance")
}

//Translated from: github.com/montanaflynn/stats._variance
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

//Translated from: github.com/montanaflynn/stats.PopulationVariance
pub fn population_variance(input: Float64Data) -> Result<f64, anyhow::Error> {
    let v = variance(input, 0)?;
    Ok(v)
}

//Translated from: github.com/montanaflynn/stats.SampleVariance
pub fn sample_variance(input: Float64Data) -> Result<f64, anyhow::Error> {
    let v = variance(input, 1)?;
    Ok(v)
}
