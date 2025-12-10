#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/DavidBelicza/TextRank.AlgorithmChain
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub struct AlgorithmChain {}

use crate::rank::Rank;
//Translated from: github.com/DavidBelicza/TextRank.Algorithm
pub trait Algorithm: crate::__synthetic::__Synth5__weighting_hits + crate::__synthetic::__Synth6__weighting_relation {}
impl<T> Algorithm for T where T: crate::__synthetic::__Synth5__weighting_hits + crate::__synthetic::__Synth6__weighting_relation {}

use std::collections::HashMap;
use crate::__synthetic::__Synth5__weighting_hits;
use crate::rank::Word;
//Translated from: (*github.com/DavidBelicza/TextRank.AlgorithmChain).WeightingHits
impl __Synth5__weighting_hits for AlgorithmChain {
    fn weighting_hits(&self, word_id: i32, rank: & Rank) -> f32 {
        let word = rank.words.get(&word_id).unwrap();
        let mut qty = 0;

        for (left_word_id, left_word_qty) in &word.connection_left {
            if let Some(left_word) = rank.words.get(left_word_id) {
                qty += left_word.qty * left_word_qty;
            }
        }

        for (right_word_id, right_word_qty) in &word.connection_right {
            if let Some(right_word) = rank.words.get(right_word_id) {
                qty += right_word.qty * right_word_qty;
            }
        }

        let weight = word.qty as f32 + qty as f32;

        weight
    }
}

use crate::__synthetic::__Synth6__weighting_relation;
use crate::relation::Relation;
use crate::relation::Score;
//Translated from: (*github.com/DavidBelicza/TextRank.AlgorithmChain).WeightingRelation
impl __Synth6__weighting_relation for AlgorithmChain {
    fn weighting_relation(&self, word1_id: i32, word2_id: i32, rank: & Rank) -> f32 {
        let relation_qty = rank.relation.node.get(&word1_id).and_then(|word1_map| word1_map.get(&word2_id)).map(|score| score.qty).unwrap_or(0);
        let word1_qty = rank.words.get(&word1_id).map(|word| word.qty).unwrap_or(0);
        let word2_qty = rank.words.get(&word2_id).map(|word| word.qty).unwrap_or(0);

        let q_diff = (f32::abs(word1_qty as f32 - word2_qty as f32)) / 100.0;
        let weight = relation_qty as f32 + q_diff;

        weight
    }
}


//Translated from: github.com/DavidBelicza/TextRank.AlgorithmDefault
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub struct AlgorithmDefault {}


//Translated from: (*github.com/DavidBelicza/TextRank.AlgorithmDefault).WeightingHits
impl __Synth5__weighting_hits for AlgorithmDefault {
    fn weighting_hits(&self, word_id: i32, rank: &Rank) -> f32 {
        let weight = rank.words.get(&word_id).map_or(0, |word| word.qty);

        weight as f32
    }
}

//Translated from: (*github.com/DavidBelicza/TextRank.AlgorithmDefault).WeightingRelation
impl __Synth6__weighting_relation for AlgorithmDefault {
    fn weighting_relation(&self, word1_id: i32, word2_id: i32, rank: &Rank) -> f32 {
        let relation_qty = rank.relation.node.get(&word1_id)
            .and_then(|word1_map| word1_map.get(&word2_id))
            .map(|score| score.qty)
            .unwrap_or(0) as f32;

        relation_qty
    }
}

//Translated from: github.com/DavidBelicza/TextRank.NewAlgorithmDefault
pub fn new_algorithm_default() -> Result<AlgorithmDefault> {
    Ok(AlgorithmDefault::default())
}
