#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/VividCortex/gohistogram.ewma
pub(crate) fn ewma(existing_val: f64, new_val: f64, alpha: f64) -> Result<f64, anyhow::Error> {
    let result = new_val * (1.0 - alpha) + existing_val * alpha;
    Ok(result)
}
use std::vec::Vec;
use crate::histogram::Bin;
//Translated from: github.com/VividCortex/gohistogram.WeightedHistogram
#[derive(Default)]#[derive(Clone)]pub struct WeightedHistogram {
    pub(crate) bins: Vec<Bin>,
    pub(crate) maxbins: usize,
    pub(crate) total: f64,
    pub(crate) alpha: f64,
}


//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).scaleDown
impl WeightedHistogram {
    pub(crate) fn scale_down(&mut self, except: usize) {
        for i in 0..self.bins.len() {
            if i != except {
                self.bins[i].count = ewma(self.bins[i].count, 0.0, self.alpha).unwrap();
            }
        }
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).trim
impl WeightedHistogram {
    pub(crate) fn trim(&mut self) -> Result<()> {
        let mut total = 0.0;
        for bin in &self.bins {
            total += bin.count;
        }
        self.total = total;

        while self.bins.len() > self.maxbins {
            // Find closest bins in terms of value
            let mut min_delta = std::f64::INFINITY;
            let mut min_delta_index = 0;
            for i in 1..self.bins.len() {
                let delta = self.bins[i].value - self.bins[i - 1].value;
                if delta < min_delta {
                    min_delta = delta;
                    min_delta_index = i;
                }
            }

            // We need to merge bins min_delta_index-1 and min_delta_index
            let total_count = self.bins[min_delta_index - 1].count
                + self.bins[min_delta_index].count;
            let merged_bin = Bin {
                value: (self.bins[min_delta_index - 1].value
                    * self.bins[min_delta_index - 1].count
                    + self.bins[min_delta_index].value
                        * self.bins[min_delta_index].count)
                    / total_count, // weighted average
                count: total_count, // summed heights
            };
            let mut head = Vec::with_capacity(min_delta_index - 1);
            head.extend_from_slice(&self.bins[..min_delta_index - 1]);
            let mut tail = Vec::with_capacity(self.bins.len() - min_delta_index);
            tail.extend_from_slice(&self.bins[min_delta_index + 1..]);
            head.push(merged_bin);
            self.bins = head;
            self.bins.extend(tail);
        }

        Ok(())
    }
}
use crate::__synthetic::__Synth0__add;
//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).Add
impl __Synth0__add for WeightedHistogram {
    fn add(&mut self, n: f64) {
        if let Err(_) = self.trim() {
            return;
        }

        for i in 0..self.bins.len() {
            if self.bins[i].value == n {
                self.bins[i].count += 1.0;
                self.scale_down(i);
                return;
            }

            if self.bins[i].value > n {
                let mut new_bins = self.bins[..i].to_vec();
                new_bins.push(Bin { value: n, count: 1.0 });
                new_bins.extend_from_slice(&self.bins[i..]);
                self.bins = new_bins;
                self.scale_down(i);
                return;
            }
        }

        self.bins.push(Bin { value: n, count: 1.0 });
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).CDF
impl WeightedHistogram {
    pub fn cdf(&self, x: f64) -> f64 {
        let mut count = 0.0;
        for bin in &self.bins {
            if bin.value <= x {
                count += bin.count;
            }
        }

        count / self.total
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).Count
impl WeightedHistogram {
    pub fn count(&self) -> f64 {
        self.total
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).Mean
impl WeightedHistogram {
    pub fn mean(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }

        let mut sum = 0.0;

        for bin in &self.bins {
            sum += bin.value * bin.count;
        }

        sum / self.total
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).Quantile
impl WeightedHistogram {
    pub fn quantile(&self, q: f64) -> Result<f64, anyhow::Error> {
        let count = q * self.total;
        let mut cumulative_count = 0.0;

        for bin in &self.bins {
            cumulative_count += bin.count;

            if cumulative_count >= count {
                return Ok(bin.value);
            }
        }

        Err(anyhow!("No quantile found"))
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.WeightedHistogram).Variance
impl WeightedHistogram {
    pub fn variance(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }

        let mut sum = 0.0;
        let mean = self.mean();

        for bin in &self.bins {
            sum += (bin.count * (bin.value - mean) * (bin.value - mean));
        }

        sum / self.total
    }
}

//Translated from: github.com/VividCortex/gohistogram.NewWeightedHistogram
impl WeightedHistogram {
    pub fn new(n: usize, alpha: f64) -> Result<Self> {
        Ok(WeightedHistogram {
            bins: Vec::with_capacity(n),
            maxbins: n,
            total: 0.0,
            alpha,
        })
    }
}
