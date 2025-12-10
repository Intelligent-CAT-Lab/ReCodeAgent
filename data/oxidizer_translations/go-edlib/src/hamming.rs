#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::internal::utils::utils::equal;
//Translated from: github.com/hbollon/go-edlib.HammingDistance
pub fn hamming_distance(str1: &str, str2: &str) -> Result<usize> {
    let mut str1_chars: Vec<char> = str1.chars().collect();
    let mut str2_chars: Vec<char> = str2.chars().collect();

    if str1_chars.len() != str2_chars.len() {
        return Err(Error::msg("Strings have unequal lengths"));
    } else if equal(&str1_chars, &str2_chars)? {
        return Ok(0);
    }

    let mut counter = 0;
    for (a, b) in str1_chars.iter().zip(str2_chars.iter()) {
        if a != b {
            counter += 1;
        }
    }

    Ok(counter)
}
