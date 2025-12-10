#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/DavidBelicza/TextRank.normalize
pub(crate) fn normalize(weight: f32, min: f32, max: f32) -> Result<f32, anyhow::Error> {
    // Check for valid range
    if max <= min {
        return Err(anyhow::anyhow!("Invalid range: max ({}) must be greater than min ({})", max, min));
    }

    // Normalize the weight
    let normalized_weight = (weight - min) / (max - min);

    Ok(normalized_weight)
}
use std::collections::HashMap;
use crate::algorithm::Algorithm;
use crate::rank::Rank;
use crate::relation::Relation;
use crate::relation::Score;
use crate::rank::Word;
use crate::__synthetic::__Synth5__weighting_hits;
use crate::__synthetic::__Synth6__weighting_relation;
//Translated from: github.com/DavidBelicza/TextRank.updateRanks
pub(crate) fn update_ranks(ranks: &mut Rank, algorithm: &dyn Algorithm) -> Result<()> { crate::mock::mock_body!({
    for word in ranks.words.values_mut() {
        let weight = algorithm.weighting_hits(word.id, ranks);
        word.weight = weight;

        ranks.max = ranks.max.max(word.weight);
        ranks.min = ranks.min.min(word.weight);
    }

    for word in ranks.words.values_mut() {
        word.weight = normalize(word.weight, ranks.min, ranks.max)?;
    }

    for (x, x_map) in ranks.relation.node.iter_mut() {
        for (y, score) in x_map.iter_mut() {
            let weight = algorithm.weighting_relation(*x, *y, ranks);
            score.weight = weight;

            ranks.relation.max = ranks.relation.max.max(weight);
            ranks.relation.min = ranks.relation.min.min(weight);
        }
    }

    for x_map in ranks.relation.node.values_mut() {
        for score in x_map.values_mut() {
            score.weight = normalize(score.weight, ranks.relation.min, ranks.relation.max)?;
        }
    }

    Ok(())
});}

//Translated from: github.com/DavidBelicza/TextRank.Calculate
pub fn calculate(ranks: &mut Rank, algorithm: &dyn Algorithm) -> Result<()> {
    update_ranks(ranks, algorithm)
}
