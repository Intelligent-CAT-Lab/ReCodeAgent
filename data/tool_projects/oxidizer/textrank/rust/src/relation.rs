use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::vec::Vec;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Score {
    #[serde(rename = "Qty")]
    pub qty: i32,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Weight")]
    pub weight: f32,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "SentenceIDs")]
    pub sentence_ids: Vec<i32>,
}

use std::collections::HashMap;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Relation {
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Max")]
    pub max: f32,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
    #[serde(rename = "Min")]
    pub min: f32,
    #[serde(rename = "Node")]
    pub node: HashMap<i32, HashMap<i32, Score>>,
}

#[cfg(not(feature = "mock"))]
impl Relation {
    pub(crate) fn create_relation(&mut self, x: i32, y: i32, sentence_id: i32) {
        self.node.entry(x).or_insert(HashMap::new());
        self.node
            .get_mut(&x)
            .unwrap()
            .insert(
                y,
                Score {
                    qty: 1,
                    weight: 0.0,
                    sentence_ids: vec![sentence_id],
                },
            );
    }
}
#[cfg(feature = "mock")]
impl Relation {
    pub(crate) fn create_relation(&mut self, x: i32, y: i32, sentence_id: i32) {
        extern "C" {
            #[link_name = "TextRank_relation___create_relation__ground_truth"]
            fn Relation_create_relation__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut Relation, i32, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Relation>, i32, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, x, y, sentence_id);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                Relation_create_relation__foreign(
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
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl Relation {
    pub(crate) fn create_relation__with_callees_mocked(
        &mut self,
        x: i32,
        y: i32,
        sentence_id: i32,
    ) {
        self.node.entry(x).or_insert(HashMap::new());
        self.node
            .get_mut(&x)
            .unwrap()
            .insert(
                y,
                Score {
                    qty: 1,
                    weight: 0.0,
                    sentence_ids: vec![sentence_id],
                },
            );
    }
}

#[cfg(not(feature = "mock"))]
impl Relation {
    pub(crate) fn extend_relation(
        &mut self,
        x: i32,
        y: i32,
        r: bool,
        sentence_id: i32,
    ) -> Result<bool> {
        if let Some(node_x) = self.node.get_mut(&x) {
            node_x
                .insert(
                    y,
                    Score {
                        qty: 1,
                        weight: 0.0,
                        sentence_ids: vec![sentence_id],
                    },
                );
            Ok(true)
        } else if r {
            self.extend_relation(y, x, false, sentence_id)
        } else {
            Ok(false)
        }
    }
}
#[cfg(feature = "mock")]
impl Relation {
    pub(crate) fn extend_relation(
        &mut self,
        x: i32,
        y: i32,
        r: bool,
        sentence_id: i32,
    ) -> Result<bool> {
        extern "C" {
            #[link_name = "TextRank_relation___extend_relation__ground_truth"]
            fn Relation_extend_relation__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut Relation, i32, i32, bool, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Relation>, i32, i32, bool, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, x, y, r, sentence_id);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                Relation_extend_relation__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                    ser(&params[3]),
                    ser(&params[4]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 5usize);
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
impl Relation {
    pub(crate) fn extend_relation__with_callees_mocked(
        &mut self,
        x: i32,
        y: i32,
        r: bool,
        sentence_id: i32,
    ) -> Result<bool> {
        if let Some(node_x) = self.node.get_mut(&x) {
            node_x
                .insert(
                    y,
                    Score {
                        qty: 1,
                        weight: 0.0,
                        sentence_ids: vec![sentence_id],
                    },
                );
            Ok(true)
        } else if r {
            self.extend_relation(y, x, false, sentence_id)
        } else {
            Ok(false)
        }
    }
}

#[cfg(not(feature = "mock"))]
impl Relation {
    pub(crate) fn update_relation(
        &mut self,
        x: i32,
        y: i32,
        r: bool,
        sentence_id: i32,
    ) -> bool {
        if let Some(score) = self.node.get_mut(&x).and_then(|m| m.get_mut(&y)) {
            score.qty += 1;
            score.sentence_ids.push(sentence_id);
            true
        } else if r {
            self.update_relation(y, x, false, sentence_id)
        } else {
            false
        }
    }
}
#[cfg(feature = "mock")]
impl Relation {
    pub(crate) fn update_relation(
        &mut self,
        x: i32,
        y: i32,
        r: bool,
        sentence_id: i32,
    ) -> bool {
        extern "C" {
            #[link_name = "TextRank_relation___update_relation__ground_truth"]
            fn Relation_update_relation__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut Relation, i32, i32, bool, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Relation>, i32, i32, bool, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, x, y, r, sentence_id);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                Relation_update_relation__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                    ser(&params[3]),
                    ser(&params[4]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 5usize);
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
impl Relation {
    pub(crate) fn update_relation__with_callees_mocked(
        &mut self,
        x: i32,
        y: i32,
        r: bool,
        sentence_id: i32,
    ) -> bool {
        if let Some(score) = self.node.get_mut(&x).and_then(|m| m.get_mut(&y)) {
            score.qty += 1;
            score.sentence_ids.push(sentence_id);
            true
        } else if r {
            self.update_relation(y, x, false, sentence_id)
        } else {
            false
        }
    }
}

#[cfg(not(feature = "mock"))]
impl Relation {
    pub fn add_relation(
        &mut self,
        word_id: i32,
        related_word_id: i32,
        sentence_id: i32,
    ) -> Result<()> {
        if related_word_id == -1 {
            return Ok(());
        }
        if self.update_relation(related_word_id, word_id, true, sentence_id) {
            return Ok(());
        }
        if self.extend_relation(word_id, related_word_id, true, sentence_id)? {
            return Ok(());
        }
        self.create_relation(word_id, related_word_id, sentence_id);
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl Relation {
    pub fn add_relation(
        &mut self,
        word_id: i32,
        related_word_id: i32,
        sentence_id: i32,
    ) -> Result<()> {
        extern "C" {
            #[link_name = "TextRank_relation___add_relation__ground_truth"]
            fn Relation_add_relation__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut Relation, i32, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Relation>, i32, i32, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, word_id, related_word_id, sentence_id);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                Relation_add_relation__foreign(
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
impl Relation {
    pub fn add_relation__with_callees_mocked(
        &mut self,
        word_id: i32,
        related_word_id: i32,
        sentence_id: i32,
    ) -> Result<()> {
        if related_word_id == -1 {
            return Ok(());
        }
        if self.update_relation(related_word_id, word_id, true, sentence_id) {
            return Ok(());
        }
        if self.extend_relation(word_id, related_word_id, true, sentence_id)? {
            return Ok(());
        }
        self.create_relation(word_id, related_word_id, sentence_id);
        Ok(())
    }
}

