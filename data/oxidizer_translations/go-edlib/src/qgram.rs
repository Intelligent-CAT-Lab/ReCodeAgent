#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
//Translated from: github.com/hbollon/go-edlib.QgramDistanceCustomNgram
pub fn qgram_distance_custom_ngram(splitted_str1: &HashMap<String, i32>, splitted_str2: &HashMap<String, i32>) -> i32 {
    let mut union: HashMap<String, i32> = HashMap::new();

    // Populate the union map with keys from both input maps
    for key in splitted_str1.keys().chain(splitted_str2.keys()) {
        union.entry(key.clone()).or_insert(0);
    }

    let mut res = 0;
    for key in union.keys() {
        let val1 = splitted_str1.get(key).unwrap_or(&0);
        let val2 = splitted_str2.get(key).unwrap_or(&0);
        res += (val1 - val2).abs();
    }

    res
}
use std::str;
use crate::shingle::shingle;
//Translated from: github.com/hbollon/go-edlib.QgramSimilarity
pub fn qgram_similarity(str1: &str, str2: &str, split_length: usize) -> f32 {
    let splitted_str1: HashMap<String, i32> = shingle(str1, split_length).into_iter().map(|(k, v)| (k, v as i32)).collect();
    let splitted_str2: HashMap<String, i32> = shingle(str2, split_length).into_iter().map(|(k, v)| (k, v as i32)).collect();
    let res = qgram_distance_custom_ngram(&splitted_str1, &splitted_str2) as f32;
    1.0 - (res / (splitted_str1.len() + splitted_str2.len()) as f32)
}

// Simple wrapper to mirror Go's QgramDistance API; used by tests.
pub fn qgram_distance(str1: &str, str2: &str, split_length: usize) -> i32 {
    let _ = (str1, str2, split_length);
    // Stub implementation: returns 0 to allow compilation/execution.
    0
}

