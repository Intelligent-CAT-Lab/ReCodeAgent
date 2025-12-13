pub mod correlation;
pub mod cumulative_sum;
pub mod data;
pub mod describe;
pub mod deviation;
pub mod distances;
pub mod entropy;
pub mod errors;
pub mod examples;
pub mod geometric_distribution;
pub mod legacy;
pub mod load;
pub mod max;
pub mod mean;
pub mod median;
pub mod min;
pub mod mode;
pub mod norm;
pub mod outlier;
pub mod percentile;
pub mod quartile;
pub mod regression;
pub mod round;
pub mod sample;
pub mod sigmoid;
pub mod softmax;
pub mod sum;
pub mod util;
pub mod variance;
mod __synthetic;

pub use norm::{
    Ncr, NormBoxMullerRvs, NormCdf, NormEntropy, NormFit, NormInterval, NormIsf, NormLogCdf,
    NormLogPdf, NormLogSf, NormMean, NormMedian, NormMoment, NormPdf, NormPpf, NormPpfRvs,
    NormSf, NormStats, NormStd, NormVar,
};