#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/hbollon/go-edlib.matchingIndex
pub(crate) fn matching_index(str1: &str, str2: &str, distance: usize) -> Result<f32, anyhow::Error> {
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    let longer_len = chars1.len().max(chars2.len());
    let shorter_len = chars1.len().min(chars2.len());

    let matching_percentage = if longer_len >= shorter_len + distance {
        (shorter_len as f32) / (longer_len as f32)
    } else {
        ((longer_len - distance) as f32) / (longer_len as f32)
    };

    Ok(matching_percentage)
}

//Translated from: github.com/hbollon/go-edlib.Algorithm
#[derive(Ord, Eq, Hash)]
#[derive(PartialOrd, PartialEq, Clone, derive_more::Add, derive_more::Sub)]
#[derive(derive_more::From, derive_more::Into)]
#[derive(Default)]pub struct Algorithm(pub u8);


//Translated from: github.com/hbollon/go-edlib.Cosine
pub const COSINE: Algorithm = Algorithm(7);

//Translated from: github.com/hbollon/go-edlib.DamerauLevenshtein
pub const DAMERAU_LEVENSHTEIN: Algorithm = Algorithm(1);

//Translated from: github.com/hbollon/go-edlib.Hamming
pub const Hamming: Algorithm = Algorithm(4);

//Translated from: github.com/hbollon/go-edlib.Jaccard
pub const JACCARD: Algorithm = Algorithm(8);

//Translated from: github.com/hbollon/go-edlib.Jaro
pub const Jaro: Algorithm = Algorithm(5);

//Translated from: github.com/hbollon/go-edlib.JaroWinkler
pub const JARO_WINKLER: Algorithm = Algorithm(6);

//Translated from: github.com/hbollon/go-edlib.Lcs
pub const Lcs: Algorithm = Algorithm(3);

//Translated from: github.com/hbollon/go-edlib.Levenshtein
pub const LEVENSHTEIN: Algorithm = Algorithm(0);

//Translated from: github.com/hbollon/go-edlib.OSADamerauLevenshtein
pub const OSADamerauLevenshtein: Algorithm = Algorithm(2);

//Translated from: github.com/hbollon/go-edlib.Qgram
pub const QGRAM: Algorithm = Algorithm(10);

//Translated from: github.com/hbollon/go-edlib.SorensenDice
pub const SorensenDice: Algorithm = Algorithm(9);
use crate::cosine::cosine_similarity;
use crate::levenshtein::damerau_levenshtein_distance;
use crate::hamming::hamming_distance;
use crate::jaccard::jaccard_similarity;
use crate::jaro::jaro_similarity;
use crate::jaro::jaro_winkler_similarity;
use crate::lcs::lcs_edit_distance;
use crate::levenshtein::levenshtein_distance;
use crate::levenshtein::osa_damerau_levenshtein_distance;
use crate::qgram::qgram_similarity;
use crate::sorensen_dice::sorensen_dice_coefficient;
//Translated from: github.com/hbollon/go-edlib.StringsSimilarity
pub fn strings_similarity(str1: &str, str2: &str, algo: Algorithm) -> Result<f32> {
    match algo {
        LEVENSHTEIN => Ok(matching_index(str1, str2, levenshtein_distance(str1, str2)? as usize)?),
        DAMERAU_LEVENSHTEIN => Ok(matching_index(str1, str2, damerau_levenshtein_distance(str1, str2)? as usize)?),
        OSADamerauLevenshtein => Ok(matching_index(str1, str2, osa_damerau_levenshtein_distance(str1, str2)? as usize)?),
        Lcs => Ok(matching_index(str1, str2, lcs_edit_distance(str1, str2)? as usize)?),
        Hamming => match hamming_distance(str1, str2) {
            Ok(distance) => matching_index(str1, str2, distance.into()),
            Err(err) => return Err(anyhow!(err)),
        },
        Jaro => jaro_similarity(str1, str2).map_err(anyhow::Error::from),
        JARO_WINKLER => jaro_winkler_similarity(str1, str2).map_err(anyhow::Error::from),
        COSINE => cosine_similarity(str1, str2, 2).map_err(anyhow::Error::from),
        JACCARD => jaccard_similarity(str1, str2, 2).map_err(anyhow::Error::from),
        SorensenDice => sorensen_dice_coefficient(str1, str2, 2).map_err(anyhow::Error::from),
        QGRAM => Ok(qgram_similarity(str1, str2, 2)),
        _ => Err(anyhow!("Illegal argument for algorithm method")),
    }
}

pub fn fuzzy_search(str: &str, str_list: &[String], algo: Algorithm) -> Result<String> {
    Ok(String::new())
}

pub fn fuzzy_search_threshold(str: &str, str_list: &[String], min_sim: f32, algo: Algorithm) -> Result<String> {
    Ok(String::new())
}

pub fn fuzzy_search_set(str: &str, str_list: &[String], quantity: usize, algo: Algorithm) -> Result<Vec<String>> {
    Ok(vec![])
}

pub fn fuzzy_search_set_threshold(str: &str, str_list: &[String], quantity: usize, min_sim: f32, algo: Algorithm) -> Result<Vec<String>> {
    Ok(vec![])
}
