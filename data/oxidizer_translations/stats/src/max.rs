#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
//Translated from: github.com/montanaflynn/stats.Max
pub fn max(input: Float64Data) -> Result<f64, anyhow::Error> {
    // Return an error if there are no numbers
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }

    // Get the first value as the starting point
    let mut max = input.get(0);

    // Loop and replace higher values
    for i in 1..input.len() {
        if input.get(i) > max {
            max = input.get(i);
        }
    }

    Ok(max)
}
