use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashMap;
use crate::text::ParsedSentence;
use crate::rank::Rank;
#[cfg(not(feature = "mock"))]
pub(crate) fn add_sentence(
    ranks: &mut Rank,
    sentence: ParsedSentence,
) -> Result<i32, Error> {
    let sentence_id = ranks.sentence_map.len() as i32;
    ranks.sentence_map.insert(sentence_id, sentence.original.clone());
    Ok(sentence_id)
}
#[cfg(feature = "mock")]
pub(crate) fn add_sentence(
    ranks: &mut Rank,
    sentence: ParsedSentence,
) -> Result<i32, Error> {
    extern "C" {
        #[link_name = "TextRank_add_sentence__ground_truth"]
        fn add_sentence__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a mut Rank, ParsedSentence);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, ParsedSentence);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(ranks, sentence);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(add_sentence__foreign(ser(&params[0]), ser(&params[1])))
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
        *ranks = *input_state_mutated.0;
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
pub(crate) fn add_sentence__with_callees_mocked(
    ranks: &mut Rank,
    sentence: ParsedSentence,
) -> Result<i32, Error> {
    let sentence_id = ranks.sentence_map.len() as i32;
    ranks.sentence_map.insert(sentence_id, sentence.original.clone());
    Ok(sentence_id)
}

use crate::__synthetic::__Synth0__find_root_word;
use crate::language::Language;
use crate::relation::Relation;
use crate::__synthetic::__Synth2__is_stop_word;
#[cfg(not(feature = "mock"))]
pub(crate) fn add_word(
    ranks: &mut Rank,
    words: &[String],
    lang: &dyn Language,
    sentence_id: i32,
) -> Result<(), Error> {
    let mut prev_word_id = -1;
    for word in words.iter().filter(|w| !lang.is_stop_word(w.as_str())) {
        let (found, root_word) = lang.find_root_word(word.as_str());
        let word = if found { root_word } else { word.clone() };
        let cur_word_id = if ranks.is_word_exist(&word) {
            ranks.update_word(&word, prev_word_id, sentence_id)?
        } else {
            ranks.add_new_word(&word, prev_word_id, sentence_id)
        };
        ranks.relation.add_relation(cur_word_id, prev_word_id, sentence_id)?;
        ranks.update_right_connection(prev_word_id, cur_word_id)?;
        prev_word_id = cur_word_id;
    }
    Ok(())
}
#[cfg(feature = "mock")]
pub(crate) fn add_word(
    ranks: &mut Rank,
    words: &[String],
    lang: &dyn Language,
    sentence_id: i32,
) -> Result<(), Error> {
    extern "C" {
        #[link_name = "TextRank_add_word__ground_truth"]
        fn add_word__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b, 'c>(&'a mut Rank, &'b [String], &'c dyn Language, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, Box<[String]>, Box<dyn Language>, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(ranks, words, lang, sentence_id);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(
            add_word__foreign(
                ser(&params[0]),
                ser(&params[1]),
                ser(&params[2]),
                ser(&params[3]),
            ),
        )
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 4usize);
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
        *ranks = *input_state_mutated.0;
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub(crate) fn add_word__with_callees_mocked(
    ranks: &mut Rank,
    words: &[String],
    lang: &dyn Language,
    sentence_id: i32,
) -> Result<(), Error> {
    let mut prev_word_id = -1;
    for word in words.iter().filter(|w| !lang.is_stop_word(w.as_str())) {
        let (found, root_word) = lang.find_root_word(word.as_str());
        let word = if found { root_word } else { word.clone() };
        let cur_word_id = if ranks.is_word_exist(&word) {
            ranks.update_word(&word, prev_word_id, sentence_id)?
        } else {
            ranks.add_new_word(&word, prev_word_id, sentence_id)
        };
        ranks.relation.add_relation(cur_word_id, prev_word_id, sentence_id)?;
        ranks.update_right_connection(prev_word_id, cur_word_id)?;
        prev_word_id = cur_word_id;
    }
    Ok(())
}

#[cfg(not(feature = "mock"))]
pub fn text_to_rank(
    sentence: ParsedSentence,
    lang: &dyn Language,
    ranks: &mut Rank,
) -> Result<(), Error> {
    let sentence_id = add_sentence(ranks, sentence.clone())?;
    add_word(ranks, sentence.get_words()?, lang, sentence_id)?;
    Ok(())
}
#[cfg(feature = "mock")]
pub fn text_to_rank(
    sentence: ParsedSentence,
    lang: &dyn Language,
    ranks: &mut Rank,
) -> Result<(), Error> {
    extern "C" {
        #[link_name = "TextRank_text_to_rank__ground_truth"]
        fn text_to_rank__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(ParsedSentence, &'a dyn Language, &'b mut Rank);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(ParsedSentence, Box<dyn Language>, Box<Rank>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(sentence, lang, ranks);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(text_to_rank__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
        *ranks = *input_state_mutated.2;
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn text_to_rank__with_callees_mocked(
    sentence: ParsedSentence,
    lang: &dyn Language,
    ranks: &mut Rank,
) -> Result<(), Error> {
    let sentence_id = add_sentence(ranks, sentence.clone())?;
    add_word(ranks, sentence.get_words()?, lang, sentence_id)?;
    Ok(())
}

