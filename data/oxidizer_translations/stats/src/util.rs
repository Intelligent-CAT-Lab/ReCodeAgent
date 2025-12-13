#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::clone::Clone;
use crate::data::Float64Data;
//Translated from: github.com/montanaflynn/stats.copyslice
pub(crate) fn copyslice(input: Float64Data) -> Float64Data {
    let mut s: Float64Data = Float64Data(Vec::with_capacity(input.len()));
    s.0.clone_from(&input.0);
    s
}
use std::vec::Vec;
//Translated from: github.com/montanaflynn/stats.sortedCopy
pub(crate) fn sorted_copy(input: Float64Data) -> Float64Data {
    let mut copy = copyslice(input);
    copy.0.sort_by(|a, b| a.partial_cmp(b).unwrap());
    copy
}


//Translated from: github.com/montanaflynn/stats.sortedCopyDif
pub(crate) fn sorted_copy_dif(input: Float64Data) -> Result<Float64Data, anyhow::Error> {
    // Check if the input data is sorted
    if input.0.windows(2).all(|w| w[0] <= w[1]) {
        // If sorted, return the input data
        Ok(input)
    } else {
        // If not sorted, create a copy
        let mut copy = copyslice(input);

        // Sort the copy
        copy.0.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Return the sorted copy
        Ok(copy)
    }
}
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
//Translated from: github.com/montanaflynn/stats.unixnano
pub(crate) fn unixnano() -> i64 {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since Unix epoch");
    duration_since_epoch.as_nanos() as i64
}

// Translated from: github.com/montanaflynn/stats.float64ToInt
// Stubbed to make tests compile; intentionally unimplemented.
pub fn float64ToInt(_v: f64) -> i64 {
    todo!("stubbed float64ToInt")
}
