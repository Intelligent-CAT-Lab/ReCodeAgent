#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
use crate::text::ParsedSentence;
use crate::rank::Rank;
//Translated from: github.com/DavidBelicza/TextRank.addSentence
pub(crate) fn add_sentence(ranks: &mut Rank, sentence: ParsedSentence) -> Result<i32, Error> {
    let sentence_id = ranks.sentence_map.len() as i32;
    ranks.sentence_map.insert(sentence_id, sentence.original.clone());
    Ok(sentence_id)
}
use crate::language::Language;
use crate::relation::Relation;
use crate::__synthetic::__Synth0__find_root_word;
use crate::__synthetic::__Synth2__is_stop_word;
//Translated from: github.com/DavidBelicza/TextRank.addWord
pub(crate) fn add_word(
    ranks: &mut Rank,
    words: &[String],
    lang: &dyn Language,
    sentence_id: i32,
) -> Result<(), Error> {
    let mut prev_word_id = -1;

    for word in words.iter().filter(|w| !lang.is_stop_word(w.as_str())) {
        let (found, root_word) = lang.find_root_word(word.as_str());
        let word = if found { root_word } else { word.clone() };

        let cur_word_id = if ranks.is_word_exist(&word) {
            ranks.update_word(&word, prev_word_id, sentence_id)?
        } else {
            ranks.add_new_word(&word, prev_word_id, sentence_id)
        };

        ranks.relation.add_relation(cur_word_id, prev_word_id, sentence_id)?;
        ranks.update_right_connection(prev_word_id, cur_word_id)?;

        prev_word_id = cur_word_id;
    }

    Ok(())
}

//Translated from: github.com/DavidBelicza/TextRank.TextToRank
pub fn text_to_rank(
    sentence: ParsedSentence,
    lang: &dyn Language,
    ranks: &mut Rank,
) -> Result<(), Error> {
    let sentence_id = add_sentence(ranks, sentence.clone())?;
    add_word(ranks, sentence.get_words()?, lang, sentence_id)?;
    Ok(())
}
