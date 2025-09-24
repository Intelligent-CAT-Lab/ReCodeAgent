use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashMap;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Word {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "SentenceIDs")]
    pub sentence_ids: Vec<i32>,
    #[serde(rename = "ConnectionLeft")]
    pub connection_left: HashMap<i32, i32>,
    #[serde(rename = "ConnectionRight")]
    pub connection_right: HashMap<i32, i32>,
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "Qty")]
    pub qty: i32,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Weight")]
    pub weight: f32,
}

use crate::relation::Relation;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Rank {
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Max")]
    pub max: f32,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Min")]
    pub min: f32,
    #[serde(rename = "Relation")]
    pub relation: Relation,
    #[serde(rename = "SentenceMap")]
    pub sentence_map: HashMap<i32, String>,
    #[serde(rename = "Words")]
    pub words: HashMap<i32, Box<Word>>,
    #[serde(rename = "WordValID")]
    pub word_val_id: HashMap<String, i32>,
}

#[cfg(not(feature = "mock"))]
impl Rank {
    pub fn add_new_word(
        &mut self,
        word: &str,
        prev_word_idx: i32,
        sentence_id: i32,
    ) -> i32 {
        let word_id = self.words.len() as i32;
        let mut connection_left = HashMap::new();
        if prev_word_idx >= 0 {
            connection_left.insert(prev_word_idx, 1);
        }
        let new_word = Word {
            id: word_id,
            sentence_ids: vec![sentence_id],
            connection_left,
            connection_right: HashMap::new(),
            token: word.to_string(),
            qty: 1,
            weight: 0.0,
        };
        self.words.insert(word_id, Box::new(new_word));
        self.word_val_id.insert(word.to_string(), word_id);
        word_id
    }
}
#[cfg(feature = "mock")]
impl Rank {
    pub fn add_new_word(
        &mut self,
        word: &str,
        prev_word_idx: i32,
        sentence_id: i32,
    ) -> i32 {
        extern "C" {
            #[link_name = "TextRank_rank___add_new_word__ground_truth"]
            fn Rank_add_new_word__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a mut Rank, &'b str, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Rank>, Box<str>, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let input_state_in = InputStateIn(self, word, prev_word_idx, sentence_id);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                Rank_add_new_word__foreign(
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
            let output = output_state.0;
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl Rank {
    pub fn add_new_word__with_callees_mocked(
        &mut self,
        word: &str,
        prev_word_idx: i32,
        sentence_id: i32,
    ) -> i32 {
        let word_id = self.words.len() as i32;
        let mut connection_left = HashMap::new();
        if prev_word_idx >= 0 {
            connection_left.insert(prev_word_idx, 1);
        }
        let new_word = Word {
            id: word_id,
            sentence_ids: vec![sentence_id],
            connection_left,
            connection_right: HashMap::new(),
            token: word.to_string(),
            qty: 1,
            weight: 0.0,
        };
        self.words.insert(word_id, Box::new(new_word));
        self.word_val_id.insert(word.to_string(), word_id);
        word_id
    }
}

#[cfg(not(feature = "mock"))]
impl Rank {
    pub fn is_word_exist(&self, word: &str) -> bool {
        self.word_val_id.contains_key(word)
    }
}
#[cfg(feature = "mock")]
impl Rank {
    pub fn is_word_exist(&self, word: &str) -> bool {
        extern "C" {
            #[link_name = "TextRank_rank___is_word_exist__ground_truth"]
            fn Rank_is_word_exist__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a Rank, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Rank>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, word);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(Rank_is_word_exist__foreign(ser(&params[0]), ser(&params[1])))
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
impl Rank {
    pub fn is_word_exist__with_callees_mocked(&self, word: &str) -> bool {
        self.word_val_id.contains_key(word)
    }
}

#[cfg(not(feature = "mock"))]
impl Rank {
    pub fn update_right_connection(
        &mut self,
        word_id: i32,
        right_word_id: i32,
    ) -> Result<()> {
        if word_id >= 0 {
            if let Some(word) = self.words.get_mut(&word_id) {
                let counter = word.connection_right.entry(right_word_id).or_insert(0);
                *counter += 1;
            }
        }
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl Rank {
    pub fn update_right_connection(
        &mut self,
        word_id: i32,
        right_word_id: i32,
    ) -> Result<()> {
        extern "C" {
            #[link_name = "TextRank_rank___update_right_connection__ground_truth"]
            fn Rank_update_right_connection__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut Rank, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Rank>, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, word_id, right_word_id);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                Rank_update_right_connection__foreign(
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
impl Rank {
    pub fn update_right_connection__with_callees_mocked(
        &mut self,
        word_id: i32,
        right_word_id: i32,
    ) -> Result<()> {
        if word_id >= 0 {
            if let Some(word) = self.words.get_mut(&word_id) {
                let counter = word.connection_right.entry(right_word_id).or_insert(0);
                *counter += 1;
            }
        }
        Ok(())
    }
}

use std::collections::hash_map::Entry;
#[cfg(not(feature = "mock"))]
impl Rank {
    pub fn update_word(
        &mut self,
        word: &str,
        prev_word_idx: i32,
        sentence_id: i32,
    ) -> Result<i32> {
        let word_id = *self
            .word_val_id
            .entry(word.to_string())
            .or_insert_with(|| {
                let id = self.words.len() as i32;
                self.words.insert(id, Box::new(Word::default()));
                id
            });
        let word = self
            .words
            .get_mut(&word_id)
            .ok_or_else(|| anyhow::anyhow!("Word not found"))?;
        if !word.sentence_ids.contains(&sentence_id) {
            word.sentence_ids.push(sentence_id);
        }
        word.qty += 1;
        if prev_word_idx >= 0 {
            *word.connection_left.entry(prev_word_idx as i32).or_insert(0) += 1;
        }
        Ok(word_id)
    }
}
#[cfg(feature = "mock")]
impl Rank {
    pub fn update_word(
        &mut self,
        word: &str,
        prev_word_idx: i32,
        sentence_id: i32,
    ) -> Result<i32> {
        extern "C" {
            #[link_name = "TextRank_rank___update_word__ground_truth"]
            fn Rank_update_word__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a mut Rank, &'b str, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Rank>, Box<str>, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let input_state_in = InputStateIn(self, word, prev_word_idx, sentence_id);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                Rank_update_word__foreign(
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
            let output = output_state.0;
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl Rank {
    pub fn update_word__with_callees_mocked(
        &mut self,
        word: &str,
        prev_word_idx: i32,
        sentence_id: i32,
    ) -> Result<i32> {
        let word_id = *self
            .word_val_id
            .entry(word.to_string())
            .or_insert_with(|| {
                let id = self.words.len() as i32;
                self.words.insert(id, Box::new(Word::default()));
                id
            });
        let word = self
            .words
            .get_mut(&word_id)
            .ok_or_else(|| anyhow::anyhow!("Word not found"))?;
        if !word.sentence_ids.contains(&sentence_id) {
            word.sentence_ids.push(sentence_id);
        }
        word.qty += 1;
        if prev_word_idx >= 0 {
            *word.connection_left.entry(prev_word_idx as i32).or_insert(0) += 1;
        }
        Ok(word_id)
    }
}

#[cfg(not(feature = "mock"))]
impl Rank {
    pub fn new_rank() -> Rank {
        Rank {
            max: 0.0,
            min: 0.0,
            relation: Relation {
                max: 0.0,
                min: 0.0,
                node: HashMap::new(),
            },
            sentence_map: HashMap::new(),
            words: HashMap::new(),
            word_val_id: HashMap::new(),
        }
    }
}
#[cfg(feature = "mock")]
impl Rank {
    pub fn new_rank() -> Rank {
        extern "C" {
            #[link_name = "TextRank_new_rank__ground_truth"]
            fn Rank_new_rank__foreign() -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Rank);
        let input_state_in = InputStateIn();
        let foreign_execution = unsafe {
            de::<ForeignExecution>(Rank_new_rank__foreign())
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
impl Rank {
    pub fn new_rank__with_callees_mocked() -> Rank {
        Rank {
            max: 0.0,
            min: 0.0,
            relation: Relation {
                max: 0.0,
                min: 0.0,
                node: HashMap::new(),
            },
            sentence_map: HashMap::new(),
            words: HashMap::new(),
            word_val_id: HashMap::new(),
        }
    }
}

