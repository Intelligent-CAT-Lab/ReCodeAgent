#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::os::raw::c_int;
//Translated from: github.com/DavidBelicza/TextRank.Phrase
#[derive(Default)]#[derive(Clone)]pub struct Phrase {
    pub left_id: c_int,
    pub right_id: c_int,
    pub left: String,
    pub right: String,
    pub weight: f32,
    pub qty: c_int,
}

use std::collections::HashMap;
use crate::rank::Rank;
use crate::relation::Relation;
use crate::relation::Score;
use crate::rank::Word;
//Translated from: github.com/DavidBelicza/TextRank.FindPhrases
pub fn find_phrases(ranks: &Rank) -> Vec<Phrase> {
    let mut phrases = Vec::new();

    for (x, x_map) in &ranks.relation.node {
        for y in x_map.keys() {
            let score = x_map.get(y).unwrap();
            phrases.push(Phrase {
                left_id: ranks.words.get(x).unwrap().id,
                right_id: ranks.words.get(y).unwrap().id,
                left: ranks.words.get(x).unwrap().token.clone(),
                right: ranks.words.get(y).unwrap().token.clone(),
                weight: score.weight,
                qty: score.qty as i32,
            });
        }
    }

    phrases.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

    phrases
}

//Translated from: github.com/DavidBelicza/TextRank.Sentence
#[derive(Default)]#[derive(Clone)]pub struct Sentence {
    pub id: i32,
    pub value: String,
}


//Translated from: github.com/DavidBelicza/TextRank.FindSentencesByPhrases
pub fn find_sentences_by_phrases(ranks: Option<&Rank>, words: &[String]) -> Result<Vec<Sentence>> {
    let ranks = ranks.ok_or_else(|| anyhow::anyhow!("Ranks is None"))?;
    let req_match = words.len() - 1;
    let mut sentence_ids = HashMap::new();

    for i in words {
        for j in words {
            let x = *ranks.word_val_id.get(i).ok_or_else(|| anyhow::anyhow!("Word not found: {}", i))?;
            let y = *ranks.word_val_id.get(j).ok_or_else(|| anyhow::anyhow!("Word not found: {}", j))?;

            if let Some(score) = ranks.relation.node.get(&x).and_then(|m| m.get(&y)) {
                for id in &score.sentence_ids {
                    *sentence_ids.entry(*id).or_insert(0) += 1;
                }
            }
        }
    }

    let mut sentences = Vec::new();
    for (sentence_id, count) in sentence_ids {
        if count >= req_match {
            let sentence = ranks.sentence_map.get(&sentence_id).ok_or_else(|| anyhow::anyhow!("Sentence not found: {}", sentence_id))?;
            sentences.push(Sentence { id: sentence_id, value: sentence.clone() });
        }
    }

    sentences.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(sentences)
}

//Translated from: github.com/DavidBelicza/TextRank.SingleWord
#[derive(Default)]#[derive(Clone)]pub struct SingleWord {
    pub id: i32,
    pub word: String,
    pub weight: f32,
    pub qty: i32,
}


//Translated from: github.com/DavidBelicza/TextRank.FindSingleWords
pub fn find_single_words(ranks: &Rank) -> Vec<SingleWord> {
    let mut single_words = Vec::new();

    for (_, word) in &ranks.words {
        single_words.push(SingleWord {
            id: word.id,
            word: word.token.clone(),
            weight: word.weight,
            qty: word.qty,
        });
    }

    single_words.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));

    single_words
}

//Translated from: github.com/DavidBelicza/TextRank.ByQty
pub const BY_QTY: i32 = 0;
//Translated from: github.com/DavidBelicza/TextRank.ByRelation
pub const BY_RELATION: i32 = 1;
//Translated from: github.com/DavidBelicza/TextRank.FindSentences
pub fn find_sentences(ranks: &Rank, kind: i32, limit: i32) -> Result<Vec<Sentence>, Error> {
    let mut sentences = Vec::new();
    let mut cache = HashMap::new();

    let mut collect = |sentence_ids: &[i32]| -> bool {
        for id in sentence_ids.iter() {
            if sentences.len() >= limit as usize {
                return true;
            }

            if !cache.contains_key(id) {
                if let Some(sentence_value) = ranks.sentence_map.get(id) {
                    sentences.push(Sentence {
                        id: *id,
                        value: sentence_value.clone(),
                    });
                    cache.insert(*id, true);
                }
            }
        }

        false
    };

    if kind == BY_QTY {
        let single_words = find_single_words(ranks);

        for single_word in single_words {
            if let Some(word) = ranks.words.get(&single_word.id) {
                if collect(&word.sentence_ids) {
                    return Ok(sentences);
                }
            }
        }
    } else if kind == BY_RELATION {
        let phrases = find_phrases(ranks);

        for phrase in phrases {
            if let Some(score) = ranks
                .relation
                .node
                .get(&phrase.left_id)
                .and_then(|left_node| left_node.get(&phrase.right_id))
            {
                if collect(&score.sentence_ids) {
                    return Ok(sentences);
                }
            }
        }
    }

    Ok(sentences)
}


//Translated from: github.com/DavidBelicza/TextRank.FindSentencesFrom
pub fn find_sentences_from(ranks: &Rank, id: i32, limit: i32) -> Vec<Sentence> {
    let mut sentences = Vec::new();

    let limit = id + limit - 1;

    for i in id..=limit {
        let value = ranks.sentence_map.get(&i).cloned().unwrap_or_default();
        sentences.push(Sentence { id: i, value });
    }

    sentences
}
