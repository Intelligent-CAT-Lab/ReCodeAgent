use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct gtin {
    #[serde(rename = "digit")]
    pub(crate) digit: i32,
    #[serde(rename = "posCorr")]
    pub(crate) pos_corr: bool,
}

use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::__synthetic::__Synth0__generate;
use std::char;
use crate::checkdigit::is_not_number;
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
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
#[cfg(feature = "mock")]
impl gtin {
    fn generate__with_callees_mocked(&self, seed: &str) -> Result<i32, &'static str> {
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
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
impl __Synth1__verify for gtin {
    fn verify(&self, code: &str) -> bool {
        extern "C" {
            #[link_name = "checkdigit_gtin___verify__ground_truth"]
            fn gtin_verify__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a gtin, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<gtin>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, code);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(gtin_verify__foreign(ser(&params[0]), ser(&params[1])))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 2usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
            };
            let input_state_mutated: InputStateOut = serde_json::from_value(
                    inputs_mutation_reserialized,
                )
                .unwrap();
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = output_state.0;
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl gtin {
    fn verify__with_callees_mocked(&self, code: &str) -> bool {
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

#[typetag::serde(name = "gtin")]
impl crate::checkdigit::Provider for gtin {}
