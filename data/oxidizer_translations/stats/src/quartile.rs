#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/montanaflynn/stats.Quartiles
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub struct Quartiles {
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
}

use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::median::median;
use crate::util::sorted_copy;
//Translated from: github.com/montanaflynn/stats.Quartile
pub fn quartile(input: Float64Data) -> Result<Quartiles, anyhow::Error> {
    let il = input.len();
    if il == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    let copy = sorted_copy(input);

    let (c1, c2) = if il % 2 == 0 {
        (il / 2, il / 2)
    } else {
        let c1 = (il - 1) / 2;
        (c1, c1 + 1)
    };

    let q1 = median(Float64Data(copy.0[..c1].to_vec()))?;
    let q2 = median(copy.clone())?;
    let q3 = median(Float64Data(copy.0[c2..].to_vec()))?;

    Ok(Quartiles { q1, q2, q3 })
}

//Translated from: github.com/montanaflynn/stats.InterQuartileRange
pub fn inter_quartile_range(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    match quartile(input) {
        Ok(qs) => {
            let iqr = qs.q3 - qs.q1;
            Ok(iqr)
        }
        Err(e) => Err(e),
    }
}

//Translated from: github.com/montanaflynn/stats.Midhinge
pub fn midhinge(input: Float64Data) -> Result<f64> {
    if input.len() == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }

    let qs = quartile(input)?;
    let mh = (qs.q1 + qs.q3) / 2.0;

    Ok(mh)
}
use std::f64::NAN;
//Translated from: github.com/montanaflynn/stats.Trimean
pub fn trimean(input: Float64Data) -> Result<f64, anyhow::Error> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    let c = sorted_copy(input);
    let q = quartile(c)?;

    let trimean = (q.q1 + (q.q2 * 2.0) + q.q3) / 4.0;
    Ok(trimean)
}
