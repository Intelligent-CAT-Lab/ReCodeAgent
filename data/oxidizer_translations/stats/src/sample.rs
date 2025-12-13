#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::BOUNDS_ERR;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::util::unixnano;
//Translated from: github.com/montanaflynn/stats.Sample
pub fn sample(input: &Float64Data, take_num: usize, replacement: bool) -> Result<Float64Data> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }

    let length = input.len();
    if replacement {
        let mut result = Float64Data::default();
        let mut rng = thread_rng();

        for _ in 0..take_num {
            let idx = rng.gen_range(0..length);
            result.0.push(input.0[idx]);
        }

        Ok(result)
    } else if !replacement && take_num <= length {
        let mut rng = thread_rng();
        let mut permutation = (0..length).collect::<Vec<_>>();
        permutation.shuffle(&mut rng);

        let result = Float64Data(permutation[..take_num].iter().map(|&idx| input.0[idx]).collect());

        Ok(result)
    } else {
        Err(BOUNDS_ERR.clone().into())
    }
}

// Stub for StableSample - intentionally unimplemented
pub fn stable_sample(_input: &Float64Data, _take_num: usize) -> Result<Float64Data> {
    todo!("stubbed stable_sample")
}
