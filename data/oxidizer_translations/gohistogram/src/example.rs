#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::numerichistogram::NumericHistogram;
use crate::weightedhistogram::WeightedHistogram;
use crate::__synthetic::__Synth0__add;
//Translated from: github.com/VividCortex/gohistogram.Example
pub fn example() -> Result<()> {
    // Create a NumericHistogram with max bins 160
    let mut h = NumericHistogram::new_histogram(160);
    
    // Add value 160 to the histogram
    h.add(160.0);
    
    // Call various methods on the histogram
    let _ = h.quantile(0.25);
    let _ = h.cdf(18.0);
    let _ = h.count();
    let _ = h.mean();
    let _ = h.variance();

    // Create a WeightedHistogram with max bins 160 and alpha 1.0
    let mut w = WeightedHistogram::new(160, 1.0)?;
    
    // Add value 160 to the weighted histogram
    w.add(160.0);
    
    // Call various methods on the weighted histogram
    let _ = w.quantile(0.25)?;
    let _ = w.cdf(18.0);
    let _ = w.count();
    let _ = w.mean();
    let _ = w.variance();

    Ok(())
}
