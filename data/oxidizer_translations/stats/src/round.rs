#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::f64;
use crate::errors::StatsError;
use crate::legacy::NAN_ERR;
//Translated from: github.com/montanaflynn/stats.Round
pub fn round(input: f64, places: i32) -> Result<f64, Error> {
    // If the float is not a number
    if input.is_nan() {
        return Err(Error::from(NAN_ERR.clone()));
    }

    // Find out the actual sign and correct the input for later
    let mut sign = 1.0;
    let mut input = input;
    if input < 0.0 {
        sign = -1.0;
        input *= -1.0;
    }

    // Use the places arg to get the amount of precision wanted
    let precision = f64::powi(10.0, places);

    // Find the decimal place we are looking to round
    let digit = input * precision;

    // Get the actual decimal number as a fraction to be compared
    let decimal = digit.fract();

    // If the decimal is less than .5 we round down otherwise up
    let rounded = if decimal >= 0.5 {
        digit.ceil()
    } else {
        digit.floor()
    };

    // Finally we do the math to actually create a rounded number
    Ok(rounded / precision * sign)
}

