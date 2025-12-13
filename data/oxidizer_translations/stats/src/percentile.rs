#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::cmp::Ordering;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::BOUNDS_ERR;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::mean::mean;
use crate::util::sorted_copy;
//Translated from: github.com/montanaflynn/stats.Percentile
pub fn percentile(input: Float64Data, percent: f64) -> Result<f64> {
    let length = input.len();
    if length == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    if length == 1 {
        return Ok(input.0[0]);
    }

    if percent <= 0.0 || percent > 100.0 {
        return Err(BOUNDS_ERR.clone().into());
    }

    let mut c = sorted_copy(input);

    let index = (percent / 100.0) * c.0.len() as f64;

    if index == index.trunc() {
        let i = index as usize;
        Ok(c.0[i - 1])
    } else if index > 1.0 {
        let i = index.trunc() as usize;
        let data = Float64Data(vec![c.0[i - 1], c.0[i]]);
        mean(data).map_err(|_| anyhow::anyhow!("Error calculating mean"))
    } else {
        Err(BOUNDS_ERR.clone().into())
    }
}
use std::cmp;
//Translated from: github.com/montanaflynn/stats.PercentileNearestRank
pub fn percentile_nearest_rank(input: Float64Data, percent: f64) -> Result<f64, Error> {
    // Find the length of items in the slice
    let il = input.len();

    // Return an error for empty slices
    if il == 0 {
        return Err(Error::from(EMPTY_INPUT_ERR.clone()));
    }

    // Return error for less than 0 or greater than 100 percentages
    if percent < 0.0 || percent > 100.0 {
        return Err(Error::from(BOUNDS_ERR.clone()));
    }

    // Start by sorting a copy of the slice
    let mut c = sorted_copy(input);

    // Return the last item
    if percent == 100.0 {
        return Ok(c.0[il - 1]);
    }

    // Find ordinal ranking
    let or = (il as f64 * percent / 100.0).ceil() as usize;

    // Return the item that is in the place of the ordinal rank
    if or == 0 {
        Ok(c.0[0])
    } else {
        Ok(c.0[or - 1])
    }
}
