use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct ParsedSentence {
    #[serde(rename = "original")]
    pub(crate) original: String,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "words")]
    pub(crate) words: Vec<String>,
}

#[cfg(not(feature = "mock"))]
impl ParsedSentence {
    pub fn get_original(&self) -> &str {
        &self.original
    }
}
#[cfg(feature = "mock")]
impl ParsedSentence {
    pub fn get_original(&self) -> &str {
        &self.original
    }
}
#[cfg(feature = "mock")]
impl ParsedSentence {
    pub fn get_original__with_callees_mocked(&self) -> &str {
        &self.original
    }
}

#[cfg(not(feature = "mock"))]
impl ParsedSentence {
    pub fn get_words(&self) -> Result<&Vec<String>> {
        Ok(&self.words)
    }
}
#[cfg(feature = "mock")]
impl ParsedSentence {
    pub fn get_words(&self) -> Result<&Vec<String>> {
        Ok(&self.words)
    }
}
#[cfg(feature = "mock")]
impl ParsedSentence {
    pub fn get_words__with_callees_mocked(&self) -> Result<&Vec<String>> {
        Ok(&self.words)
    }
}

use std::vec::Vec;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Text {
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "parsedSentences")]
    pub(crate) parsed_sentences: Vec<ParsedSentence>,
}

#[cfg(not(feature = "mock"))]
impl Text {
    pub fn append(&mut self, raw_sentence: &str, words: &[String]) -> Result<()> {
        if !words.is_empty() {
            let parsed_sentence = ParsedSentence {
                original: raw_sentence.to_owned(),
                words: words.to_vec(),
            };
            self.parsed_sentences.push(parsed_sentence);
        }
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl Text {
    pub fn append(&mut self, raw_sentence: &str, words: &[String]) -> Result<()> {
        extern "C" {
            #[link_name = "TextRank_text___append__ground_truth"]
            fn Text_append__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b, 'c>(&'a mut Text, &'b str, &'c [String]);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Text>, Box<str>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, raw_sentence, words);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(Text_append__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 3usize);
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
            *self = *input_state_mutated.0;
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
}
#[cfg(feature = "mock")]
impl Text {
    pub fn append__with_callees_mocked(
        &mut self,
        raw_sentence: &str,
        words: &[String],
    ) -> Result<()> {
        if !words.is_empty() {
            let parsed_sentence = ParsedSentence {
                original: raw_sentence.to_owned(),
                words: words.to_vec(),
            };
            self.parsed_sentences.push(parsed_sentence);
        }
        Ok(())
    }
}

#[cfg(not(feature = "mock"))]
impl Text {
    pub fn get_sentences(&self) -> Vec<ParsedSentence> {
        self.parsed_sentences.clone()
    }
}
#[cfg(feature = "mock")]
impl Text {
    pub fn get_sentences(&self) -> Vec<ParsedSentence> {
        extern "C" {
            #[link_name = "TextRank_text___get_sentences__ground_truth"]
            fn Text_get_sentences__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a Text);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Text>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "serde_with::DefaultOnNull")]
            Vec<ParsedSentence>,
        );
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(Text_get_sentences__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
impl Text {
    pub fn get_sentences__with_callees_mocked(&self) -> Vec<ParsedSentence> {
        self.parsed_sentences.clone()
    }
}

