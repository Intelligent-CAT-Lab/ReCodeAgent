#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
//Translated from: github.com/montanaflynn/stats.Min
pub fn min(input: Float64Data) -> Result<f64, anyhow::Error> {
    // Get the count of numbers in the slice
    let l = input.len();

    // Return an error if there are no numbers
    if l == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }

    // Get the first value as the starting point
    let mut min = input.get(0);

    // Iterate until done checking for a lower value
    for i in 1..l {
        if input.get(i) < min {
            min = input.get(i);
        }
    }
    Ok(min)
}
