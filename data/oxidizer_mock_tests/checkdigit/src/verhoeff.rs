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
#[cfg(test)]
mod checkdigit_verhoeff_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "checkdigit_verhoeff_roundtrip"]
        fn Verhoeff__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn Verhoeff__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!("(*{}).", "github.com-osamingo-checkdigit.verhoeff"),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!("({}).", "github.com-osamingo-checkdigit.verhoeff"),
                            )
                    {
                        let unittests_file: std::fs::File = std::fs::File::open(
                                entry.path(),
                            )
                            .unwrap();
                        let unittests_reader = std::io::BufReader::new(unittests_file);
                        let unittests: Vec<ExecutionData> = serde_json::from_reader(
                                unittests_reader,
                            )
                            .unwrap();
                        for unittest in unittests {
                            let obj = unittest.inputs[0].clone();
                            if obj == serde_json::Value::Null {
                                continue;
                            }
                            let obj_once = serde_json::to_value(
                                    serde_json::from_value::<Verhoeff>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<Verhoeff>(obj_once.clone())
                                        .unwrap(),
                                )
                                .unwrap();
                            assert_json_diff::assert_json_eq!(obj_once, obj_twice);
                        }
                    }
                }
            }
        }
    }
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
#[cfg(test)]
mod checkdigit_verhoeff___generate_harness {
    use super::*;
    #[test]
    fn Verhoeff_generate__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-osamingo-checkdigit.verhoeff).Generate.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Verhoeff>, Box<str>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(i32);
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    {
                        ((&*input_state.0)
                            .generate__with_callees_mocked(&*input_state.1))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { ((&*input_state.0).generate(&*input_state.1)).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.1).unwrap(),
                        serde_json::to_value(& input_state_mutated.1).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn Verhoeff_generate__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Verhoeff>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-osamingo-checkdigit.verhoeff).Generate.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
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
#[cfg(test)]
mod checkdigit_verhoeff___verify_harness {
    use super::*;
    #[test]
    fn Verhoeff_verify__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(github.com-osamingo-checkdigit.verhoeff).Verify.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Verhoeff>, Box<str>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(bool);
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (&*input_state.0).verify__with_callees_mocked(&*input_state.1) }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).verify(&*input_state.1) }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.1).unwrap(),
                        serde_json::to_value(& input_state_mutated.1).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn Verhoeff_verify__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Verhoeff>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(github.com-osamingo-checkdigit.verhoeff).Verify.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}

#[typetag::serde(name = "verhoeff")]
impl crate::checkdigit::Provider for Verhoeff {}
