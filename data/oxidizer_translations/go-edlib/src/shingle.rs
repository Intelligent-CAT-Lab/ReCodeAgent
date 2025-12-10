#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;
//Translated from: github.com/hbollon/go-edlib.ShingleSlice
pub fn shingle_slice(s: &str, k: usize) -> Result<Vec<String>, Error> {
    let mut out = Vec::new();
    let mut shingles = HashSet::new();

    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() - k + 1 {
            let shingle: String = chars[i..i + k].iter().collect();
            shingles.insert(shingle);
        }

        out = shingles.into_iter().collect();
    }

    Ok(out)
}
use std::collections::HashMap;
//Translated from: github.com/hbollon/go-edlib.Shingle
pub fn shingle(s: &str, k: usize) -> HashMap<String, usize> {
    let mut m = HashMap::new();

    if !s.is_empty() && k != 0 {
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() - k + 1 {
            let shingle: String = chars[i..i + k].iter().collect();
            *m.entry(shingle).or_insert(0) += 1;
        }
    }

    m
}
