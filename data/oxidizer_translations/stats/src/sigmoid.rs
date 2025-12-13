#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT;
//Translated from: github.com/montanaflynn/stats.Sigmoid
pub fn sigmoid(input: Float64Data) -> Result<Vec<f64>, anyhow::Error> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT.err.clone()));
    }

    let mut output = Vec::with_capacity(input.len());
    for v in input.0 {
        output.push(1.0 / (1.0 + (-v).exp()));
    }

    Ok(output)
}
