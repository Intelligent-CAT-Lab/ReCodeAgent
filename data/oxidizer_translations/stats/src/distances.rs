#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::legacy::SIZE_ERR;
//Translated from: github.com/montanaflynn/stats.validateData
pub(crate) fn validate_data(data_point_x: Float64Data, data_point_y: Float64Data) -> Result<()> {
    if data_point_x.0.is_empty() || data_point_y.0.is_empty() {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    if data_point_x.0.len() != data_point_y.0.len() {
        return Err(SIZE_ERR.clone().into());
    }

    Ok(())
}
use std::f64::NAN;
//Translated from: github.com/montanaflynn/stats.ChebyshevDistance
pub fn chebyshev_distance(data_point_x: Float64Data, data_point_y: Float64Data) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;

    let mut distance = 0.0;
    for i in 0..data_point_y.0.len() {
        let temp_distance = (data_point_x.0[i] - data_point_y.0[i]).abs();
        if distance < temp_distance {
            distance = temp_distance;
        }
    }

    Ok(distance)
}
use std::ops::Sub;
//Translated from: github.com/montanaflynn/stats.EuclideanDistance
pub fn euclidean_distance(data_point_x: Float64Data, data_point_y: Float64Data) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;

    let mut distance: f64 = 0.0;
    for (x, y) in data_point_x.0.iter().zip(data_point_y.0.iter()) {
        distance += (x - y).powi(2);
    }

    Ok(distance.sqrt())
}
use std::cmp::PartialEq;
//Translated from: github.com/montanaflynn/stats.ManhattanDistance
pub fn manhattan_distance(data_point_x: Float64Data, data_point_y: Float64Data) -> Result<f64> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;

    let mut distance = 0.0;
    let data_point_x: Vec<f64> = data_point_x.into();
    let data_point_y: Vec<f64> = data_point_y.into();

    if data_point_x.len() != data_point_y.len() {
        return Err(anyhow::anyhow!("Input data points have different lengths"));
    }

    for (x, y) in data_point_x.iter().zip(data_point_y.iter()) {
        distance += (x - y).abs();
    }

    Ok(distance)
}
use std::f64;
use crate::legacy::INF_VALUE;
//Translated from: github.com/montanaflynn/stats.MinkowskiDistance
pub fn minkowski_distance(data_point_x: Float64Data, data_point_y: Float64Data, lambda: f64) -> Result<f64, Error> {
    validate_data(data_point_x.clone(), data_point_y.clone())?;

    let mut distance = 0.0;
    for i in 0..data_point_y.0.len() {
        distance += f64::powf(f64::abs(data_point_x.0[i] - data_point_y.0[i]), lambda);
    }
    distance = f64::powf(distance, 1.0 / lambda);

    if distance.is_infinite() {
        return Err(INF_VALUE.clone().into());
    }

    Ok(distance)
}
