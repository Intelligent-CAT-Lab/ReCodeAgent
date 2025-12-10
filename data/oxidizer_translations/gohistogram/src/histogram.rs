#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/VividCortex/gohistogram.bin
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub(crate) struct Bin {
    pub(crate) value: f64,
    pub(crate) count: f64,
}


//Translated from: github.com/VividCortex/gohistogram.Histogram
pub trait Histogram: crate::__synthetic::__Synth0__add + crate::__synthetic::__Synth1__quantile {}
impl<T> Histogram for T where T: crate::__synthetic::__Synth0__add + crate::__synthetic::__Synth1__quantile {}

