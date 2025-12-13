#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use crate::errors::StatsError;
use crate::errors::ERR_EMPTY_INPUT;
//Translated from: github.com/montanaflynn/stats.EmptyInputErr
pub static EMPTY_INPUT_ERR: Lazy<StatsError> = Lazy::new(|| ERR_EMPTY_INPUT.clone());
use crate::errors::ERR_SIZE;
//Translated from: github.com/montanaflynn/stats.SizeErr
pub static SIZE_ERR: Lazy<StatsError> = Lazy::new(|| ERR_SIZE.clone());

//Translated from: github.com/montanaflynn/stats.EmptyInput
pub static EMPTY_INPUT: Lazy<StatsError> = Lazy::new(|| ERR_EMPTY_INPUT.clone());
use crate::errors::ERR_BOUNDS;
//Translated from: github.com/montanaflynn/stats.BoundsErr
pub static BOUNDS_ERR: Lazy<StatsError> = Lazy::new(|| ERR_BOUNDS.clone());
use crate::errors::ERR_Y_COORD;
//Translated from: github.com/montanaflynn/stats.YCoordErr
pub static Y_COORD_ERR: Lazy<StatsError> = Lazy::new(|| ERR_Y_COORD.clone());
use crate::errors::ERR_NEGATIVE;
//Translated from: github.com/montanaflynn/stats.NegativeErr
pub static NEGATIVE_ERR: Lazy<StatsError> = Lazy::new(|| {
    ERR_NEGATIVE.clone()
});
use crate::errors::ERR_ZERO;
//Translated from: github.com/montanaflynn/stats.ZeroErr
pub static ZERO_ERR: Lazy<StatsError> = Lazy::new(|| ERR_ZERO.clone());
use crate::errors::ERR_INF_VALUE;
//Translated from: github.com/montanaflynn/stats.InfValue
pub static INF_VALUE: Lazy<StatsError> = Lazy::new(|| ERR_INF_VALUE.clone());
use crate::errors::ERR_NAN;
//Translated from: github.com/montanaflynn/stats.NaNErr
pub static NAN_ERR: Lazy<StatsError> = Lazy::new(|| {
    ERR_NAN.clone()
});

use crate::data::Float64Data;
use crate::regression::Series;

// Stubbed compatibility shims; intentionally unimplemented.
pub fn VarP(_data: &Float64Data) -> Result<f64, Error> {
    todo!("stubbed VarP")
}

pub fn VarS(_data: &Float64Data) -> Result<f64, Error> {
    todo!("stubbed VarS")
}

pub fn StdDevP(_data: &Float64Data) -> Result<f64, Error> {
    todo!("stubbed StdDevP")
}

pub fn StdDevS(_data: &Float64Data) -> Result<f64, Error> {
    todo!("stubbed StdDevS")
}

pub fn LinReg(_coords: &Vec<crate::regression::Coordinate>) -> Result<Series, Error> {
    todo!("stubbed LinReg")
}

pub fn ExpReg(_coords: &Vec<crate::regression::Coordinate>) -> Result<Vec<crate::regression::Coordinate>, Error> {
    todo!("stubbed ExpReg")
}

pub fn LogReg(_coords: &Vec<crate::regression::Coordinate>) -> Result<Series, Error> {
    todo!("stubbed LogReg")
}
