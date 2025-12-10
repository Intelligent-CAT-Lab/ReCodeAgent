#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/osamingo/checkdigit.isbn10
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub struct isbn10 {}

use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::checkdigit::is_not_number;
//Translated from: (*github.com/osamingo/checkdigit.isbn10).Generate
impl __Synth0__generate for isbn10 {
    fn generate(&self, seed: &str) -> Result<i32, &'static str> {
        if seed.len() != 9 {
            return Err(ERR_INVALID_ARGUMENT);
        }

        let mut sum = 0;
        let mut multiply = 10;

        for n in seed.chars() {
            if is_not_number(n) {
                return Err(ERR_INVALID_ARGUMENT);
            }

            sum += multiply * (n as i32 - '0' as i32);
            multiply -= 1;
        }

        Ok(11 - (sum % 11))
    }
}

//Translated from: github.com/osamingo/checkdigit.isbn13
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]#[derive(Clone)]pub struct Isbn13 {}

use std::convert::TryFrom;
//Translated from: (*github.com/osamingo/checkdigit.isbn13).Generate
impl __Synth0__generate for Isbn13 {
    fn generate(&self, seed: &str) -> Result<i32, &'static str> {
        if seed.len() != 12 {
            return Err(ERR_INVALID_ARGUMENT);
        }

        let mut sum = 0;
        let mut weight = 1;
        for n in seed.chars() {
            if is_not_number(n) {
                return Err(ERR_INVALID_ARGUMENT);
            }
            sum += i32::try_from(n.to_digit(10).unwrap()).unwrap() * weight;
            weight = if weight == 1 { 3 } else { 1 };
        }

        let d = 10 - sum % 10;
        let d = if d == 10 { 0 } else { d };

        Ok(d)
    }
}
use crate::__synthetic::__Synth1__verify;
//Translated from: (github.com/osamingo/checkdigit.isbn10).Verify
impl __Synth1__verify for isbn10 {
    fn verify(&self, code: &str) -> bool {
        if code.len() != 10 {
            return false;
        }

        let mut sum = 0;
        let mut multiply = 10;
        for n in code.chars() {
            let digit = match n {
                'X' => 10,
                n if is_not_number(n) => return false,
                n => n.to_digit(10).unwrap() as usize,
            };

            sum += multiply * digit;
            multiply -= 1;
        }

        sum % 11 == 0
    }
}

//Translated from: (github.com/osamingo/checkdigit.isbn13).Verify
impl __Synth1__verify for Isbn13 {
    fn verify(&self, code: &str) -> bool {
        if code.len() != 13 {
            return false;
        }

        let seed = &code[..code.len() - 1];
        let result = self.generate(seed);

        match result {
            Ok(checkdigit) => {
                let last_digit = code.chars().last().unwrap().to_digit(10).unwrap();
                checkdigit == last_digit as i32
            }
            Err(_) => false,
        }
    }
}
