#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::f64;
use crate::errors::StatsError;
use crate::errors::ERR_NEGATIVE;
//Translated from: github.com/montanaflynn/stats.ExpGeom
pub fn exp_geom(p: f64) -> Result<f64, anyhow::Error> {
    if p > 1.0 || p < 0.0 {
        return Err(anyhow!("{}", ERR_NEGATIVE.err));
    }

    Ok(1.0 / p)
}
use std::f64::NAN;
use crate::errors::ERR_BOUNDS;
//Translated from: github.com/montanaflynn/stats.ProbGeom
pub fn prob_geom(a: i32, b: i32, p: f64) -> Result<f64> {
    if a > b || a < 1 {
        return Err(anyhow::Error::new(ERR_BOUNDS.clone()));
    }

    let mut prob = 0.0;
    let q = 1.0 - p; // probability of failure

    for k in (a + 1)..=b {
        prob += p * q.powi(k as i32 - 1);
    }

    Ok(prob)
}
use std::f64::consts::SQRT_2;
//Translated from: github.com/montanaflynn/stats.VarGeom
pub fn var_geom(p: f64) -> Result<f64, anyhow::Error> {
    if p > 1.0 || p < 0.0 {
        return Err(anyhow::Error::msg("p must be between 0 and 1"));
    }

    let exp = (1.0 - p) / (p * p);
    Ok(exp)
}

