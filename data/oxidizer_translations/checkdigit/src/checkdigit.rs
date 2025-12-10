#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/osamingo/checkdigit.isNotNumber
pub(crate) fn is_not_number(n: char) -> bool {
    n < '0' || n > '9'
}

//Translated from: github.com/osamingo/checkdigit.ErrInvalidArgument
pub static ERR_INVALID_ARGUMENT: &'static str = "checkdigit: invalid argument";

//Translated from: github.com/osamingo/checkdigit.Generator
pub trait Generator: crate::__synthetic::__Synth0__generate {}
impl<T> Generator for T where T: crate::__synthetic::__Synth0__generate {}


//Translated from: github.com/osamingo/checkdigit.Provider
pub trait Provider: crate::__synthetic::__Synth1__verify + crate::__synthetic::__Synth0__generate {}
impl<T> Provider for T where T: crate::__synthetic::__Synth1__verify + crate::__synthetic::__Synth0__generate {}


//Translated from: github.com/osamingo/checkdigit.Verifier
pub trait Verifier: crate::__synthetic::__Synth1__verify {}
impl<T> Verifier for T where T: crate::__synthetic::__Synth1__verify {}

use crate::damm::Damm;
//Translated from: github.com/osamingo/checkdigit.NewDamm
impl Damm {
    pub fn new_damm() -> Result<Box<dyn Provider>> {
        let matrix = vec![
            vec![0, 3, 1, 7, 5, 9, 8, 6, 4, 2],
            vec![7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
            vec![4, 2, 0, 6, 8, 7, 1, 3, 5, 9],
            vec![1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
            vec![6, 1, 2, 3, 0, 4, 5, 9, 7, 8],
            vec![3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
            vec![5, 8, 6, 9, 7, 2, 0, 1, 3, 4],
            vec![8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
            vec![9, 4, 3, 8, 6, 1, 7, 2, 0, 5],
            vec![2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
        ];

        let damm = Damm { matrix };
        Ok(Box::new(damm) as Box<dyn Provider>)
    }
}
use crate::gtin::gtin;
//Translated from: github.com/osamingo/checkdigit.NewEAN13
pub fn new_ean13() -> Box<dyn Provider> {
    Box::new(gtin {
        digit: 13,
        pos_corr: true,
    })
}

//Translated from: github.com/osamingo/checkdigit.NewEAN8
pub fn new_ean8() -> Result<Box<dyn Provider>, anyhow::Error> {
    Ok(Box::new(gtin {
        digit: 8,
        pos_corr: true,
    }))
}
use crate::isbn::isbn10;
//Translated from: github.com/osamingo/checkdigit.NewISBN10
pub fn NewISBN10() -> Box<dyn Provider> {
    Box::new(isbn10::default())
}
use crate::isbn::Isbn13;
//Translated from: github.com/osamingo/checkdigit.NewISBN13
pub fn new_isbn13() -> Result<Box<dyn Provider>, anyhow::Error> {
    Ok(Box::new(Isbn13::default()))
}

//Translated from: github.com/osamingo/checkdigit.NewITF
pub fn new_itf() -> Result<Box<dyn Provider>> {
    Ok(Box::new(gtin {
        digit: 14,
        pos_corr: false,
    }))
}

//Translated from: github.com/osamingo/checkdigit.NewJAN13
pub fn new_jan13() -> Box<dyn Provider> {
    Box::new(gtin {
        digit: 13,
        pos_corr: true,
    })
}

//Translated from: github.com/osamingo/checkdigit.NewJAN8
impl gtin {
    pub fn new_jan8() -> Self {
        Self {
            digit: 8,
            pos_corr: true,
        }
    }
}
use std::boxed::Box;
use crate::luhn::Luhn;
//Translated from: github.com/osamingo/checkdigit.NewLuhn
pub fn new_luhn() -> Box<dyn Provider> {
    Box::new(Luhn {})
}

//Translated from: github.com/osamingo/checkdigit.NewSSCC
pub fn new_sscc() -> Box<dyn Provider> {
    Box::new(gtin {
        digit: 18,
        pos_corr: false,
    })
}

//Translated from: github.com/osamingo/checkdigit.NewUPC
pub fn new_upc() -> Box<dyn Provider> {
    Box::new(gtin {
        digit: 12,
        pos_corr: true,
    })
}
use crate::verhoeff::Verhoeff;
//Translated from: github.com/osamingo/checkdigit.NewVerhoeff
impl Verhoeff {
    pub fn new_verhoeff() -> Verhoeff {
        Verhoeff {
            multiplication: vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
                vec![2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
                vec![3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
                vec![4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
                vec![5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
                vec![6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
                vec![7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
                vec![8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            ],
            permutation: vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
                vec![5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
                vec![8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
                vec![9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
                vec![4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
                vec![2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
                vec![7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
            ],
            inverse: vec![0, 4, 3, 2, 1, 5, 6, 7, 8, 9],
        }
    }
}
