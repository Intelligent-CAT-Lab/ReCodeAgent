use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;


#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct NameString {
    #[serde(rename = "FullName")]
    pub full_name: String,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "SplitName")]
    pub split_name: Vec<String>,
    #[serde(rename = "Nickname")]
    pub nickname: String,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "Aliases")]
    pub aliases: Vec<String>,
}
#[cfg(test)]
mod gonameparts_nameString_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "gonameparts_nameString_roundtrip"]
        fn NameString__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn NameString__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!(
                                "(*{}).", "github.com-polera-gonameparts.nameString"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-polera-gonameparts.nameString"
                                ),
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
                                    serde_json::from_value::<NameString>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<NameString>(obj_once.clone())
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
use std::str;
#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn split(&mut self) -> &Vec<String> {
        self.split_name = str::split_whitespace(&self.full_name)
            .map(|s| s.to_string())
            .collect();
        &self.split_name
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn split(&mut self) -> &Vec<String> {
        self.split_name = str::split_whitespace(&self.full_name)
            .map(|s| s.to_string())
            .collect();
        &self.split_name
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn split__with_callees_mocked(&mut self) -> &Vec<String> {
        self.split_name = str::split_whitespace(&self.full_name)
            .map(|s| s.to_string())
            .collect();
        &self.split_name
    }
}
#[cfg(test)]
mod gonameparts_name_string___split_harness {
    use super::*;
    #[test]
    fn NameString_split__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).split.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a Vec<String>);
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
                    { (&mut *input_state.0).split__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { (&mut *input_state.0).split() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_split__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<Vec<String>>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).split.json",
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

#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn cleaned(&mut self) -> Vec<String> {
        let unwanted = vec![",", "."];
        let mut cleaned = Vec::new();
        for x in self.split() {
            let mut x = x.clone();
            for y in &unwanted {
                x = x.replace(y, "");
            }
            cleaned.push(x.trim().to_string());
        }
        cleaned
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn cleaned(&mut self) -> Vec<String> {
        extern "C" {
            #[link_name = "gonameparts_name_string___cleaned__ground_truth"]
            fn NameString_cleaned__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut NameString);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(NameString_cleaned__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
            *self = *input_state_mutated.0;
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
impl NameString {
    pub(crate) fn cleaned__with_callees_mocked(&mut self) -> Vec<String> {
        let unwanted = vec![",", "."];
        let mut cleaned = Vec::new();
        for x in self.split() {
            let mut x = x.clone();
            for y in &unwanted {
                x = x.replace(y, "");
            }
            cleaned.push(x.trim().to_string());
        }
        cleaned
    }
}
#[cfg(test)]
mod gonameparts_name_string___cleaned_harness {
    use super::*;
    #[test]
    fn NameString_cleaned__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).cleaned.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
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
                    { (&mut *input_state.0).cleaned__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { (&mut *input_state.0).cleaned() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_cleaned__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).cleaned.json",
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
use std::cmp::Ordering;
#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn search_parts(&mut self, parts: &[String]) -> i32 {
        for (i, x) in self.cleaned().iter().enumerate() {
            for y in parts {
                if x.to_uppercase() == y.to_uppercase() {
                    return i as i32;
                }
            }
        }
        -1
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn search_parts(&mut self, parts: &[String]) -> i32 {
        extern "C" {
            #[link_name = "gonameparts_name_string___search_parts__ground_truth"]
            fn NameString_search_parts__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a mut NameString, &'b [String]);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let input_state_in = InputStateIn(self, parts);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameString_search_parts__foreign(ser(&params[0]), ser(&params[1])))
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
            *self = *input_state_mutated.0;
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
impl NameString {
    pub(crate) fn search_parts__with_callees_mocked(&mut self, parts: &[String]) -> i32 {
        for (i, x) in self.cleaned().iter().enumerate() {
            for y in parts {
                if x.to_uppercase() == y.to_uppercase() {
                    return i as i32;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod gonameparts_name_string___search_parts_harness {
    use super::*;
    #[test]
    fn NameString_search_parts__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).searchParts.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<[String]>);
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
                        (&mut *input_state.0)
                            .search_parts__with_callees_mocked(&*input_state.1)
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&mut *input_state.0).search_parts(&*input_state.1) }
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
    fn NameString_search_parts__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).searchParts.json",
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
use crate::nameparts::SUPPLEMENTAL_INFO;
use crate::nameparts::NON_NAME;
use crate::nameparts::SALUTATIONS;
use crate::nameparts::LN_PREFIXES;
use crate::nameparts::SUFFIXES;
use crate::nameparts::GENERATIONS;
#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn find(&mut self, part: &str) -> Result<i32, Error> {
        match part {
            "salutation" => {
                Ok(
                    self
                        .search_parts(
                            &SALUTATIONS
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                        ),
                )
            }
            "generation" => {
                Ok(
                    self
                        .search_parts(
                            &GENERATIONS
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                        ),
                )
            }
            "suffix" => {
                Ok(
                    self
                        .search_parts(
                            &SUFFIXES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                        ),
                )
            }
            "lnprefix" => Ok(self.search_parts(&LN_PREFIXES)),
            "nonname" => {
                Ok(
                    self
                        .search_parts(
                            &NON_NAME.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                        ),
                )
            }
            "supplemental" => {
                Ok(
                    self
                        .search_parts(
                            &SUPPLEMENTAL_INFO
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                        ),
                )
            }
            _ => Ok(-1),
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn find(&mut self, part: &str) -> Result<i32, Error> {
        extern "C" {
            #[link_name = "gonameparts_name_string___find__ground_truth"]
            fn NameString_find__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a mut NameString, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let input_state_in = InputStateIn(self, part);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameString_find__foreign(ser(&params[0]), ser(&params[1])))
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
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = output_state.0;
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn find__with_callees_mocked(
        &mut self,
        part: &str,
    ) -> Result<i32, Error> {
        match part {
            "salutation" => {
                Ok(
                    self
                        .search_parts(
                            &SALUTATIONS
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                        ),
                )
            }
            "generation" => {
                Ok(
                    self
                        .search_parts(
                            &GENERATIONS
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                        ),
                )
            }
            "suffix" => {
                Ok(
                    self
                        .search_parts(
                            &SUFFIXES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                        ),
                )
            }
            "lnprefix" => Ok(self.search_parts(&LN_PREFIXES)),
            "nonname" => {
                Ok(
                    self
                        .search_parts(
                            &NON_NAME.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                        ),
                )
            }
            "supplemental" => {
                Ok(
                    self
                        .search_parts(
                            &SUPPLEMENTAL_INFO
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>(),
                        ),
                )
            }
            _ => Ok(-1),
        }
    }
}
#[cfg(test)]
mod gonameparts_name_string___find_harness {
    use super::*;
    #[test]
    fn NameString_find__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).find.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<str>);
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
                        ((&mut *input_state.0)
                            .find__with_callees_mocked(&*input_state.1))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { ((&mut *input_state.0).find(&*input_state.1)).unwrap() }
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
    fn NameString_find__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).find.json",
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

#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn find_not_slotted(&self, slotted: &[usize]) -> Result<Vec<usize>> {
        let mut not_slotted = Vec::new();
        for (i, _) in self.split_name.iter().enumerate() {
            if !slotted.contains(&i) {
                not_slotted.push(i);
            }
        }
        Ok(not_slotted)
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn find_not_slotted(&self, slotted: &[usize]) -> Result<Vec<usize>> {
        extern "C" {
            #[link_name = "gonameparts_name_string___find_not_slotted__ground_truth"]
            fn NameString_find_not_slotted__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a NameString, &'b [usize]);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>, Box<[usize]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<usize>);
        let input_state_in = InputStateIn(self, slotted);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameString_find_not_slotted__foreign(ser(&params[0]), ser(&params[1])))
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
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn find_not_slotted__with_callees_mocked(
        &self,
        slotted: &[usize],
    ) -> Result<Vec<usize>> {
        let mut not_slotted = Vec::new();
        for (i, _) in self.split_name.iter().enumerate() {
            if !slotted.contains(&i) {
                not_slotted.push(i);
            }
        }
        Ok(not_slotted)
    }
}
#[cfg(test)]
mod gonameparts_name_string___find_not_slotted_harness {
    use super::*;
    #[test]
    fn NameString_find_not_slotted__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).findNotSlotted.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<[usize]>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<usize>);
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
                            .find_not_slotted__with_callees_mocked(&*input_state.1))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { ((&*input_state.0).find_not_slotted(&*input_state.1)).unwrap() }
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
    fn NameString_find_not_slotted__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<[usize]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<usize>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).findNotSlotted.json",
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

#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn fix_misplaced_apostrophe(&mut self) -> Result<()> {
        let mut ends_with_apostrophe = Vec::new();
        for (index, x) in self.split().iter().enumerate() {
            if x.ends_with('\'') {
                ends_with_apostrophe.push(index);
            }
        }
        if !ends_with_apostrophe.is_empty() {
            for y in ends_with_apostrophe {
                if self.split_name[y] == self.split_name[self.split_name.len() - 1] {
                    let mut tmp_name = self.split_name[..y].to_vec();
                    tmp_name.push(self.split_name[y].trim_matches('\'').to_string());
                    self.full_name = tmp_name.join(" ");
                } else {
                    let misplaced_start = y;
                    let mut fixed_name = vec![self.split_name[misplaced_start].clone()];
                    fixed_name.push(self.split_name[misplaced_start + 1].clone());
                    let fixed_placement = fixed_name.join("");
                    let mut tmp_name = self.split_name[..misplaced_start].to_vec();
                    tmp_name.push(fixed_placement);
                    tmp_name.extend_from_slice(&self.split_name[misplaced_start + 2..]);
                    self.full_name = tmp_name.join(" ");
                }
            }
        }
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn fix_misplaced_apostrophe(&mut self) -> Result<()> {
        extern "C" {
            #[link_name = "gonameparts_name_string___fix_misplaced_apostrophe__ground_truth"]
            fn NameString_fix_misplaced_apostrophe__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut NameString);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameString_fix_misplaced_apostrophe__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = ();
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn fix_misplaced_apostrophe__with_callees_mocked(
        &mut self,
    ) -> Result<()> {
        let mut ends_with_apostrophe = Vec::new();
        for (index, x) in self.split().iter().enumerate() {
            if x.ends_with('\'') {
                ends_with_apostrophe.push(index);
            }
        }
        if !ends_with_apostrophe.is_empty() {
            for y in ends_with_apostrophe {
                if self.split_name[y] == self.split_name[self.split_name.len() - 1] {
                    let mut tmp_name = self.split_name[..y].to_vec();
                    tmp_name.push(self.split_name[y].trim_matches('\'').to_string());
                    self.full_name = tmp_name.join(" ");
                } else {
                    let misplaced_start = y;
                    let mut fixed_name = vec![self.split_name[misplaced_start].clone()];
                    fixed_name.push(self.split_name[misplaced_start + 1].clone());
                    let fixed_placement = fixed_name.join("");
                    let mut tmp_name = self.split_name[..misplaced_start].to_vec();
                    tmp_name.push(fixed_placement);
                    tmp_name.extend_from_slice(&self.split_name[misplaced_start + 2..]);
                    self.full_name = tmp_name.join(" ");
                }
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod gonameparts_name_string___fix_misplaced_apostrophe_harness {
    use super::*;
    #[test]
    fn NameString_fix_misplaced_apostrophe__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).fixMisplacedApostrophe.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState;
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
                        ((&mut *input_state.0)
                            .fix_misplaced_apostrophe__with_callees_mocked())
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { ((&mut *input_state.0).fix_misplaced_apostrophe()).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState;
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_fix_misplaced_apostrophe__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).fixMisplacedApostrophe.json",
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

#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn has_aliases(&self) -> (bool, String) {
        let upper_name = self.full_name.to_uppercase();
        for x in NON_NAME.iter() {
            if upper_name.contains(x) && !upper_name.ends_with(x) {
                return (true, x.to_string());
            }
        }
        (false, String::new())
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn has_aliases(&self) -> (bool, String) {
        extern "C" {
            #[link_name = "gonameparts_name_string___has_aliases__ground_truth"]
            fn NameString_has_aliases__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a NameString);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool, String);
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(NameString_has_aliases__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
            let output = (output_state.0, output_state.1);
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn has_aliases__with_callees_mocked(&self) -> (bool, String) {
        let upper_name = self.full_name.to_uppercase();
        for x in NON_NAME.iter() {
            if upper_name.contains(x) && !upper_name.ends_with(x) {
                return (true, x.to_string());
            }
        }
        (false, String::new())
    }
}
#[cfg(test)]
mod gonameparts_name_string___has_aliases_harness {
    use super::*;
    #[test]
    fn NameString_has_aliases__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).hasAliases.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(bool, String);
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
                    { (&*input_state.0).has_aliases__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { (&*input_state.0).has_aliases() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value.0, return_value.1);
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_has_aliases__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool, String);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).hasAliases.json",
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

#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn has_comma(&self) -> bool {
        for part in &self.split_name {
            if str::contains(part, ",") {
                return true;
            }
        }
        false
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn has_comma(&self) -> bool {
        extern "C" {
            #[link_name = "gonameparts_name_string___has_comma__ground_truth"]
            fn NameString_has_comma__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a NameString);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(NameString_has_comma__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
impl NameString {
    pub(crate) fn has_comma__with_callees_mocked(&self) -> bool {
        for part in &self.split_name {
            if str::contains(part, ",") {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod gonameparts_name_string___has_comma_harness {
    use super::*;
    #[test]
    fn NameString_has_comma__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).hasComma.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
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
                    { (&*input_state.0).has_comma__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { (&*input_state.0).has_comma() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_has_comma__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).hasComma.json",
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
use crate::nameparts::CORP_ENTITY;
#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn looks_corporate(&mut self) -> bool {
        self.search_parts(&CORP_ENTITY.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            >= 0
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn looks_corporate(&mut self) -> bool {
        extern "C" {
            #[link_name = "gonameparts_name_string___looks_corporate__ground_truth"]
            fn NameString_looks_corporate__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut NameString);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameString_looks_corporate__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
            *self = *input_state_mutated.0;
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
impl NameString {
    pub(crate) fn looks_corporate__with_callees_mocked(&mut self) -> bool {
        self.search_parts(&CORP_ENTITY.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            >= 0
    }
}
#[cfg(test)]
mod gonameparts_name_string___looks_corporate_harness {
    use super::*;
    #[test]
    fn NameString_looks_corporate__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).looksCorporate.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
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
                    { (&mut *input_state.0).looks_corporate__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))]
                    { (&mut *input_state.0).looks_corporate() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_looks_corporate__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).looksCorporate.json",
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

#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn slot_nickname(&mut self) -> Result<(), Error> {
        let mut nick_name_boundaries = Vec::new();
        for (index, x) in self.split().iter().enumerate() {
            if x.starts_with('\'') || x.starts_with('\"') {
                nick_name_boundaries.push(index);
            }
            if x.ends_with('\'') || x.ends_with('\"') {
                nick_name_boundaries.push(index);
            }
        }
        if !nick_name_boundaries.is_empty() && nick_name_boundaries.len() % 2 == 0 {
            let nick_start = nick_name_boundaries[0];
            let nick_end = nick_name_boundaries[1];
            let mut nick = self.split_name[..nick_start].to_vec();
            let post_nick = self.split_name[nick_end + 1..].to_vec();
            self.nickname = self.split_name[nick_start..=nick_end].join(" ");
            nick.extend(post_nick);
            self.full_name = nick.join(" ");
        }
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn slot_nickname(&mut self) -> Result<(), Error> {
        extern "C" {
            #[link_name = "gonameparts_name_string___slot_nickname__ground_truth"]
            fn NameString_slot_nickname__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut NameString);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameString_slot_nickname__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = ();
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn slot_nickname__with_callees_mocked(&mut self) -> Result<(), Error> {
        let mut nick_name_boundaries = Vec::new();
        for (index, x) in self.split().iter().enumerate() {
            if x.starts_with('\'') || x.starts_with('\"') {
                nick_name_boundaries.push(index);
            }
            if x.ends_with('\'') || x.ends_with('\"') {
                nick_name_boundaries.push(index);
            }
        }
        if !nick_name_boundaries.is_empty() && nick_name_boundaries.len() % 2 == 0 {
            let nick_start = nick_name_boundaries[0];
            let nick_end = nick_name_boundaries[1];
            let mut nick = self.split_name[..nick_start].to_vec();
            let post_nick = self.split_name[nick_end + 1..].to_vec();
            self.nickname = self.split_name[nick_start..=nick_end].join(" ");
            nick.extend(post_nick);
            self.full_name = nick.join(" ");
        }
        Ok(())
    }
}
#[cfg(test)]
mod gonameparts_name_string___slot_nickname_harness {
    use super::*;
    #[test]
    fn NameString_slot_nickname__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).slotNickname.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState;
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
                        ((&mut *input_state.0).slot_nickname__with_callees_mocked())
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { ((&mut *input_state.0).slot_nickname()).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState;
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_slot_nickname__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).slotNickname.json",
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

#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn split_aliases(&mut self, alias_sep: &str) -> Result<(), Error> {
        let split_names: Vec<_> = self.split().iter().cloned().collect();
        let transformed_names: Vec<_> = split_names
            .iter()
            .map(|part| {
                if part.to_uppercase() == alias_sep.to_uppercase() {
                    "*|*".to_string()
                } else {
                    part.clone()
                }
            })
            .collect();
        let names: Vec<_> = str::replace(&transformed_names.join(" "), "*|*", "")
            .split('*')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        self.full_name = names[0].clone();
        self.aliases = names[1..].to_vec();
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn split_aliases(&mut self, alias_sep: &str) -> Result<(), Error> {
        extern "C" {
            #[link_name = "gonameparts_name_string___split_aliases__ground_truth"]
            fn NameString_split_aliases__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a mut NameString, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, alias_sep);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NameString_split_aliases__foreign(ser(&params[0]), ser(&params[1])))
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
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = ();
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn split_aliases__with_callees_mocked(
        &mut self,
        alias_sep: &str,
    ) -> Result<(), Error> {
        let split_names: Vec<_> = self.split().iter().cloned().collect();
        let transformed_names: Vec<_> = split_names
            .iter()
            .map(|part| {
                if part.to_uppercase() == alias_sep.to_uppercase() {
                    "*|*".to_string()
                } else {
                    part.clone()
                }
            })
            .collect();
        let names: Vec<_> = str::replace(&transformed_names.join(" "), "*|*", "")
            .split('*')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        self.full_name = names[0].clone();
        self.aliases = names[1..].to_vec();
        Ok(())
    }
}
#[cfg(test)]
mod gonameparts_name_string___split_aliases_harness {
    use super::*;
    #[test]
    fn NameString_split_aliases__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).splitAliases.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<str>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState;
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
                        ((&mut *input_state.0)
                            .split_aliases__with_callees_mocked(&*input_state.1))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { ((&mut *input_state.0).split_aliases(&*input_state.1)).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState;
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
    fn NameString_split_aliases__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).splitAliases.json",
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
use std::collections::HashSet;
#[cfg(not(feature = "mock"))]
impl NameString {
    pub(crate) fn normalize(&mut self) -> Result<Vec<String>, Error> {
        let (has_alias, alias_sep) = self.has_aliases();
        if has_alias {
            self.split_aliases(&alias_sep)?;
        }
        if let Ok(supplemental_index) = self.find("supplemental") {
            if supplemental_index > -1 {
                self.full_name = self
                    .split_name
                    .iter()
                    .take(supplemental_index as usize)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }
        self.slot_nickname()?;
        self.fix_misplaced_apostrophe()?;
        if self.has_comma() {
            let mut comma_split: Vec<&str> = self.full_name.split(',').collect();
            comma_split.swap(0, 1);
            let mut name_parts: Vec<String> = comma_split
                .into_iter()
                .map(|s| s.trim().to_string())
                .collect();
            name_parts.dedup();
            self.full_name = name_parts.join(" ");
        }
        let cleaned: Vec<String> = self.cleaned();
        Ok(cleaned)
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn normalize(&mut self) -> Result<Vec<String>, Error> {
        extern "C" {
            #[link_name = "gonameparts_name_string___normalize__ground_truth"]
            fn NameString_normalize__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut NameString);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(NameString_normalize__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = output_state.0;
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl NameString {
    pub(crate) fn normalize__with_callees_mocked(
        &mut self,
    ) -> Result<Vec<String>, Error> {
        let (has_alias, alias_sep) = self.has_aliases();
        if has_alias {
            self.split_aliases(&alias_sep)?;
        }
        if let Ok(supplemental_index) = self.find("supplemental") {
            if supplemental_index > -1 {
                self.full_name = self
                    .split_name
                    .iter()
                    .take(supplemental_index as usize)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }
        self.slot_nickname()?;
        self.fix_misplaced_apostrophe()?;
        if self.has_comma() {
            let mut comma_split: Vec<&str> = self.full_name.split(',').collect();
            comma_split.swap(0, 1);
            let mut name_parts: Vec<String> = comma_split
                .into_iter()
                .map(|s| s.trim().to_string())
                .collect();
            name_parts.dedup();
            self.full_name = name_parts.join(" ");
        }
        let cleaned: Vec<String> = self.cleaned();
        Ok(cleaned)
    }
}
#[cfg(test)]
mod gonameparts_name_string___normalize_harness {
    use super::*;
    #[test]
    fn NameString_normalize__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-polera-gonameparts.nameString).normalize.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
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
                    { ((&mut *input_state.0).normalize__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))]
                    { ((&mut *input_state.0).normalize()).unwrap() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NameString_normalize__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NameString>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-polera-gonameparts.nameString).normalize.json",
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
