#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;
use crate::data::Float64Data;
use crate::errors::StatsError;
use crate::legacy::EMPTY_INPUT_ERR;
use crate::util::sorted_copy_dif;
//Translated from: github.com/montanaflynn/stats.Mode
pub fn mode(input: Float64Data) -> Result<Vec<f64>, anyhow::Error> {
    // Return the input if there's only one number
    let l = input.len();
    if l == 1 {
        return Ok(input.0);
    } else if l == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }

    let mut c = sorted_copy_dif(input)?;
    // Traverse sorted array,
    // tracking the longest repeating sequence
    let mut mode = Vec::with_capacity(5);
    let mut cnt = 1;
    let mut max_cnt = 1;
    for i in 1..l {
        if c.0[i] == c.0[i - 1] {
            cnt += 1;
        } else {
            if cnt == max_cnt && max_cnt != 1 {
                mode.push(c.0[i - 1]);
                cnt = 1;
            } else if cnt > max_cnt {
                mode.truncate(0);
                mode.push(c.0[i - 1]);
                max_cnt = cnt;
                cnt = 1;
            } else {
                cnt = 1;
            }
        }
    }
    if cnt == max_cnt {
        mode.push(c.0[l - 1]);
    } else if cnt > max_cnt {
        mode.truncate(0);
        mode.push(c.0[l - 1]);
        max_cnt = cnt;
    }

    // Since length must be greater than 1,
    // check for slices of distinct values
    if max_cnt == 1 || mode.len() * max_cnt == l && max_cnt != l {
        return Ok(Vec::new());
    }

    Ok(mode)
}
