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
#[cfg(test)]
mod TextRank_Phrase_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "TextRank_Phrase_roundtrip"]
        fn Phrase__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn Phrase__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!("(*{}).", "github.com-DavidBelicza-TextRank.Phrase"),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!("({}).", "github.com-DavidBelicza-TextRank.Phrase"),
                            )
                    {
                        let unittests_file: std::fs::File = std::fs::File::open(
                                entry.path(),
                            )
                            .unwrap();
                        let unittests_reader = std::io::BufReader::new(unittests_file);
                        let unittests: Vec<ExecutionData> = serde_json::from_reader(
                                unittests_reader,
                            )
                            .unwrap();
                        for unittest in unittests {
                            let obj = unittest.inputs[0].clone();
                            if obj == serde_json::Value::Null {
                                continue;
                            }
                            let obj_once = serde_json::to_value(
                                    serde_json::from_value::<Phrase>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<Phrase>(obj_once.clone()).unwrap(),
                                )
                                .unwrap();
                            assert_json_diff::assert_json_eq!(obj_once, obj_twice);
                        }
                    }
                }
            }
        }
    }
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
#[cfg(test)]
mod TextRank_find_phrases_harness {
    use super::*;
    #[test]
    fn find_phrases__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.FindPhrases.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Phrase>);
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { find_phrases__with_callees_mocked(&*input_state.0) }
                    #[cfg(not(feature = "mock"))] { find_phrases(&*input_state.0) }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn find_phrases__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Phrase>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.FindPhrases.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
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
#[cfg(test)]
mod TextRank_Sentence_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "TextRank_Sentence_roundtrip"]
        fn Sentence__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn Sentence__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!(
                                "(*{}).", "github.com-DavidBelicza-TextRank.Sentence"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-DavidBelicza-TextRank.Sentence"
                                ),
                            )
                    {
                        let unittests_file: std::fs::File = std::fs::File::open(
                                entry.path(),
                            )
                            .unwrap();
                        let unittests_reader = std::io::BufReader::new(unittests_file);
                        let unittests: Vec<ExecutionData> = serde_json::from_reader(
                                unittests_reader,
                            )
                            .unwrap();
                        for unittest in unittests {
                            let obj = unittest.inputs[0].clone();
                            if obj == serde_json::Value::Null {
                                continue;
                            }
                            let obj_once = serde_json::to_value(
                                    serde_json::from_value::<Sentence>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<Sentence>(obj_once.clone())
                                        .unwrap(),
                                )
                                .unwrap();
                            assert_json_diff::assert_json_eq!(obj_once, obj_twice);
                        }
                    }
                }
            }
        }
    }
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
#[cfg(test)]
mod TextRank_find_sentences_by_phrases_harness {
    use super::*;
    #[test]
    fn find_sentences_by_phrases__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSentencesByPhrases.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Option<Box<Rank>>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    {
                        (find_sentences_by_phrases__with_callees_mocked(
                            input_state.0.as_deref(),
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (find_sentences_by_phrases(
                            input_state.0.as_deref(),
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.1).unwrap(),
                        serde_json::to_value(& input_state_mutated.1).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn find_sentences_by_phrases__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Option<Box<Rank>>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSentencesByPhrases.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
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
#[cfg(test)]
mod TextRank_SingleWord_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "TextRank_SingleWord_roundtrip"]
        fn SingleWord__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn SingleWord__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!(
                                "(*{}).", "github.com-DavidBelicza-TextRank.SingleWord"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-DavidBelicza-TextRank.SingleWord"
                                ),
                            )
                    {
                        let unittests_file: std::fs::File = std::fs::File::open(
                                entry.path(),
                            )
                            .unwrap();
                        let unittests_reader = std::io::BufReader::new(unittests_file);
                        let unittests: Vec<ExecutionData> = serde_json::from_reader(
                                unittests_reader,
                            )
                            .unwrap();
                        for unittest in unittests {
                            let obj = unittest.inputs[0].clone();
                            if obj == serde_json::Value::Null {
                                continue;
                            }
                            let obj_once = serde_json::to_value(
                                    serde_json::from_value::<SingleWord>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<SingleWord>(obj_once.clone())
                                        .unwrap(),
                                )
                                .unwrap();
                            assert_json_diff::assert_json_eq!(obj_once, obj_twice);
                        }
                    }
                }
            }
        }
    }
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
#[cfg(test)]
mod TextRank_find_single_words_harness {
    use super::*;
    #[test]
    fn find_single_words__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSingleWords.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "serde_with::DefaultOnNull")]
            Vec<SingleWord>,
        );
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { find_single_words__with_callees_mocked(&*input_state.0) }
                    #[cfg(not(feature = "mock"))] { find_single_words(&*input_state.0) }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn find_single_words__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "serde_with::DefaultOnNull")]
            Vec<SingleWord>,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSingleWords.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
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
#[cfg(test)]
mod TextRank_find_sentences_harness {
    use super::*;
    #[test]
    fn find_sentences__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSentences.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, i32, i32);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    {
                        (find_sentences__with_callees_mocked(
                            &*input_state.0,
                            input_state.1,
                            input_state.2,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (find_sentences(&*input_state.0, input_state.1, input_state.2))
                            .unwrap()
                    }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn find_sentences__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSentences.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
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
#[cfg(test)]
mod TextRank_find_sentences_from_harness {
    use super::*;
    #[test]
    fn find_sentences_from__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSentencesFrom.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, i32, i32);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    {
                        find_sentences_from__with_callees_mocked(
                            &*input_state.0,
                            input_state.1,
                            input_state.2,
                        )
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        find_sentences_from(
                            &*input_state.0,
                            input_state.1,
                            input_state.2,
                        )
                    }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn find_sentences_from__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.FindSentencesFrom.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}
