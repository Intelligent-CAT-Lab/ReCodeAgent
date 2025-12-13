#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/montanaflynn/stats.Coordinate
#[derive(PartialEq, PartialOrd, Debug)]
#[derive(Default)]#[derive(Clone)]pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}


//Translated from: github.com/montanaflynn/stats.Series
#[derive(derive_more::From, derive_more::Into, Debug)]
#[derive(Default)]#[derive(Clone)]pub struct Series(pub Vec<Coordinate>);

use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::legacy::Y_COORD_ERR;
//Translated from: github.com/montanaflynn/stats.ExponentialRegression
pub fn exponential_regression(s: &Series) -> Result<Vec<Coordinate>, anyhow::Error> {
    if s.0.is_empty() {
        return Err(EMPTY_INPUT_ERR.to_owned().into());
    }

    let mut sum = [0.0; 6];

    for coord in &s.0 {
        if coord.y < 0.0 {
            return Err(Y_COORD_ERR.to_owned().into());
        }
        sum[0] += coord.x;
        sum[1] += coord.y;
        sum[2] += coord.x * coord.x * coord.y;
        sum[3] += coord.y * coord.y.ln();
        sum[4] += coord.x * coord.y * coord.y.ln();
        sum[5] += coord.x * coord.y;
    }

    let denominator = sum[1] * sum[2] - sum[5] * sum[5];
    let a = f64::exp((sum[2] * sum[3] - sum[5] * sum[4]) / denominator);
    let b = (sum[1] * sum[4] - sum[5] * sum[3]) / denominator;

    let mut regressions = Vec::with_capacity(s.0.len());
    for coord in &s.0 {
        regressions.push(Coordinate {
            x: coord.x,
            y: a * f64::exp(b * coord.x),
        });
    }

    Ok(regressions)
}

use std::iter::Sum;
//Translated from: github.com/montanaflynn/stats.LinearRegression
pub fn linear_regression(series: &Series) -> Result<Series, Error> {
    if series.0.is_empty() {
        return Err(anyhow::Error::msg(EMPTY_INPUT_ERR.err.clone()));
    }

    let mut sum = [0.0; 5];
    let mut i = 0;

    for coord in &series.0 {
        sum[0] += coord.x;
        sum[1] += coord.y;
        sum[2] += coord.x * coord.x;
        sum[3] += coord.x * coord.y;
        sum[4] += coord.y * coord.y;
        i += 1;
    }

    let f = i as f64;
    let gradient = (f * sum[3] - sum[0] * sum[1]) / (f * sum[2] - sum[0] * sum[0]);
    let intercept = (sum[1] / f) - (gradient * sum[0] / f);

    let regressions = series
        .0
        .iter()
        .map(|coord| Coordinate {
            x: coord.x,
            y: coord.x * gradient + intercept,
        })
        .collect();

    Ok(Series(regressions))
}


//Translated from: github.com/montanaflynn/stats.LogarithmicRegression
pub fn logarithmic_regression(s: Series) -> Result<Series, Error> {
    if s.0.is_empty() {
        return Err(Error::from(EMPTY_INPUT_ERR.clone()));
    }

    let mut sum = [0.0; 4];

    for coord in &s.0 {
        sum[0] += coord.x.ln();
        sum[1] += coord.y * coord.x.ln();
        sum[2] += coord.y;
        sum[3] += (coord.x.ln()).powi(2);
    }

    let f = s.0.len() as f64;
    let a = (f * sum[1] - sum[2] * sum[0]) / (f * sum[3] - sum[0] * sum[0]);
    let b = (sum[2] - a * sum[0]) / f;

    let regressions = s
        .0
        .iter()
        .map(|coord| Coordinate {
            x: coord.x,
            y: b + a * coord.x.ln(),
        })
        .collect();

    Ok(Series(regressions))
}
