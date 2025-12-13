#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::mean::mean;
use crate::util::sorted_copy;
//Translated from: github.com/montanaflynn/stats.Median
pub fn median(input: Float64Data) -> Result<f64, anyhow::Error> {
    let c = sorted_copy(input);
    let l = c.0.len();

    if l == 0 {
        return Err(anyhow!("{}", EMPTY_INPUT_ERR.err));
    } else if l % 2 == 0 {
        let mean_input = Float64Data(c.0[l / 2 - 1..l / 2 + 1].to_vec());
        let median = mean(mean_input)?;
        Ok(median)
    } else {
        Ok(c.0[l / 2])
    }
}
