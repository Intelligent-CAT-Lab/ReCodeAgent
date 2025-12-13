#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::errors::StatsError;

//Translated from: github.com/montanaflynn/stats.Float64Data
#[derive(derive_more::From, derive_more::Into)]
#[derive(Default)]#[derive(Clone, Debug, PartialEq)]pub struct Float64Data(pub Vec<f64>);

impl Float64Data {
    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }
}


//Translated from: (github.com/montanaflynn/stats.Float64Data).Get
impl Float64Data {
    pub fn get(&self, i: usize) -> f64 {
        self.0[i]
    }
}

//Translated from: (github.com/montanaflynn/stats.Float64Data).Len
impl Float64Data {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
use crate::sum::sum;
//Translated from: (github.com/montanaflynn/stats.Float64Data).Sum
impl Float64Data {
    pub fn sum(&self) -> Result<f64, Error> {
        sum(self.clone())
    }
}

// Stubbed methods to mirror the translated API surface; intentionally unimplemented.
impl Float64Data {
    pub fn Less(&self, _i: usize, _j: usize) -> bool {
        todo!("stubbed Less")
    }
    pub fn Swap(&mut self, _i: usize, _j: usize) {
        todo!("stubbed Swap")
    }
    pub fn Min(&self) -> Result<f64, StatsError> {
        todo!("stubbed Min")
    }
    pub fn Max(&self) -> Result<f64, StatsError> {
        todo!("stubbed Max")
    }
    pub fn CumulativeSum(&self) -> Result<Vec<f64>, StatsError> {
        todo!("stubbed CumulativeSum")
    }
    pub fn Mean(&self) -> Result<f64, StatsError> {
        todo!("stubbed Mean")
    }
    pub fn GeometricMean(&self) -> Result<f64, StatsError> {
        todo!("stubbed GeometricMean")
    }
    pub fn HarmonicMean(&self) -> Result<f64, StatsError> {
        todo!("stubbed HarmonicMean")
    }
    pub fn Median(&self) -> Result<f64, StatsError> {
        todo!("stubbed Median")
    }
    pub fn Mode(&self) -> Result<Vec<f64>, StatsError> {
        todo!("stubbed Mode")
    }
    pub fn InterQuartileRange(&self) -> Result<f64, StatsError> {
        todo!("stubbed InterQuartileRange")
    }
    pub fn MedianAbsoluteDeviation(&self) -> Result<f64, StatsError> {
        todo!("stubbed MedianAbsoluteDeviation")
    }
    pub fn MedianAbsoluteDeviationPopulation(&self) -> Result<f64, StatsError> {
        todo!("stubbed MedianAbsoluteDeviationPopulation")
    }
    pub fn StandardDeviation(&self) -> Result<f64, StatsError> {
        todo!("stubbed StandardDeviation")
    }
    pub fn StandardDeviationPopulation(&self) -> Result<f64, StatsError> {
        todo!("stubbed StandardDeviationPopulation")
    }
    pub fn StandardDeviationSample(&self) -> Result<f64, StatsError> {
        todo!("stubbed StandardDeviationSample")
    }
    pub fn Variance(&self) -> Result<f64, StatsError> {
        todo!("stubbed Variance")
    }
    pub fn PopulationVariance(&self) -> Result<f64, StatsError> {
        todo!("stubbed PopulationVariance")
    }
    pub fn SampleVariance(&self) -> Result<f64, StatsError> {
        todo!("stubbed SampleVariance")
    }
    pub fn Percentile(&self, _p: f64) -> Result<f64, StatsError> {
        todo!("stubbed Percentile")
    }
    pub fn PercentileNearestRank(&self, _p: f64) -> Result<f64, StatsError> {
        todo!("stubbed PercentileNearestRank")
    }
    pub fn Correlation(&self, _other: &Float64Data) -> Result<f64, StatsError> {
        todo!("stubbed Correlation")
    }
    pub fn Pearson(&self, _other: &Float64Data) -> Result<f64, StatsError> {
        todo!("stubbed Pearson")
    }
    pub fn Midhinge(&self, _other: &Float64Data) -> Result<f64, StatsError> {
        todo!("stubbed Midhinge")
    }
    pub fn Trimean(&self, _other: &Float64Data) -> Result<f64, StatsError> {
        todo!("stubbed Trimean")
    }
    pub fn Covariance(&self, _other: &Float64Data) -> Result<f64, StatsError> {
        todo!("stubbed Covariance")
    }
    pub fn CovariancePopulation(&self, _other: &Float64Data) -> Result<f64, StatsError> {
        todo!("stubbed CovariancePopulation")
    }
    pub fn AutoCorrelation(&self, _lag: usize) -> Result<f64, StatsError> {
        todo!("stubbed AutoCorrelation")
    }
    pub fn Sample(&self, _n: usize, _replace: bool) -> Result<Float64Data, StatsError> {
        todo!("stubbed Sample")
    }
    pub fn QuartileOutliers(&self) -> Result<(), StatsError> {
        todo!("stubbed QuartileOutliers")
    }
    pub fn Quartile(&self, _other: &Float64Data) -> Result<(), StatsError> {
        todo!("stubbed Quartile")
    }
    pub fn Quartiles(&self) -> Result<(), StatsError> {
        todo!("stubbed Quartiles")
    }
    pub fn Sigmoid(&self) -> Result<Vec<f64>, StatsError> {
        todo!("stubbed Sigmoid")
    }
    pub fn SoftMax(&self) -> Result<Vec<f64>, StatsError> {
        todo!("stubbed SoftMax")
    }
    pub fn Entropy(&self) -> Result<f64, StatsError> {
        todo!("stubbed Entropy")
    }
}

// Stubbed loader to match expected API; intentionally unimplemented.
pub fn LoadRawData(_input: &[f64]) -> Float64Data {
    todo!("stubbed LoadRawData")
}
