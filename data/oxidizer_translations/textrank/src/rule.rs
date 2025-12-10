#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::vec::Vec;
//Translated from: github.com/DavidBelicza/TextRank.RuleDefault
#[derive(Default)]#[derive(Clone)]pub struct RuleDefault {
    pub(crate) word_separators: Vec<String>,
    pub(crate) sentence_separators: Vec<String>,
}


//Translated from: github.com/DavidBelicza/TextRank.Rule
pub trait Rule: crate::__synthetic::__Synth3__is_word_separator + crate::__synthetic::__Synth1__is_sentence_separator {}
impl<T> Rule for T where T: crate::__synthetic::__Synth3__is_word_separator + crate::__synthetic::__Synth1__is_sentence_separator {}

use crate::__synthetic::__Synth1__is_sentence_separator;
//Translated from: (*github.com/DavidBelicza/TextRank.RuleDefault).IsSentenceSeparator
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
use std::string::ToString;
use crate::__synthetic::__Synth3__is_word_separator;
//Translated from: (*github.com/DavidBelicza/TextRank.RuleDefault).IsWordSeparator
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

//Translated from: github.com/DavidBelicza/TextRank.NewRule
impl RuleDefault {
    pub fn new() -> Result<RuleDefault> {
        let word_separators = vec![
            " ".to_string(),
            ",".to_string(),
            "'".to_string(),
            "'".to_string(),
            "\"".to_string(),
            ")".to_string(),
            "(".to_string(),
            "[".to_string(),
            "]".to_string(),
            "{".to_string(),
            "}".to_string(),
            "\"".to_string(),
            ";".to_string(),
            "\n".to_string(),
            ">".to_string(),
            "<".to_string(),
            "%".to_string(),
            "@".to_string(),
            "&".to_string(),
            "=".to_string(),
            "#".to_string(),
        ];

        let sentence_separators = vec![
            "!".to_string(),
            ".".to_string(),
            "?".to_string(),
        ];

        Ok(RuleDefault {
            word_separators,
            sentence_separators,
        })
    }
}
