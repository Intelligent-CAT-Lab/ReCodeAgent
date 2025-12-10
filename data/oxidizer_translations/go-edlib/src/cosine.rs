#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::internal::utils::utils::equal;
//Translated from: github.com/hbollon/go-edlib.find
pub fn find(slice: &[Vec<char>], val: &[char]) -> Result<i32, Error> {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, val)? {
            return Ok(i as i32);
        }
    }
    Ok(-1)
}

//Translated from: github.com/hbollon/go-edlib.sum
pub fn sum(arr: &[i32]) -> i32 {
    let mut res = 0;
    for v in arr {
        res += v;
    }
    res
}
use std::collections::HashSet;
//Translated from: github.com/hbollon/go-edlib.union
pub fn union(a: &[String], b: &[String]) -> Result<Vec<Vec<char>>, Error> {
    let mut set: HashSet<String> = HashSet::new();

    // Populate the set with elements from 'a'
    for item in a {
        set.insert(item.clone());
    }

    // Add unique elements from 'b' to 'a'
    let mut out: Vec<String> = a.to_vec();
    for item in b {
        if !set.contains(item) {
            out.push(item.clone());
        }
    }

    // Convert the output to Vec<Vec<char>>
    let out: Vec<Vec<char>> = out
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    Ok(out)
}
use crate::shingle::shingle_slice;
//Translated from: github.com/hbollon/go-edlib.CosineSimilarity
pub fn cosine_similarity(str1: &str, str2: &str, split_length: usize) -> Result<f32, Error> {
    if str1.is_empty() || str2.is_empty() {
        return Ok(0.0);
    }

    let splitted_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str1, split_length)?
    };

    let splitted_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str2, split_length)?
    };

    let rune_str1: Vec<Vec<char>> = splitted_str1.iter().map(|s| s.chars().collect()).collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2.iter().map(|s| s.chars().collect()).collect();

    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    let union_str: Vec<Vec<char>> = union(&splitted_str1, &splitted_str2)?;

    for word in &union_str {
        let fw = find(&rune_str1, &word)?;
        l1.push(if fw != -1 { 1 } else { 0 });

        let fw = find(&rune_str2, &word)?;
        l2.push(if fw != -1 { 1 } else { 0 });
    }

    let mut cosine_sim: f32 = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += (l1[i] * l2[i]) as f32;
    }

    let l1_sum: f32 = sum(&l1) as f32;
    let l2_sum: f32 = sum(&l2) as f32;

    Ok(cosine_sim / (l1_sum.sqrt() * l2_sum.sqrt()))
}
