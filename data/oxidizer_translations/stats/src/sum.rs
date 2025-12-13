#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
//Translated from: github.com/montanaflynn/stats.Sum
pub fn sum(input: Float64Data) -> Result<f64, Error> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }

    // Add em up
    let mut sum = 0.0;
    for n in input.0 {
        sum += n;
    }

    Ok(sum)
}
