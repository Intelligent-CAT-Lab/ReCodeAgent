#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use std::f64::consts::E;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT;
use crate::max::max;
//Translated from: github.com/montanaflynn/stats.SoftMax
pub fn soft_max(input: Float64Data) -> Result<Vec<f64>> {
    if input.len() == 0 {
        bail!(EMPTY_INPUT.err.clone());
    }

    let c = max(input.clone())?.to_owned();
    let s: f64 = input
        .0
        .iter()
        .map(|x| E.powf(x - c))
        .sum();

    let sm: Vec<f64> = input
        .0
        .iter()
        .map(|x| E.powf(x - c) / s)
        .collect();

    Ok(sm)
}
