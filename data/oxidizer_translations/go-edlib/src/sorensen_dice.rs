#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
use crate::shingle::shingle;
//Translated from: github.com/hbollon/go-edlib.SorensenDiceCoefficient
pub fn sorensen_dice_coefficient(str1: &str, str2: &str, split_length: usize) -> Result<f32> {
    if str1.is_empty() && str2.is_empty() {
        return Ok(0.0);
    }

    let shingle1 = shingle(str1, split_length);
    let shingle2 = shingle(str2, split_length);

    let mut intersection = 0;
    for (shingle, _) in &shingle1 {
        if shingle2.contains_key(shingle) {
            intersection += 1;
        }
    }

    let total_shingles = shingle1.len() + shingle2.len();
    let coefficient = 2.0 * intersection as f32 / total_shingles as f32;
    Ok(coefficient)
}
