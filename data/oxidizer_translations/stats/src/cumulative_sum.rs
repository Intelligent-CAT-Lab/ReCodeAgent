#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT;
//Translated from: github.com/montanaflynn/stats.CumulativeSum
pub fn cumulative_sum(input: &Float64Data) -> Result<Float64Data> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT.err.clone()));
    }

    let mut cum_sum = Vec::with_capacity(input.len());
    let mut sum = 0.0;
    for val in &input.0 {
        sum += val;
        cum_sum.push(sum);
    }

    Ok(Float64Data(cum_sum))
}
