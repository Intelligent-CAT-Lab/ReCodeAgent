use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::vec::Vec;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct RuleDefault {
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "wordSeparators")]
    pub(crate) word_separators: Vec<String>,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "sentenceSeparators")]
    pub(crate) sentence_separators: Vec<String>,
}

#[typetag::serde(tag = "Type", content = "Value")]
pub trait Rule: crate::__synthetic::__Synth3__is_word_separator + crate::__synthetic::__Synth1__is_sentence_separator {}
use crate::__synthetic::__Synth1__is_sentence_separator;
#[cfg(not(feature = "mock"))]
impl __Synth1__is_sentence_separator for RuleDefault {
    fn is_sentence_separator(&self, c: char) -> bool {
        for separator in &self.sentence_separators {
            if separator.chars().next().unwrap() == c {
                return true;
            }
        }
        false
    }
}
#[cfg(feature = "mock")]
impl __Synth1__is_sentence_separator for RuleDefault {
    fn is_sentence_separator(&self, c: char) -> bool {
        extern "C" {
            #[link_name = "TextRank_rule_default___is_sentence_separator__ground_truth"]
            fn RuleDefault_is_sentence_separator__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a RuleDefault, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<RuleDefault>, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, c);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                RuleDefault_is_sentence_separator__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 2usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl RuleDefault {
    fn is_sentence_separator__with_callees_mocked(&self, c: char) -> bool {
        for separator in &self.sentence_separators {
            if separator.chars().next().unwrap() == c {
                return true;
            }
        }
        false
    }
}

use crate::__synthetic::__Synth3__is_word_separator;
use std::string::ToString;
#[cfg(not(feature = "mock"))]
impl __Synth3__is_word_separator for RuleDefault {
    fn is_word_separator(&self, c: char) -> bool {
        let chr = c.to_string();
        for val in &self.word_separators {
            if chr == *val {
                return true;
            }
        }
        self.is_sentence_separator(c)
    }
}
#[cfg(feature = "mock")]
impl __Synth3__is_word_separator for RuleDefault {
    fn is_word_separator(&self, c: char) -> bool {
        extern "C" {
            #[link_name = "TextRank_rule_default___is_word_separator__ground_truth"]
            fn RuleDefault_is_word_separator__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a RuleDefault, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<RuleDefault>, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, c);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(RuleDefault_is_word_separator__foreign(ser(&params[0]), ser(&params[1])))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 2usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl RuleDefault {
    fn is_word_separator__with_callees_mocked(&self, c: char) -> bool {
        let chr = c.to_string();
        for val in &self.word_separators {
            if chr == *val {
                return true;
            }
        }
        self.is_sentence_separator(c)
    }
}

#[typetag::serde(name = "RuleDefault")]
impl crate::rule::Rule for RuleDefault {}

#[cfg(not(feature = "mock"))]
impl RuleDefault {
    pub fn new() -> Result<RuleDefault> {
        let word_separators = vec![
            " ".to_string(), ",".to_string(), "'".to_string(), "'".to_string(), "\""
            .to_string(), ")".to_string(), "(".to_string(), "[".to_string(), "]"
            .to_string(), "{".to_string(), "}".to_string(), "\"".to_string(), ";"
            .to_string(), "\n".to_string(), ">".to_string(), "<".to_string(), "%"
            .to_string(), "@".to_string(), "&".to_string(), "=".to_string(), "#"
            .to_string(),
        ];
        let sentence_separators = vec![
            "!".to_string(), ".".to_string(), "?".to_string(),
        ];
        Ok(RuleDefault {
            word_separators,
            sentence_separators,
        })
    }
}
#[cfg(feature = "mock")]
impl RuleDefault {
    pub fn new() -> Result<RuleDefault> {
        extern "C" {
            #[link_name = "TextRank_new_rule__ground_truth"]
            fn RuleDefault_new__foreign() -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(RuleDefault);
        let input_state_in = InputStateIn();
        let foreign_execution = unsafe {
            de::<ForeignExecution>(RuleDefault_new__foreign())
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 0usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl RuleDefault {
    pub fn new__with_callees_mocked() -> Result<RuleDefault> {
        let word_separators = vec![
            " ".to_string(), ",".to_string(), "'".to_string(), "'".to_string(), "\""
            .to_string(), ")".to_string(), "(".to_string(), "[".to_string(), "]"
            .to_string(), "{".to_string(), "}".to_string(), "\"".to_string(), ";"
            .to_string(), "\n".to_string(), ">".to_string(), "<".to_string(), "%"
            .to_string(), "@".to_string(), "&".to_string(), "=".to_string(), "#"
            .to_string(),
        ];
        let sentence_separators = vec![
            "!".to_string(), ".".to_string(), "?".to_string(),
        ];
        Ok(RuleDefault {
            word_separators,
            sentence_separators,
        })
    }
}

