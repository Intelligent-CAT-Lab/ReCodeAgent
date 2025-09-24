use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::language::LanguageDefault;
use crate::textrank::TextRank;
use crate::rule::Rule;
use crate::language::Language;
use crate::textrank::new_default_language;
use crate::algorithm::Algorithm;
use crate::sorting::Phrase;
use std::collections::HashMap;
use crate::textrank::new_default_algorithm;
use crate::sorting::Sentence;
use crate::algorithm::AlgorithmDefault;
use crate::rule::RuleDefault;
use crate::sorting::SingleWord;
#[cfg(not(feature = "mock"))]
pub fn example() -> Result<(), Error> {
    let raw_text = "Your long raw text, it could be a book. Lorem ipsum...";
    let mut tr = TextRank::new_text_rank();
    let rule = RuleDefault::new_default_rule()?;
    let language = new_default_language()?;
    let algorithm_def = new_default_algorithm()?;
    tr.populate(&raw_text, &language, &rule)?;
    tr.ranking(&algorithm_def)?;
    let _ = tr.find_phrases();
    let _ = tr.find_single_words();
    let _ = tr.find_sentences_by_relation_weight(10)?;
    let _ = tr.find_sentences_by_word_qty_weight(10)?;
    let _ = tr.find_sentences_from(5, 10);
    let _ = tr
        .find_sentences_by_phrase_chain(
            &vec!["gnome".to_string(), "shell".to_string(), "extension".to_string()],
        )?;
    Ok(())
}
#[cfg(feature = "mock")]
pub fn example() -> Result<(), Error> {
    extern "C" {
        #[link_name = "TextRank_example__ground_truth"]
        fn example__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(example__foreign()) };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 0usize);
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
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn example__with_callees_mocked() -> Result<(), Error> {
    let raw_text = "Your long raw text, it could be a book. Lorem ipsum...";
    let mut tr = TextRank::new_text_rank();
    let rule = RuleDefault::new_default_rule()?;
    let language = new_default_language()?;
    let algorithm_def = new_default_algorithm()?;
    tr.populate(&raw_text, &language, &rule)?;
    tr.ranking(&algorithm_def)?;
    let _ = tr.find_phrases();
    let _ = tr.find_single_words();
    let _ = tr.find_sentences_by_relation_weight(10)?;
    let _ = tr.find_sentences_by_word_qty_weight(10)?;
    let _ = tr.find_sentences_from(5, 10);
    let _ = tr
        .find_sentences_by_phrase_chain(
            &vec!["gnome".to_string(), "shell".to_string(), "extension".to_string()],
        )?;
    Ok(())
}

