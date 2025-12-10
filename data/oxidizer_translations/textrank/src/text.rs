#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/DavidBelicza/TextRank.ParsedSentence
#[derive(Default)]#[derive(Clone)]pub struct ParsedSentence {
    pub(crate) original: String,
    pub(crate) words: Vec<String>,
}


//Translated from: (*github.com/DavidBelicza/TextRank.ParsedSentence).GetOriginal
impl ParsedSentence {
    pub fn get_original(&self) -> &str {
        &self.original
    }
}

//Translated from: (*github.com/DavidBelicza/TextRank.ParsedSentence).GetWords
impl ParsedSentence {
    pub fn get_words(&self) -> Result<&Vec<String>> {
        Ok(&self.words)
    }
}
use std::vec::Vec;
//Translated from: github.com/DavidBelicza/TextRank.Text
#[derive(Default)]#[derive(Clone)]pub struct Text {
    pub(crate) parsed_sentences: Vec<ParsedSentence>,
}


//Translated from: (*github.com/DavidBelicza/TextRank.Text).Append
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

//Translated from: (*github.com/DavidBelicza/TextRank.Text).GetSentences
impl Text {
    pub fn get_sentences(&self) -> Vec<ParsedSentence> {
        self.parsed_sentences.clone()
    }
}
