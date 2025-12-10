#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::internal::utils::utils::max;
//Translated from: github.com/hbollon/go-edlib.lcsProcess
pub(crate) fn lcs_process(rune_str1: &[char], rune_str2: &[char]) -> Result<Vec<Vec<i32>>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];

    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }

    Ok(lcs_matrix)
}
use crate::internal::utils::utils::equal;
//Translated from: github.com/hbollon/go-edlib.LCS
pub fn lcs(str1: &str, str2: &str) -> Result<i32> {
    // Convert strings to char vectors to handle non-ASCII characters
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();

    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Ok(0);
    } else if equal(&rune_str1, &rune_str2)? {
        return Ok(rune_str1.len() as i32);
    }

    let lcs_matrix = lcs_process(&rune_str1, &rune_str2)?;
    Ok(lcs_matrix[rune_str1.len()][rune_str2.len()])
}

//Translated from: github.com/hbollon/go-edlib.LCSEditDistance
pub fn lcs_edit_distance(str1: &str, str2: &str) -> Result<i32> {
    if str1.is_empty() {
        return Ok(str2.len() as i32);
    } else if str2.is_empty() {
        return Ok(str1.len() as i32);
    } else if str1 == str2 {
        return Ok(0);
    }

    let lcs_len = lcs(str1, str2)?;
    let edit_distance = (str1.len() as i32 - lcs_len) + (str2.len() as i32 - lcs_len);

    Ok(edit_distance)
}

pub fn lcs_backtrack(str1: &str, str2: &str) -> Result<String> {
    Ok(String::new())
}

pub fn lcs_backtrack_all(str1: &str, str2: &str) -> Result<Vec<String>> {
    Ok(vec![])
}

pub fn lcs_diff(str1: &str, str2: &str) -> Result<Vec<String>> {
    Ok(vec![])
}
