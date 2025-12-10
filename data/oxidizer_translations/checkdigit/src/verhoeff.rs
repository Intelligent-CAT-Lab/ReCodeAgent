#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
//Translated from: github.com/osamingo/checkdigit.verhoeff
#[derive(Default)]#[derive(Clone)]pub struct Verhoeff {
    pub(crate) multiplication: Vec<Vec<i32>>,
    pub(crate) permutation: Vec<Vec<i32>>,
    pub(crate) inverse: Vec<i32>,
}

use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::checkdigit::is_not_number;
//Translated from: (*github.com/osamingo/checkdigit.verhoeff).Generate
impl __Synth0__generate for Verhoeff {
    fn generate(&self, seed: &str) -> Result<i32, &'static str> {
        if seed.is_empty() {
            return Err(ERR_INVALID_ARGUMENT);
        }

        let mut interim = 0;
        for (i, n) in seed.chars().rev().enumerate() {
            if is_not_number(n) {
                return Err(ERR_INVALID_ARGUMENT);
            }
            interim = self.multiplication[interim as usize][self.permutation[i % 8][((n as u8 - b'0') as usize)] as usize];
        }

        Ok(self.inverse[interim as usize])
    }
}

use crate::__synthetic::__Synth1__verify;
//Translated from: (github.com/osamingo/checkdigit.verhoeff).Verify
impl __Synth1__verify for Verhoeff {
    fn verify(&self, code: &str) -> bool {
        if code.len() < 2 {
            return false;
        }

        let result = self.generate(&code[..code.len() - 1]);

        match result {
            Ok(digit) => digit as u8 as char == code.chars().last().unwrap(),
            Err(_) => false,
        }
    }
}
