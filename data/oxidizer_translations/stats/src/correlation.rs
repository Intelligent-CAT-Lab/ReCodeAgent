#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::mean::mean;
//Translated from: github.com/montanaflynn/stats.AutoCorrelation
pub fn auto_correlation(data: Float64Data, lags: usize) -> Result<f64> {
    if data.0.is_empty() {
        bail!("{}", EMPTY_INPUT_ERR.err);
    }

    let mean = mean(data.clone())?;
    let mut result = 0.0;
    let mut q = 0.0;

    for _ in 0..lags {
        let mut v = (data.0[0] - mean).powi(2);

        for i in 1..data.0.len() {
            let delta0 = data.0[i - 1] - mean;
            let delta1 = data.0[i] - mean;
            q += (delta0 * delta1 - q) / (i as f64 + 1.0);
            v += (delta1.powi(2) - v) / (i as f64 + 1.0);
        }

        result = q / v;
    }

    Ok(result)
}
use crate::legacy::SIZE_ERR;
use crate::variance::covariance_population;
use crate::deviation::standard_deviation_population;
//Translated from: github.com/montanaflynn/stats.Correlation
pub fn correlation(data1: Float64Data, data2: Float64Data) -> Result<f64> {
    let l1 = data1.len();
    let l2 = data2.len();

    if l1 == 0 || l2 == 0 {
        bail!(EMPTY_INPUT_ERR.err.clone());
    }

    if l1 != l2 {
        bail!(SIZE_ERR.err.clone());
    }

    let sdev1 = standard_deviation_population(data1.clone())?;
    let sdev2 = standard_deviation_population(data2.clone())?;

    if sdev1 == 0.0 || sdev2 == 0.0 {
        return Ok(0.0);
    }

    let covp = covariance_population(data1, data2)?;
    Ok(covp / (sdev1 * sdev2))
}

// Stub to mirror translated Pearson API; intentionally unimplemented.
pub fn Pearson(_data1: Float64Data, _data2: Float64Data) -> Result<f64> {
    todo!("stubbed Pearson correlation")
}
