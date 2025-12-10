#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::vec::Vec;
//Translated from: github.com/DavidBelicza/TextRank.Score
#[derive(Default)]#[derive(Clone)]pub struct Score {
    pub qty: i32,
    pub weight: f32,
    pub sentence_ids: Vec<i32>,
}

use std::collections::HashMap;
//Translated from: github.com/DavidBelicza/TextRank.Relation
#[derive(Default)]#[derive(Clone)]pub struct Relation {
    pub max: f32,
    pub min: f32,
    pub node: HashMap<i32, HashMap<i32, Score>>,
}


//Translated from: (*github.com/DavidBelicza/TextRank.Relation).createRelation
impl Relation {
    pub(crate) fn create_relation(&mut self, x: i32, y: i32, sentence_id: i32) {
        self.node.entry(x).or_insert(HashMap::new());
        self.node.get_mut(&x).unwrap().insert(
            y,
            Score {
                qty: 1,
                weight: 0.0,
                sentence_ids: vec![sentence_id],
            },
        );
    }
}

//Translated from: (*github.com/DavidBelicza/TextRank.Relation).extendRelation
impl Relation {
    pub(crate) fn extend_relation(&mut self, x: i32, y: i32, r: bool, sentence_id: i32) -> Result<bool> {
        if let Some(node_x) = self.node.get_mut(&x) {
            node_x.insert(y, Score {
                qty: 1,
                weight: 0.0,
                sentence_ids: vec![sentence_id],
            });
            Ok(true)
        } else if r {
            self.extend_relation(y, x, false, sentence_id)
        } else {
            Ok(false)
        }
    }
}

//Translated from: (*github.com/DavidBelicza/TextRank.Relation).updateRelation
impl Relation {
    pub(crate) fn update_relation(&mut self, x: i32, y: i32, r: bool, sentence_id: i32) -> bool {
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

//Translated from: (*github.com/DavidBelicza/TextRank.Relation).AddRelation
impl Relation {
    pub fn add_relation(&mut self, word_id: i32, related_word_id: i32, sentence_id: i32) -> Result<()> {
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
