#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/osamingo/checkdigit.luhn
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub struct Luhn {}

use std::char;
use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::checkdigit::is_not_number;
//Translated from: (*github.com/osamingo/checkdigit.luhn).Generate
impl __Synth0__generate for Luhn {
    fn generate(&self, seed: &str) -> Result<i32, &'static str> {
        if seed.is_empty() {
            return Err(ERR_INVALID_ARGUMENT);
        }

        let parity = (seed.len() + 1) % 2;
        let mut sum = 0;

        for (i, n) in seed.chars().enumerate() {
            if is_not_number(n) {
                return Err(ERR_INVALID_ARGUMENT);
            }
            let d = n.to_digit(10).unwrap() as i32;
            let d = if i % 2 == parity {
                let d = d * 2;
                if d > 9 {
                    d - 9
                } else {
                    d
                }
            } else {
                d
            };
            sum += d;
        }

        Ok(sum * 9 % 10)
    }
}
use crate::__synthetic::__Synth1__verify;
//Translated from: (github.com/osamingo/checkdigit.luhn).Verify
impl __Synth1__verify for Luhn {
    fn verify(&self, code: &str) -> bool {
        if code.len() < 2 {
            return false;
        }
        let i = match self.generate(&code[..code.len() - 1]) {
            Ok(i) => i,
            Err(_) => return false,
        };

        i == code.chars().last().unwrap().to_digit(10).unwrap() as i32
    }
}
