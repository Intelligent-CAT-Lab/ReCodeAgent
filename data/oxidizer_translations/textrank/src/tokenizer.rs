#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::rule::Rule;
use crate::__synthetic::__Synth3__is_word_separator;
//Translated from: github.com/DavidBelicza/TextRank.findWords
pub(crate) fn find_words(raw_sentence: &str, rule: &dyn Rule) -> Result<Vec<String>> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut i = 0;
    let slen = raw_sentence.len();

    for (j, chr) in raw_sentence.char_indices() {
        let chrlen = chr.len_utf8();
        let j = j + chrlen;

        if rule.is_word_separator(chr) || j == slen {
            if rule.is_word_separator(chr) {
                word = raw_sentence[i..j - chrlen].to_string();
            } else {
                word = raw_sentence[i..j].to_string();
            }
            if !word.is_empty() {
                words.push(word.to_lowercase());
            }
            word.clear();
            i = j;
        }
    }

    Ok(words)
}
use crate::text::Text;
use crate::__synthetic::__Synth1__is_sentence_separator;
//Translated from: github.com/DavidBelicza/TextRank.TokenizeText
pub fn tokenize_text(raw_text: &str, rule: &dyn Rule) -> Result<Text> {
    let mut text = Text::default();

    let mut sentence = String::new();
    let mut i = 0;
    let slen = raw_text.len();

    for (j, chr) in raw_text.char_indices() {
        let j = j + chr.len_utf8();
        
        // When separator or the last
        if rule.is_sentence_separator(chr) || j == slen {
            sentence = raw_text[i..j].to_string();
            if !sentence.is_empty() {
                text.append(&sentence, &find_words(&sentence, rule)?)?;
            }

            sentence.clear();
            i = j;
        }
    }

    Ok(text)
}
