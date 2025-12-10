#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
//Translated from: github.com/DavidBelicza/TextRank.LanguageDefault
#[derive(Default)]#[derive(Clone)]pub struct LanguageDefault {
    pub default_lang: String,
    pub languages: HashMap<String, Vec<String>>,
}


//Translated from: github.com/DavidBelicza/TextRank.Language
pub trait Language: crate::__synthetic::__Synth2__is_stop_word + crate::__synthetic::__Synth0__find_root_word + crate::__synthetic::__Synth4__set_words {}
impl<T> Language for T where T: crate::__synthetic::__Synth2__is_stop_word + crate::__synthetic::__Synth0__find_root_word + crate::__synthetic::__Synth4__set_words {}

use crate::__synthetic::__Synth0__find_root_word;
//Translated from: (*github.com/DavidBelicza/TextRank.LanguageDefault).FindRootWord
impl __Synth0__find_root_word for LanguageDefault {
    fn find_root_word(&self, _: &str) -> (bool, String) {
        (false, String::new())
    }
}
use crate::__synthetic::__Synth2__is_stop_word;
//Translated from: (*github.com/DavidBelicza/TextRank.LanguageDefault).IsStopWord
impl __Synth2__is_stop_word for LanguageDefault {
    fn is_stop_word(&self, word: &str) -> bool {
        // Check if word length is <= 2
        if word.chars().count() <= 2 {
            return true;
        }

        // Check if word is in stop words list for default language
        if let Some(stop_words) = self.languages.get(&self.default_lang) {
            if stop_words.contains(&word.to_string()) {
                return true;
            }
        }

        false
    }
}
use crate::__synthetic::__Synth4__set_words;
//Translated from: (*github.com/DavidBelicza/TextRank.LanguageDefault).SetWords
impl __Synth4__set_words for LanguageDefault {
    fn set_words(&mut self, code: &str, words: &[String]) {
        self.languages.insert(code.to_string(), words.to_vec());
    }
}
use crate::stop_word::get_default_english;
//Translated from: github.com/DavidBelicza/TextRank.NewLanguage
pub fn new_language() -> Result<LanguageDefault, Error> {
    let mut lang = LanguageDefault {
        default_lang: "en".to_string(),
        languages: HashMap::new(),
    };

    let words = get_default_english();

    lang.set_words("en", &words);

    Ok(lang)
}
