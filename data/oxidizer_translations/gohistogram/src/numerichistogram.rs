#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::VecDeque;
use crate::histogram::Bin;
//Translated from: github.com/VividCortex/gohistogram.NumericHistogram
#[derive(Default)]#[derive(Clone)]pub struct NumericHistogram {
    pub(crate) bins: VecDeque<Bin>,
    pub(crate) maxbins: usize,
    pub(crate) total: u64,
}

use std::f64;
//Translated from: (*github.com/VividCortex/gohistogram.NumericHistogram).trim
impl NumericHistogram {
    pub(crate) fn trim(&mut self) -> Result<()> {
        while self.bins.len() > self.maxbins {
            // Find closest bins in terms of value
            let mut min_delta = f64::INFINITY;
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
            let mut head = self.bins.drain(..min_delta_index - 1).collect::<VecDeque<_>>();
            let tail = self.bins.drain(min_delta_index + 1..).collect::<VecDeque<_>>();
            head.push_back(merged_bin);
            self.bins = head;
            self.bins.extend(tail);
        }
        Ok(())
    }
}
use crate::__synthetic::__Synth0__add;
//Translated from: (*github.com/VividCortex/gohistogram.NumericHistogram).Add
impl __Synth0__add for NumericHistogram {
    fn add(&mut self, n: f64) {
        self.trim().unwrap();
        self.total += 1;
        for i in 0..self.bins.len() {
            if self.bins[i].value == n {
                self.bins[i].count += 1.0;
                return;
            }

            if self.bins[i].value > n {
                let newbin = Bin { value: n, count: 1.0 };
                let mut head = VecDeque::from_iter(self.bins.iter().take(i).cloned());
                head.push_back(newbin);
                let tail = self.bins.split_off(i);
                self.bins = head.into_iter().chain(tail.into_iter()).collect();
                return;
            }
        }

        self.bins.push_back(Bin { count: 1.0, value: n });
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.NumericHistogram).CDF
impl NumericHistogram {
    pub fn cdf(&self, x: f64) -> f64 {
        let mut count = 0.0;
        for bin in &self.bins {
            if bin.value <= x {
                count += bin.count;
            }
        }

        count / self.total as f64
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.NumericHistogram).Count
impl NumericHistogram {
    pub fn count(&self) -> f64 {
        self.total as f64
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.NumericHistogram).Mean
impl NumericHistogram {
    pub fn mean(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }

        let mut sum = 0.0;

        for bin in &self.bins {
            sum += bin.value * bin.count;
        }

        sum / self.total as f64
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.NumericHistogram).Quantile
impl NumericHistogram {
    pub fn quantile(&self, q: f64) -> f64 {
        let mut count = q * (self.total as f64);
        for bin in &self.bins {
            count -= bin.count;

            if count <= 0.0 {
                return bin.value;
            }
        }

        -1.0
    }
}

//Translated from: (*github.com/VividCortex/gohistogram.NumericHistogram).Variance
impl NumericHistogram {
    pub fn variance(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }

        let mut sum = 0.0;
        let mean = self.mean();

        for bin in &self.bins {
            sum += bin.count * (bin.value - mean).powi(2);
        }

        sum / self.total as f64
    }
}

//Translated from: github.com/VividCortex/gohistogram.NewHistogram
impl NumericHistogram {
    pub fn new_histogram(n: usize) -> NumericHistogram {
        NumericHistogram {
            bins: VecDeque::new(),
            maxbins: n,
            total: 0,
        }
    }
}
