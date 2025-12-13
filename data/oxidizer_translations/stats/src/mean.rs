#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
//Translated from: github.com/montanaflynn/stats.Mean
pub fn mean(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(anyhow!("Input is empty"));
    }

    let sum = input.sum()?;
    let result = sum / (input.len() as f64);

    Ok(result)
}
use std::f64;
//Translated from: github.com/montanaflynn/stats.GeometricMean
pub fn geometric_mean(input: &Float64Data) -> Result<f64> {
    let l = input.len();
    if l == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }

    // Get the product of all the numbers
    let mut p = 1.0;
    for n in &input.0 {
        p *= n;
    }

    // Calculate the geometric mean
    let geometric_mean = p.powf(1.0 / (l as f64));
    Ok(geometric_mean)
}
use crate::legacy::NEGATIVE_ERR;
use crate::legacy::ZERO_ERR;
//Translated from: github.com/montanaflynn/stats.HarmonicMean
pub fn harmonic_mean(input: Float64Data) -> Result<f64> {
    let l = input.len();
    if l == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    let mut p = 0.0;
    for n in input.0 {
        if n < 0.0 {
            return Err(NEGATIVE_ERR.clone().into());
        } else if n == 0.0 {
            return Err(ZERO_ERR.clone().into());
        }
        p += 1.0 / n;
    }

    Ok(l as f64 / p)
}
