use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct Damm {
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "matrix")]
    pub(crate) matrix: Vec<Vec<i32>>,
}

use crate::checkdigit::ERR_INVALID_ARGUMENT;
use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::is_not_number;
#[cfg(not(feature = "mock"))]
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
            interim = self
                .matrix[interim as usize][char::to_digit(c, 10).unwrap() as usize];
        }
        Ok(interim)
    }
}
#[cfg(feature = "mock")]
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
            interim = self
                .matrix[interim as usize][char::to_digit(c, 10).unwrap() as usize];
        }
        Ok(interim)
    }
}
#[cfg(feature = "mock")]
impl Damm {
    fn generate__with_callees_mocked(&self, seed: &str) -> Result<i32, &'static str> {
        if seed.is_empty() {
            return Err(ERR_INVALID_ARGUMENT);
        }
        let mut interim = 0;
        for c in seed.chars() {
            if is_not_number(c) {
                return Err(ERR_INVALID_ARGUMENT);
            }
            interim = self
                .matrix[interim as usize][char::to_digit(c, 10).unwrap() as usize];
        }
        Ok(interim)
    }
}

use crate::__synthetic::__Synth1__verify;
#[cfg(not(feature = "mock"))]
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
#[cfg(feature = "mock")]
impl __Synth1__verify for Damm {
    fn verify(&self, code: &str) -> bool {
        extern "C" {
            #[link_name = "checkdigit_damm___verify__ground_truth"]
            fn Damm_verify__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a Damm, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Damm>, Box<str>);
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
            >(Damm_verify__foreign(ser(&params[0]), ser(&params[1])))
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
impl Damm {
    fn verify__with_callees_mocked(&self, code: &str) -> bool {
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

#[typetag::serde(name = "damm")]
impl crate::checkdigit::Provider for Damm {}
