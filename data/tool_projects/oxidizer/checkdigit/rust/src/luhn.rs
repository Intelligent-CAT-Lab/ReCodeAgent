use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct Luhn {}

use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::__synthetic::__Synth0__generate;
use std::char;
use crate::checkdigit::is_not_number;
#[cfg(not(feature = "mock"))]
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
                if d > 9 { d - 9 } else { d }
            } else {
                d
            };
            sum += d;
        }
        Ok(sum * 9 % 10)
    }
}
#[cfg(feature = "mock")]
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
                if d > 9 { d - 9 } else { d }
            } else {
                d
            };
            sum += d;
        }
        Ok(sum * 9 % 10)
    }
}
#[cfg(feature = "mock")]
impl Luhn {
    fn generate__with_callees_mocked(&self, seed: &str) -> Result<i32, &'static str> {
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
                if d > 9 { d - 9 } else { d }
            } else {
                d
            };
            sum += d;
        }
        Ok(sum * 9 % 10)
    }
}

use crate::__synthetic::__Synth1__verify;
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
impl __Synth1__verify for Luhn {
    fn verify(&self, code: &str) -> bool {
        extern "C" {
            #[link_name = "checkdigit_luhn___verify__ground_truth"]
            fn Luhn_verify__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a Luhn, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Luhn>, Box<str>);
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
            >(Luhn_verify__foreign(ser(&params[0]), ser(&params[1])))
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
impl Luhn {
    fn verify__with_callees_mocked(&self, code: &str) -> bool {
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

#[typetag::serde(name = "luhn")]
impl crate::checkdigit::Provider for Luhn {}
