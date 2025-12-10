#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::boxed::Box;
use crate::rank::Rank;
//Translated from: github.com/DavidBelicza/TextRank.TextRank
#[derive(Default)]#[derive(Clone)]pub struct TextRank {
    pub(crate) rank: Box<Rank>,
}

use crate::sorting::Phrase;
use crate::sorting::find_phrases;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).FindPhrases
impl TextRank {
    pub fn find_phrases(&self) -> Vec<Phrase> {
        find_phrases(&self.rank)
    }
}
use crate::sorting::Sentence;
use crate::sorting::find_sentences_by_phrases;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).FindSentencesByPhraseChain
impl TextRank {
    pub fn find_sentences_by_phrase_chain(&self, phrases: &[String]) -> Result<Vec<Sentence>> {
        find_sentences_by_phrases(Some(&self.rank), phrases)
    }
}
use crate::sorting::BY_RELATION;
use crate::sorting::find_sentences;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).FindSentencesByRelationWeight
impl TextRank {
    pub fn find_sentences_by_relation_weight(&self, limit: i32) -> Result<Vec<Sentence>, Error> {
        find_sentences(&self.rank, BY_RELATION, limit)
    }
}
use crate::sorting::BY_QTY;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).FindSentencesByWordQtyWeight
impl TextRank {
    pub fn find_sentences_by_word_qty_weight(&self, limit: i32) -> Result<Vec<Sentence>, Error> {
        find_sentences(&self.rank, BY_QTY, limit)
    }
}
use crate::sorting::find_sentences_from;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).FindSentencesFrom
impl TextRank {
    pub fn find_sentences_from(&self, sentence_id: i32, limit: i32) -> Vec<Sentence> {
        find_sentences_from(&self.rank, sentence_id, limit)
    }
}
use crate::sorting::SingleWord;
use crate::sorting::find_single_words;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).FindSingleWords
impl TextRank {
    pub fn find_single_words(&self) -> Vec<SingleWord> {
        find_single_words(&self.rank)
    }
}
use crate::language::Language;
use crate::text::ParsedSentence;
use crate::rule::Rule;
use crate::text::Text;
use crate::builder::text_to_rank;
use crate::tokenizer::tokenize_text;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).Populate
impl TextRank {
    pub fn populate(&mut self, text: &str, lang: &dyn Language, rule: &dyn Rule) -> Result<(), Error> {
        let parsed_text = tokenize_text(text, rule)?;

        for sentence in parsed_text.get_sentences() {
            text_to_rank(sentence, lang, &mut self.rank)?;
        }

        Ok(())
    }
}
use crate::algorithm::Algorithm;
use crate::ranking::calculate;
//Translated from: (*github.com/DavidBelicza/TextRank.TextRank).Ranking
impl TextRank {
    pub fn ranking(&mut self, algorithm: &dyn Algorithm) -> Result<(), anyhow::Error> {
        calculate(&mut self.rank, algorithm)?;
        Ok(())
    }
}
use crate::algorithm::AlgorithmDefault;
use crate::algorithm::new_algorithm_default;
//Translated from: github.com/DavidBelicza/TextRank.NewDefaultAlgorithm
pub fn new_default_algorithm() -> Result<AlgorithmDefault> {
    new_algorithm_default()
}
use crate::language::LanguageDefault;
use crate::language::new_language;
//Translated from: github.com/DavidBelicza/TextRank.NewDefaultLanguage
pub fn new_default_language() -> Result<LanguageDefault, Error> {
    new_language()
}
use crate::rule::RuleDefault;
//Translated from: github.com/DavidBelicza/TextRank.NewDefaultRule
impl RuleDefault {
    pub fn new_default_rule() -> Result<RuleDefault> {
        RuleDefault::new()
    }
}

//Translated from: github.com/DavidBelicza/TextRank.NewTextRank
impl TextRank {
    pub fn new_text_rank() -> TextRank {
        TextRank {
            rank: Box::new(Rank::new_rank()),
        }
    }
}
