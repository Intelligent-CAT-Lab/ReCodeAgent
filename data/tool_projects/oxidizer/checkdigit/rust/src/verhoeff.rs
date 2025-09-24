use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashMap;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct Verhoeff {
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "multiplication")]
    pub(crate) multiplication: Vec<Vec<i32>>,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "permutation")]
    pub(crate) permutation: Vec<Vec<i32>>,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "inverse")]
    pub(crate) inverse: Vec<i32>,
}

use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::is_not_number;
#[cfg(not(feature = "mock"))]
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
            interim = self
                .multiplication[interim
                as usize][self.permutation[i % 8][((n as u8 - b'0') as usize)] as usize];
        }
        Ok(self.inverse[interim as usize])
    }
}
#[cfg(feature = "mock")]
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
            interim = self
                .multiplication[interim
                as usize][self.permutation[i % 8][((n as u8 - b'0') as usize)] as usize];
        }
        Ok(self.inverse[interim as usize])
    }
}
#[cfg(feature = "mock")]
impl Verhoeff {
    fn generate__with_callees_mocked(&self, seed: &str) -> Result<i32, &'static str> {
        if seed.is_empty() {
            return Err(ERR_INVALID_ARGUMENT);
        }
        let mut interim = 0;
        for (i, n) in seed.chars().rev().enumerate() {
            if is_not_number(n) {
                return Err(ERR_INVALID_ARGUMENT);
            }
            interim = self
                .multiplication[interim
                as usize][self.permutation[i % 8][((n as u8 - b'0') as usize)] as usize];
        }
        Ok(self.inverse[interim as usize])
    }
}

use crate::__synthetic::__Synth1__verify;
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
impl __Synth1__verify for Verhoeff {
    fn verify(&self, code: &str) -> bool {
        extern "C" {
            #[link_name = "checkdigit_verhoeff___verify__ground_truth"]
            fn Verhoeff_verify__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a Verhoeff, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Verhoeff>, Box<str>);
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
            >(Verhoeff_verify__foreign(ser(&params[0]), ser(&params[1])))
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
impl Verhoeff {
    fn verify__with_callees_mocked(&self, code: &str) -> bool {
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

#[typetag::serde(name = "verhoeff")]
impl crate::checkdigit::Provider for Verhoeff {}
