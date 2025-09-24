use crate::careful::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use arbitrary::Arbitrary;
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::rank::Rank;
pub trait __Synth5__weighting_hits {
    fn weighting_hits(&self, _: i32, _: &Rank) -> f32;
}
macro_rules! delegate___Synth5__weighting_hits {
    ($type_name:ident, $field_name:ident) => {
        impl __Synth5__weighting_hits for $type_name { delegate::delegate! { to self.
        $field_name { fn weighting_hits(& self, input0 : i32, input1 : & Rank) -> f32; }
        } }
    };
}
macro_rules! use_delegate___Synth5__weighting_hits {
    () => {
        pub (crate) use delegate___Synth5__weighting_hits;
    };
}
use_delegate___Synth5__weighting_hits!();
pub trait __Synth6__weighting_relation {
    fn weighting_relation(&self, _: i32, _: i32, _: &Rank) -> f32;
}
macro_rules! delegate___Synth6__weighting_relation {
    ($type_name:ident, $field_name:ident) => {
        impl __Synth6__weighting_relation for $type_name { delegate::delegate! { to self.
        $field_name { fn weighting_relation(& self, input0 : i32, input1 : i32, input2 :
        & Rank) -> f32; } } }
    };
}
macro_rules! use_delegate___Synth6__weighting_relation {
    () => {
        pub (crate) use delegate___Synth6__weighting_relation;
    };
}
use_delegate___Synth6__weighting_relation!();
pub trait __Synth2__is_stop_word {
    fn is_stop_word(&self, _: &str) -> bool;
}
macro_rules! delegate___Synth2__is_stop_word {
    ($type_name:ident, $field_name:ident) => {
        impl __Synth2__is_stop_word for $type_name { delegate::delegate! { to self.
        $field_name { fn is_stop_word(& self, input0 : & str) -> bool; } } }
    };
}
macro_rules! use_delegate___Synth2__is_stop_word {
    () => {
        pub (crate) use delegate___Synth2__is_stop_word;
    };
}
use_delegate___Synth2__is_stop_word!();
pub trait __Synth0__find_root_word {
    fn find_root_word(&self, _: &str) -> (bool, String);
}
macro_rules! delegate___Synth0__find_root_word {
    ($type_name:ident, $field_name:ident) => {
        impl __Synth0__find_root_word for $type_name { delegate::delegate! { to self.
        $field_name { fn find_root_word(& self, input0 : & str) -> (bool, String); } } }
    };
}
macro_rules! use_delegate___Synth0__find_root_word {
    () => {
        pub (crate) use delegate___Synth0__find_root_word;
    };
}
use_delegate___Synth0__find_root_word!();
pub trait __Synth4__set_words {
    fn set_words(&mut self, _: &str, _: &[String]);
}
macro_rules! delegate___Synth4__set_words {
    ($type_name:ident, $field_name:ident) => {
        impl __Synth4__set_words for $type_name { delegate::delegate! { to self.
        $field_name { fn set_words(& mut self, input0 : & str, input1 : & [String]); } }
        }
    };
}
macro_rules! use_delegate___Synth4__set_words {
    () => {
        pub (crate) use delegate___Synth4__set_words;
    };
}
use_delegate___Synth4__set_words!();
pub trait __Synth3__is_word_separator {
    fn is_word_separator(&self, _: char) -> bool;
}
macro_rules! delegate___Synth3__is_word_separator {
    ($type_name:ident, $field_name:ident) => {
        impl __Synth3__is_word_separator for $type_name { delegate::delegate! { to self.
        $field_name { fn is_word_separator(& self, input0 : char) -> bool; } } }
    };
}
macro_rules! use_delegate___Synth3__is_word_separator {
    () => {
        pub (crate) use delegate___Synth3__is_word_separator;
    };
}
use_delegate___Synth3__is_word_separator!();
pub trait __Synth1__is_sentence_separator {
    fn is_sentence_separator(&self, _: char) -> bool;
}
macro_rules! delegate___Synth1__is_sentence_separator {
    ($type_name:ident, $field_name:ident) => {
        impl __Synth1__is_sentence_separator for $type_name { delegate::delegate! { to
        self. $field_name { fn is_sentence_separator(& self, input0 : char) -> bool; } }
        }
    };
}
macro_rules! use_delegate___Synth1__is_sentence_separator {
    () => {
        pub (crate) use delegate___Synth1__is_sentence_separator;
    };
}
use_delegate___Synth1__is_sentence_separator!();
