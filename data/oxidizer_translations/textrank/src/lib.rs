pub mod algorithm;
pub mod builder;
pub mod examples;
pub mod language;
pub mod mock;
pub mod rank;
pub mod ranking;
pub mod relation;
pub mod rule;
pub mod sorting;
pub mod stop_word;
pub mod text;
pub mod textrank_impl;
pub mod tokenizer;
mod __synthetic;

// Re-exports for convenience
pub use algorithm::{Algorithm, AlgorithmDefault, AlgorithmChain, new_algorithm_default};
pub use language::{Language, LanguageDefault, new_language};
pub use rank::{Rank, Word};
pub use relation::{Relation, Score};
pub use rule::{Rule, RuleDefault};
pub use sorting::{Phrase, Sentence, SingleWord, find_phrases, find_sentences, find_sentences_from, find_single_words, find_sentences_by_phrases, BY_QTY, BY_RELATION};
pub use text::{Text, ParsedSentence};
pub use textrank_impl::{TextRank, new_default_algorithm, new_default_language};
pub use tokenizer::tokenize_text;

// Go-style wrapper functions for test compatibility
pub fn NewTextRank() -> TextRank {
    TextRank::new_text_rank()
}

pub fn NewDefaultRule() -> RuleDefault {
    RuleDefault::new().unwrap()
}

pub fn NewDefaultLanguage() -> LanguageDefault {
    new_language().unwrap()
}

pub fn NewDefaultAlgorithm() -> AlgorithmDefault {
    new_algorithm_default().unwrap()
}

pub fn NewChainAlgorithm() -> AlgorithmChain {
    AlgorithmChain::default()
}

pub fn NewAlgorithmDefault() -> AlgorithmDefault {
    new_algorithm_default().unwrap()
}

pub fn NewAlgorithmChain() -> AlgorithmChain {
    AlgorithmChain::default()
}

pub fn NewRank() -> Rank {
    Rank::new_rank()
}

pub fn NewLanguage() -> LanguageDefault {
    new_language().unwrap()
}

pub fn NewRule() -> RuleDefault {
    RuleDefault::new().unwrap()
}

pub fn FindSentences(ranks: &Rank, kind: i32, limit: i32) -> Vec<Sentence> {
    find_sentences(ranks, kind, limit).unwrap_or_default()
}

pub fn TokenizeText(text: &str, rule: &dyn Rule) -> Text {
    tokenize_text(text, rule).unwrap()
}

// Add Go-style methods to TextRank
impl TextRank {
    pub fn Populate(&mut self, text: &str, lang: &dyn Language, rule: &dyn Rule) {
        self.populate(text, lang, rule).unwrap();
    }

    pub fn Ranking(&mut self, algorithm: &dyn Algorithm) {
        self.ranking(algorithm).unwrap();
    }

    pub fn FindPhrases(&self) -> Vec<Phrase> {
        self.find_phrases()
    }

    pub fn FindSingleWords(&self) -> Vec<SingleWord> {
        self.find_single_words()
    }

    pub fn FindSentencesByWordQtyWeight(&self, limit: i32) -> Vec<Sentence> {
        self.find_sentences_by_word_qty_weight(limit).unwrap_or_default()
    }

    pub fn FindSentencesByRelationWeight(&self, limit: i32) -> Vec<Sentence> {
        self.find_sentences_by_relation_weight(limit).unwrap_or_default()
    }

    pub fn FindSentencesByPhraseChain(&self, phrases: &Vec<String>) -> Vec<Sentence> {
        self.find_sentences_by_phrase_chain(phrases).unwrap_or_default()
    }

    pub fn FindSentencesFrom(&self, sentence_id: i32, limit: i32) -> Vec<Sentence> {
        self.find_sentences_from(sentence_id, limit)
    }

    pub fn GetRankData(&self) -> &Rank {
        &self.rank
    }
}

// Add Go-style field accessors to Phrase
impl Phrase {
    pub fn Left(&self) -> &str { &self.left }
    pub fn Right(&self) -> &str { &self.right }
    pub fn LeftID(&self) -> i32 { self.left_id }
    pub fn RightID(&self) -> i32 { self.right_id }
    pub fn Weight(&self) -> f32 { self.weight }
    pub fn Qty(&self) -> i32 { self.qty }
}

// Add Go-style field accessors to SingleWord
impl SingleWord {
    pub fn Word(&self) -> &str { &self.word }
    pub fn ID(&self) -> i32 { self.id }
    pub fn Weight(&self) -> f32 { self.weight }
    pub fn Qty(&self) -> i32 { self.qty }
}

// Add Go-style field accessors to Sentence
impl Sentence {
    pub fn ID(&self) -> i32 { self.id }
    pub fn Value(&self) -> &str { &self.value }
}

// Add Go-style field accessors to Rank
impl Rank {
    pub fn WordValID(&self) -> &std::collections::HashMap<String, i32> { &self.word_val_id }
    pub fn SentenceMap(&self) -> &std::collections::HashMap<i32, String> { &self.sentence_map }

    pub fn AddNewWord(&mut self, word: String, prev_word_idx: i32, sentence_id: i32) -> i32 {
        self.add_new_word(&word, prev_word_idx, sentence_id)
    }

    pub fn UpdateWord(&mut self, word: &str, prev_word_idx: i32, sentence_id: i32) -> i32 {
        self.update_word(word, prev_word_idx, sentence_id).unwrap()
    }

    pub fn GetWordData(&self) -> &std::collections::HashMap<i32, Box<Word>> {
        &self.words
    }
}

// Add Go-style methods to Relation
impl Relation {
    pub fn AddRelation(&mut self, word_id: i32, related_word_id: i32, sentence_id: i32) {
        self.add_relation(word_id, related_word_id, sentence_id).unwrap();
    }

    pub fn Max(&self) -> f32 { self.max }
    pub fn Min(&self) -> f32 { self.min }
}

// Add Go-style methods to Algorithm types
impl AlgorithmDefault {
    pub fn WeightingRelation(&self, word1_id: i32, word2_id: i32, rank: &Rank) -> f32 {
        use __synthetic::__Synth6__weighting_relation;
        self.weighting_relation(word1_id, word2_id, rank)
    }

    pub fn WeightingHits(&self, word_id: i32, rank: &Rank) -> f32 {
        use __synthetic::__Synth5__weighting_hits;
        self.weighting_hits(word_id, rank)
    }
}

impl AlgorithmChain {
    pub fn WeightingRelation(&self, word1_id: i32, word2_id: i32, rank: &Rank) -> f32 {
        use __synthetic::__Synth6__weighting_relation;
        self.weighting_relation(word1_id, word2_id, rank)
    }

    pub fn WeightingHits(&self, word_id: i32, rank: &Rank) -> f32 {
        use __synthetic::__Synth5__weighting_hits;
        self.weighting_hits(word_id, rank)
    }
}

// Add Go-style methods to LanguageDefault
impl LanguageDefault {
    pub fn SetActiveLanguage(&mut self, code: &str) {
        self.default_lang = code.to_string();
    }

    pub fn SetWords(&mut self, code: &str, words: Vec<String>) {
        use __synthetic::__Synth4__set_words;
        __synthetic::__Synth4__set_words::set_words(self, code, &words);
    }

    pub fn IsStopWord(&self, word: &str) -> bool {
        use __synthetic::__Synth2__is_stop_word;
        __synthetic::__Synth2__is_stop_word::is_stop_word(self, word)
    }
}

// Add Go-style methods to Text
impl Text {
    pub fn GetSentences(&self) -> Vec<ParsedSentence> {
        self.get_sentences()
    }
}

// Add Go-style methods to ParsedSentence
impl ParsedSentence {
    pub fn GetOriginal(&self) -> &str {
        self.get_original()
    }
}