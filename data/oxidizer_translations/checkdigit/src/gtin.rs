#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/osamingo/checkdigit.gtin
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub struct gtin {
    pub(crate) digit: i32,
    pub(crate) pos_corr: bool,
}

use std::char;
use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::checkdigit::is_not_number;
//Translated from: (*github.com/osamingo/checkdigit.gtin).Generate
impl __Synth0__generate for gtin {
    fn generate(&self, seed: &str) -> Result<i32, &'static str> {
        let chars: Vec<char> = seed.chars().collect();
        if chars.len() != (self.digit - 1) as usize {
            return Err(ERR_INVALID_ARGUMENT);
        }

        let mut odd_sum = 0;
        let mut even_sum = 0;
        for (i, n) in chars.iter().enumerate() {
            if is_not_number(*n) {
                return Err(ERR_INVALID_ARGUMENT);
            }
            let idx = if self.pos_corr { i + 1 } else { i };
            if idx % 2 == 0 {
                even_sum += char::to_digit(*n, 10).unwrap() as i32;
            } else {
                odd_sum += char::to_digit(*n, 10).unwrap() as i32;
            }
        }

        let d = 10 - (even_sum * 3 + odd_sum) % 10;
        let d = if d == 10 { 0 } else { d };

        Ok(d)
    }
}
use crate::__synthetic::__Synth1__verify;
//Translated from: (github.com/osamingo/checkdigit.gtin).Verify
impl __Synth1__verify for gtin {
    fn verify(&self, code: &str) -> bool {
        if code.len() as i32 != self.digit {
            return false;
        }
        let seed: &str = &code[..code.len() - 1];
        match self.generate(seed) {
            Ok(i) => {
                let last_digit = code.chars().last().unwrap() as i32 - '0' as i32;
                i == last_digit
            }
            Err(_) => false,
        }
    }
}
