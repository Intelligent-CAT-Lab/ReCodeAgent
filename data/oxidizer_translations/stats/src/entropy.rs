#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
//Translated from: github.com/montanaflynn/stats.normalize
pub(crate) fn normalize(input: Float64Data) -> Result<Float64Data> {
    let sum = input.sum()?;
    let normalized = Float64Data(input.0.into_iter().map(|x| x / sum).collect());
    Ok(normalized)
}
use std::f64::consts::LN_2;
//Translated from: github.com/montanaflynn/stats.Entropy
pub fn entropy(input: Float64Data) -> Result<f64> {
    let input = normalize(input)?;
    let mut result = 0.0;
    for i in 0..input.len() {
        let v = input.get(i);
        if v != 0.0 {
            result += v * v.ln();
        }
    }
    Ok(-result)
}
