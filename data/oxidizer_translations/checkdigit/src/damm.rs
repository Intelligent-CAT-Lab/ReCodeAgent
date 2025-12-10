#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/osamingo/checkdigit.damm
#[derive(Default)]#[derive(Clone)]pub struct Damm {
    pub(crate) matrix: Vec<Vec<i32>>,
}

use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::checkdigit::is_not_number;
//Translated from: (*github.com/osamingo/checkdigit.damm).Generate
impl __Synth0__generate for Damm {
    fn generate(&self, seed: &str) -> Result<i32, &'static str> {
        if seed.is_empty() {
            return Err(ERR_INVALID_ARGUMENT);
        }

        let mut interim = 0;
        for c in seed.chars() {
            if is_not_number(c) {
                return Err(ERR_INVALID_ARGUMENT);
            }
            interim = self.matrix[interim as usize][char::to_digit(c, 10).unwrap() as usize];
        }

        Ok(interim)
    }
}
use crate::__synthetic::__Synth1__verify;
//Translated from: (github.com/osamingo/checkdigit.damm).Verify
impl __Synth1__verify for Damm {
    fn verify(&self, code: &str) -> bool {
        if code.len() < 2 {
            return false;
        }
        let result = self.generate(&code[..code.len() - 1]);
        match result {
            Ok(digit) => digit == (code.chars().last().unwrap() as i32 - '0' as i32),
            Err(_) => false,
        }
    }
}
