#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
use crate::algorithm::Algorithm;
use crate::algorithm::AlgorithmDefault;
use crate::language::Language;
use crate::language::LanguageDefault;
use crate::sorting::Phrase;
use crate::rule::Rule;
use crate::rule::RuleDefault;
use crate::sorting::Sentence;
use crate::sorting::SingleWord;
use crate::textrank_impl::TextRank;
use crate::textrank_impl::new_default_algorithm;
use crate::textrank_impl::new_default_language;
//Translated from: github.com/DavidBelicza/TextRank.Example
pub fn example() -> Result<(), Error> {
    let raw_text = "Your long raw text, it could be a book. Lorem ipsum...";

    // TextRank object
    let mut tr = TextRank::new_text_rank();

    // Default Rule for parsing.
    let rule = RuleDefault::new_default_rule()?;

    // Default Language for filtering stop words.
    let language = new_default_language()?;

    // Default algorithm for ranking text.
    let algorithm_def = new_default_algorithm()?;

    // Add text.
    tr.populate(&raw_text, &language, &rule)?;

    // Run the ranking.
    tr.ranking(&algorithm_def)?;

    // Get all phrases by weight.
    let _ = tr.find_phrases();

    // Get all words order by weight.
    let _ = tr.find_single_words();

    // Get the most important 10 sentences. Importance by phrase weights.
    let _ = tr.find_sentences_by_relation_weight(10)?;

    // Get the most important 10 sentences. Importance by word occurrence.
    let _ = tr.find_sentences_by_word_qty_weight(10)?;

    // Get the first 10 sentences, start from 5th sentence.
    let _ = tr.find_sentences_from(5, 10);

    // Get sentences by phrase/word chains order by position in text.
    let _ = tr.find_sentences_by_phrase_chain(&vec!["gnome".to_string(), "shell".to_string(), "extension".to_string()])?;

    Ok(())
}

