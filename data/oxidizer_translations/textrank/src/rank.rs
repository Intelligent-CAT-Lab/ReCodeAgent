#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
//Translated from: github.com/DavidBelicza/TextRank.Word
#[derive(Default)]#[derive(Clone)]pub struct Word {
    pub id: i32,
    pub sentence_ids: Vec<i32>,
    pub connection_left: HashMap<i32, i32>,
    pub connection_right: HashMap<i32, i32>,
    pub token: String,
    pub qty: i32,
    pub weight: f32,
}

use crate::relation::Relation;
//Translated from: github.com/DavidBelicza/TextRank.Rank
#[derive(Default)]#[derive(Clone)]pub struct Rank {
    pub max: f32,
    pub min: f32,
    pub relation: Relation,
    pub sentence_map: HashMap<i32, String>,
    pub words: HashMap<i32, Box<Word>>,
    pub word_val_id: HashMap<String, i32>,
}


//Translated from: (*github.com/DavidBelicza/TextRank.Rank).AddNewWord
impl Rank {
    pub fn add_new_word(&mut self, word: &str, prev_word_idx: i32, sentence_id: i32) -> i32 {
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

//Translated from: (*github.com/DavidBelicza/TextRank.Rank).IsWordExist
impl Rank {
    pub fn is_word_exist(&self, word: &str) -> bool {
        self.word_val_id.contains_key(word)
    }
}

//Translated from: (*github.com/DavidBelicza/TextRank.Rank).UpdateRightConnection
impl Rank {
    pub fn update_right_connection(&mut self, word_id: i32, right_word_id: i32) -> Result<()> {
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
//Translated from: (*github.com/DavidBelicza/TextRank.Rank).UpdateWord
impl Rank {
    pub fn update_word(&mut self, word: &str, prev_word_idx: i32, sentence_id: i32) -> Result<i32> {
        let word_id = *self.word_val_id.entry(word.to_string()).or_insert_with(|| {
            let id = self.words.len() as i32;
            self.words.insert(id, Box::new(Word::default()));
            id
        });

        let word = self.words.get_mut(&word_id).ok_or_else(|| anyhow::anyhow!("Word not found"))?;

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

//Translated from: github.com/DavidBelicza/TextRank.NewRank
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
