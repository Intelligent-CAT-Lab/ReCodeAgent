use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct isbn10 {}

use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::is_not_number;
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
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
#[cfg(feature = "mock")]
impl isbn10 {
    fn generate__with_callees_mocked(&self, seed: &str) -> Result<i32, &'static str> {
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

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct Isbn13 {}

use std::convert::TryFrom;
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
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
#[cfg(feature = "mock")]
impl Isbn13 {
    fn generate__with_callees_mocked(&self, seed: &str) -> Result<i32, &'static str> {
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
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
impl __Synth1__verify for isbn10 {
    fn verify(&self, code: &str) -> bool {
        extern "C" {
            #[link_name = "checkdigit_isbn10___verify__ground_truth"]
            fn isbn10_verify__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a isbn10, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<isbn10>, Box<str>);
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
            >(isbn10_verify__foreign(ser(&params[0]), ser(&params[1])))
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
impl isbn10 {
    fn verify__with_callees_mocked(&self, code: &str) -> bool {
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

#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
impl __Synth1__verify for Isbn13 {
    fn verify(&self, code: &str) -> bool {
        extern "C" {
            #[link_name = "checkdigit_isbn13___verify__ground_truth"]
            fn Isbn13_verify__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a Isbn13, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Isbn13>, Box<str>);
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
            >(Isbn13_verify__foreign(ser(&params[0]), ser(&params[1])))
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
impl Isbn13 {
    fn verify__with_callees_mocked(&self, code: &str) -> bool {
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

#[typetag::serde(name = "isbn10")]
impl crate::checkdigit::Provider for isbn10 {}

#[typetag::serde(name = "isbn13")]
impl crate::checkdigit::Provider for Isbn13 {}
