use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::os::raw::c_int;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Phrase {
    #[serde(rename = "LeftID")]
    pub left_id: c_int,
    #[serde(rename = "RightID")]
    pub right_id: c_int,
    #[serde(rename = "Left")]
    pub left: String,
    #[serde(rename = "Right")]
    pub right: String,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Weight")]
    pub weight: f32,
    #[serde(rename = "Qty")]
    pub qty: c_int,
}

use crate::rank::Word;
use crate::relation::Score;
use crate::relation::Relation;
use std::collections::HashMap;
use crate::rank::Rank;
#[cfg(not(feature = "mock"))]
pub fn find_phrases(ranks: &Rank) -> Vec<Phrase> {
    let mut phrases = Vec::new();
    for (x, x_map) in &ranks.relation.node {
        for y in x_map.keys() {
            let score = x_map.get(y).unwrap();
            phrases
                .push(Phrase {
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
#[cfg(feature = "mock")]
pub fn find_phrases(ranks: &Rank) -> Vec<Phrase> {
    extern "C" {
        #[link_name = "TextRank_find_phrases__ground_truth"]
        fn find_phrases__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Rank);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Phrase>);
    let input_state_in = InputStateIn(ranks);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(find_phrases__foreign(ser(&input_state_in)))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 1usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub fn find_phrases__with_callees_mocked(ranks: &Rank) -> Vec<Phrase> {
    let mut phrases = Vec::new();
    for (x, x_map) in &ranks.relation.node {
        for y in x_map.keys() {
            let score = x_map.get(y).unwrap();
            phrases
                .push(Phrase {
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

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Sentence {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Value")]
    pub value: String,
}

#[cfg(not(feature = "mock"))]
pub fn find_sentences_by_phrases(
    ranks: Option<&Rank>,
    words: &[String],
) -> Result<Vec<Sentence>> {
    let ranks = ranks.ok_or_else(|| anyhow::anyhow!("Ranks is None"))?;
    let req_match = words.len() - 1;
    let mut sentence_ids = HashMap::new();
    for i in words {
        for j in words {
            let x = *ranks
                .word_val_id
                .get(i)
                .ok_or_else(|| anyhow::anyhow!("Word not found: {}", i))?;
            let y = *ranks
                .word_val_id
                .get(j)
                .ok_or_else(|| anyhow::anyhow!("Word not found: {}", j))?;
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
            let sentence = ranks
                .sentence_map
                .get(&sentence_id)
                .ok_or_else(|| anyhow::anyhow!("Sentence not found: {}", sentence_id))?;
            sentences
                .push(Sentence {
                    id: sentence_id,
                    value: sentence.clone(),
                });
        }
    }
    sentences.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(sentences)
}
#[cfg(feature = "mock")]
pub fn find_sentences_by_phrases(
    ranks: Option<&Rank>,
    words: &[String],
) -> Result<Vec<Sentence>> {
    extern "C" {
        #[link_name = "TextRank_find_sentences_by_phrases__ground_truth"]
        fn find_sentences_by_phrases__foreign(
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(Option<&'a Rank>, &'b [String]);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Option<Box<Rank>>, Box<[String]>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
    let input_state_in = InputStateIn(ranks, words);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(find_sentences_by_phrases__foreign(ser(&params[0]), ser(&params[1])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 2usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn find_sentences_by_phrases__with_callees_mocked(
    ranks: Option<&Rank>,
    words: &[String],
) -> Result<Vec<Sentence>> {
    let ranks = ranks.ok_or_else(|| anyhow::anyhow!("Ranks is None"))?;
    let req_match = words.len() - 1;
    let mut sentence_ids = HashMap::new();
    for i in words {
        for j in words {
            let x = *ranks
                .word_val_id
                .get(i)
                .ok_or_else(|| anyhow::anyhow!("Word not found: {}", i))?;
            let y = *ranks
                .word_val_id
                .get(j)
                .ok_or_else(|| anyhow::anyhow!("Word not found: {}", j))?;
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
            let sentence = ranks
                .sentence_map
                .get(&sentence_id)
                .ok_or_else(|| anyhow::anyhow!("Sentence not found: {}", sentence_id))?;
            sentences
                .push(Sentence {
                    id: sentence_id,
                    value: sentence.clone(),
                });
        }
    }
    sentences.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(sentences)
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct SingleWord {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Word")]
    pub word: String,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Weight")]
    pub weight: f32,
    #[serde(rename = "Qty")]
    pub qty: i32,
}

#[cfg(not(feature = "mock"))]
pub fn find_single_words(ranks: &Rank) -> Vec<SingleWord> {
    let mut single_words = Vec::new();
    for (_, word) in &ranks.words {
        single_words
            .push(SingleWord {
                id: word.id,
                word: word.token.clone(),
                weight: word.weight,
                qty: word.qty,
            });
    }
    single_words
        .sort_by(|a, b| {
            b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal)
        });
    single_words
}
#[cfg(feature = "mock")]
pub fn find_single_words(ranks: &Rank) -> Vec<SingleWord> {
    extern "C" {
        #[link_name = "TextRank_find_single_words__ground_truth"]
        fn find_single_words__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Rank);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<SingleWord>);
    let input_state_in = InputStateIn(ranks);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(find_single_words__foreign(ser(&input_state_in)))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 1usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub fn find_single_words__with_callees_mocked(ranks: &Rank) -> Vec<SingleWord> {
    let mut single_words = Vec::new();
    for (_, word) in &ranks.words {
        single_words
            .push(SingleWord {
                id: word.id,
                word: word.token.clone(),
                weight: word.weight,
                qty: word.qty,
            });
    }
    single_words
        .sort_by(|a, b| {
            b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal)
        });
    single_words
}

pub const BY_QTY: i32 = 0;

pub const BY_RELATION: i32 = 1;

#[cfg(not(feature = "mock"))]
pub fn find_sentences(
    ranks: &Rank,
    kind: i32,
    limit: i32,
) -> Result<Vec<Sentence>, Error> {
    let mut sentences = Vec::new();
    let mut cache = HashMap::new();
    let mut collect = |sentence_ids: &[i32]| -> bool {
        for id in sentence_ids.iter() {
            if sentences.len() >= limit as usize {
                return true;
            }
            if !cache.contains_key(id) {
                if let Some(sentence_value) = ranks.sentence_map.get(id) {
                    sentences
                        .push(Sentence {
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
#[cfg(feature = "mock")]
pub fn find_sentences(
    ranks: &Rank,
    kind: i32,
    limit: i32,
) -> Result<Vec<Sentence>, Error> {
    extern "C" {
        #[link_name = "TextRank_find_sentences__ground_truth"]
        fn find_sentences__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Rank, i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
    let input_state_in = InputStateIn(ranks, kind, limit);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(find_sentences__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 3usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn find_sentences__with_callees_mocked(
    ranks: &Rank,
    kind: i32,
    limit: i32,
) -> Result<Vec<Sentence>, Error> {
    let mut sentences = Vec::new();
    let mut cache = HashMap::new();
    let mut collect = |sentence_ids: &[i32]| -> bool {
        for id in sentence_ids.iter() {
            if sentences.len() >= limit as usize {
                return true;
            }
            if !cache.contains_key(id) {
                if let Some(sentence_value) = ranks.sentence_map.get(id) {
                    sentences
                        .push(Sentence {
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

#[cfg(not(feature = "mock"))]
pub fn find_sentences_from(ranks: &Rank, id: i32, limit: i32) -> Vec<Sentence> {
    let mut sentences = Vec::new();
    let limit = id + limit - 1;
    for i in id..=limit {
        let value = ranks.sentence_map.get(&i).cloned().unwrap_or_default();
        sentences.push(Sentence { id: i, value });
    }
    sentences
}
#[cfg(feature = "mock")]
pub fn find_sentences_from(ranks: &Rank, id: i32, limit: i32) -> Vec<Sentence> {
    extern "C" {
        #[link_name = "TextRank_find_sentences_from__ground_truth"]
        fn find_sentences_from__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a Rank, i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
    let input_state_in = InputStateIn(ranks, id, limit);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(
            find_sentences_from__foreign(
                ser(&params[0]),
                ser(&params[1]),
                ser(&params[2]),
            ),
        )
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 3usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
        };
        let input_state_mutated: InputStateOut = serde_json::from_value(
                inputs_mutation_reserialized,
            )
            .unwrap();
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = output_state.0;
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub fn find_sentences_from__with_callees_mocked(
    ranks: &Rank,
    id: i32,
    limit: i32,
) -> Vec<Sentence> {
    let mut sentences = Vec::new();
    let limit = id + limit - 1;
    for i in id..=limit {
        let value = ranks.sentence_map.get(&i).cloned().unwrap_or_default();
        sentences.push(Sentence { id: i, value });
    }
    sentences
}

