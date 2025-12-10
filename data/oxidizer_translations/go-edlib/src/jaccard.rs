#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::shingle::shingle_slice;
use crate::cosine::union;
//Translated from: github.com/hbollon/go-edlib.JaccardSimilarity
pub fn jaccard_similarity(str1: &str, str2: &str, split_length: usize) -> Result<f32, Error> {
    if str1.is_empty() || str2.is_empty() {
        return Ok(0.0);
    }

    let split_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str1, split_length)?
    };

    let split_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str2, split_length)?
    };

    let rune_str1: Vec<Vec<char>> = split_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = split_str2.iter().map(|s| s.chars().collect()).collect();

    let union_str = union(&split_str1, &split_str2)?;
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;

    Ok(jacc / union_str.len() as f32)
}
