use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::rank::Rank;
use std::boxed::Box;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct TextRank {
    #[serde(rename = "rank")]
    pub(crate) rank: Box<Rank>,
}

use crate::sorting::find_phrases;
use crate::sorting::Phrase;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn find_phrases(&self) -> Vec<Phrase> {
        find_phrases(&self.rank)
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_phrases(&self) -> Vec<Phrase> {
        extern "C" {
            #[link_name = "TextRank_text_rank___find_phrases__ground_truth"]
            fn TextRank_find_phrases__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a TextRank);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Phrase>);
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(TextRank_find_phrases__foreign(ser(&input_state_in)))
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
impl TextRank {
    pub fn find_phrases__with_callees_mocked(&self) -> Vec<Phrase> {
        find_phrases(&self.rank)
    }
}

use crate::sorting::find_sentences_by_phrases;
use crate::sorting::Sentence;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn find_sentences_by_phrase_chain(
        &self,
        phrases: &[String],
    ) -> Result<Vec<Sentence>> {
        find_sentences_by_phrases(Some(&self.rank), phrases)
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_sentences_by_phrase_chain(
        &self,
        phrases: &[String],
    ) -> Result<Vec<Sentence>> {
        extern "C" {
            #[link_name = "TextRank_text_rank___find_sentences_by_phrase_chain__ground_truth"]
            fn TextRank_find_sentences_by_phrase_chain__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a TextRank, &'b [String]);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        let input_state_in = InputStateIn(self, phrases);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                TextRank_find_sentences_by_phrase_chain__foreign(
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
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_sentences_by_phrase_chain__with_callees_mocked(
        &self,
        phrases: &[String],
    ) -> Result<Vec<Sentence>> {
        find_sentences_by_phrases(Some(&self.rank), phrases)
    }
}

use crate::sorting::find_sentences;
use crate::sorting::BY_RELATION;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn find_sentences_by_relation_weight(
        &self,
        limit: i32,
    ) -> Result<Vec<Sentence>, Error> {
        find_sentences(&self.rank, BY_RELATION, limit)
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_sentences_by_relation_weight(
        &self,
        limit: i32,
    ) -> Result<Vec<Sentence>, Error> {
        extern "C" {
            #[link_name = "TextRank_text_rank___find_sentences_by_relation_weight__ground_truth"]
            fn TextRank_find_sentences_by_relation_weight__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a TextRank, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        let input_state_in = InputStateIn(self, limit);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                TextRank_find_sentences_by_relation_weight__foreign(
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
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_sentences_by_relation_weight__with_callees_mocked(
        &self,
        limit: i32,
    ) -> Result<Vec<Sentence>, Error> {
        find_sentences(&self.rank, BY_RELATION, limit)
    }
}

use crate::sorting::BY_QTY;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn find_sentences_by_word_qty_weight(
        &self,
        limit: i32,
    ) -> Result<Vec<Sentence>, Error> {
        find_sentences(&self.rank, BY_QTY, limit)
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_sentences_by_word_qty_weight(
        &self,
        limit: i32,
    ) -> Result<Vec<Sentence>, Error> {
        extern "C" {
            #[link_name = "TextRank_text_rank___find_sentences_by_word_qty_weight__ground_truth"]
            fn TextRank_find_sentences_by_word_qty_weight__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a TextRank, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        let input_state_in = InputStateIn(self, limit);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                TextRank_find_sentences_by_word_qty_weight__foreign(
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
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_sentences_by_word_qty_weight__with_callees_mocked(
        &self,
        limit: i32,
    ) -> Result<Vec<Sentence>, Error> {
        find_sentences(&self.rank, BY_QTY, limit)
    }
}

use crate::sorting::find_sentences_from;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn find_sentences_from(&self, sentence_id: i32, limit: i32) -> Vec<Sentence> {
        find_sentences_from(&self.rank, sentence_id, limit)
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_sentences_from(&self, sentence_id: i32, limit: i32) -> Vec<Sentence> {
        extern "C" {
            #[link_name = "TextRank_text_rank___find_sentences_from__ground_truth"]
            fn TextRank_find_sentences_from__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a TextRank, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Sentence>);
        let input_state_in = InputStateIn(self, sentence_id, limit);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                TextRank_find_sentences_from__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                ),
            )
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
impl TextRank {
    pub fn find_sentences_from__with_callees_mocked(
        &self,
        sentence_id: i32,
        limit: i32,
    ) -> Vec<Sentence> {
        find_sentences_from(&self.rank, sentence_id, limit)
    }
}

use crate::sorting::find_single_words;
use crate::sorting::SingleWord;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn find_single_words(&self) -> Vec<SingleWord> {
        find_single_words(&self.rank)
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn find_single_words(&self) -> Vec<SingleWord> {
        extern "C" {
            #[link_name = "TextRank_text_rank___find_single_words__ground_truth"]
            fn TextRank_find_single_words__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a TextRank);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "serde_with::DefaultOnNull")]
            Vec<SingleWord>,
        );
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(TextRank_find_single_words__foreign(ser(&input_state_in)))
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
impl TextRank {
    pub fn find_single_words__with_callees_mocked(&self) -> Vec<SingleWord> {
        find_single_words(&self.rank)
    }
}

use crate::rule::Rule;
use crate::language::Language;
use crate::tokenizer::tokenize_text;
use crate::builder::text_to_rank;
use crate::text::Text;
use crate::text::ParsedSentence;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn populate(
        &mut self,
        text: &str,
        lang: &dyn Language,
        rule: &dyn Rule,
    ) -> Result<(), Error> {
        let parsed_text = tokenize_text(text, rule)?;
        for sentence in parsed_text.get_sentences() {
            text_to_rank(sentence, lang, &mut self.rank)?;
        }
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn populate(
        &mut self,
        text: &str,
        lang: &dyn Language,
        rule: &dyn Rule,
    ) -> Result<(), Error> {
        extern "C" {
            #[link_name = "TextRank_text_rank___populate__ground_truth"]
            fn TextRank_populate__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b, 'c, 'd>(
            &'a mut TextRank,
            &'b str,
            &'c dyn Language,
            &'d dyn Rule,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>, Box<str>, Box<dyn Language>, Box<dyn Rule>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, text, lang, rule);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                TextRank_populate__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                    ser(&params[3]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 4usize);
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
impl TextRank {
    pub fn populate__with_callees_mocked(
        &mut self,
        text: &str,
        lang: &dyn Language,
        rule: &dyn Rule,
    ) -> Result<(), Error> {
        let parsed_text = tokenize_text(text, rule)?;
        for sentence in parsed_text.get_sentences() {
            text_to_rank(sentence, lang, &mut self.rank)?;
        }
        Ok(())
    }
}

use crate::algorithm::Algorithm;
use crate::ranking::calculate;
#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn ranking(&mut self, algorithm: &dyn Algorithm) -> Result<(), anyhow::Error> {
        calculate(&mut self.rank, algorithm)?;
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn ranking(&mut self, algorithm: &dyn Algorithm) -> Result<(), anyhow::Error> {
        extern "C" {
            #[link_name = "TextRank_text_rank___ranking__ground_truth"]
            fn TextRank_ranking__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a mut TextRank, &'b dyn Algorithm);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<TextRank>, Box<dyn Algorithm>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, algorithm);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(TextRank_ranking__foreign(ser(&params[0]), ser(&params[1])))
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
impl TextRank {
    pub fn ranking__with_callees_mocked(
        &mut self,
        algorithm: &dyn Algorithm,
    ) -> Result<(), anyhow::Error> {
        calculate(&mut self.rank, algorithm)?;
        Ok(())
    }
}

use crate::algorithm::new_algorithm_default;
use crate::algorithm::AlgorithmDefault;
#[cfg(not(feature = "mock"))]
pub fn new_default_algorithm() -> Result<AlgorithmDefault> {
    new_algorithm_default()
}
#[cfg(feature = "mock")]
pub fn new_default_algorithm() -> Result<AlgorithmDefault> {
    extern "C" {
        #[link_name = "TextRank_new_default_algorithm__ground_truth"]
        fn new_default_algorithm__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(AlgorithmDefault);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe {
        de::<ForeignExecution>(new_default_algorithm__foreign())
    };
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
        let output = output_state.0;
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn new_default_algorithm__with_callees_mocked() -> Result<AlgorithmDefault> {
    new_algorithm_default()
}

use crate::language::LanguageDefault;
use crate::language::new_language;
#[cfg(not(feature = "mock"))]
pub fn new_default_language() -> Result<LanguageDefault, Error> {
    new_language()
}
#[cfg(feature = "mock")]
pub fn new_default_language() -> Result<LanguageDefault, Error> {
    extern "C" {
        #[link_name = "TextRank_new_default_language__ground_truth"]
        fn new_default_language__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(LanguageDefault);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe {
        de::<ForeignExecution>(new_default_language__foreign())
    };
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
        let output = output_state.0;
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn new_default_language__with_callees_mocked() -> Result<LanguageDefault, Error> {
    new_language()
}

use crate::rule::RuleDefault;
#[cfg(not(feature = "mock"))]
impl RuleDefault {
    pub fn new_default_rule() -> Result<RuleDefault> {
        RuleDefault::new()
    }
}
#[cfg(feature = "mock")]
impl RuleDefault {
    pub fn new_default_rule() -> Result<RuleDefault> {
        extern "C" {
            #[link_name = "TextRank_new_default_rule__ground_truth"]
            fn RuleDefault_new_default_rule__foreign() -> JSONObject;
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
            de::<ForeignExecution>(RuleDefault_new_default_rule__foreign())
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
    pub fn new_default_rule__with_callees_mocked() -> Result<RuleDefault> {
        RuleDefault::new()
    }
}

#[cfg(not(feature = "mock"))]
impl TextRank {
    pub fn new_text_rank() -> TextRank {
        TextRank {
            rank: Box::new(Rank::new_rank()),
        }
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn new_text_rank() -> TextRank {
        extern "C" {
            #[link_name = "TextRank_new_text_rank__ground_truth"]
            fn TextRank_new_text_rank__foreign() -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(TextRank);
        let input_state_in = InputStateIn();
        let foreign_execution = unsafe {
            de::<ForeignExecution>(TextRank_new_text_rank__foreign())
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
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl TextRank {
    pub fn new_text_rank__with_callees_mocked() -> TextRank {
        TextRank {
            rank: Box::new(Rank::new_rank()),
        }
    }
}

