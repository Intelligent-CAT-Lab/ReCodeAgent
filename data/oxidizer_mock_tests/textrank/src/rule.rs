use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::vec::Vec;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct RuleDefault {
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "wordSeparators")]
    pub(crate) word_separators: Vec<String>,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "sentenceSeparators")]
    pub(crate) sentence_separators: Vec<String>,
}
#[cfg(test)]
mod TextRank_RuleDefault_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "TextRank_RuleDefault_roundtrip"]
        fn RuleDefault__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn RuleDefault__weak__interoperation() {
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
                                "(*{}).", "github.com-DavidBelicza-TextRank.RuleDefault"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-DavidBelicza-TextRank.RuleDefault"
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
                                    serde_json::from_value::<RuleDefault>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<RuleDefault>(obj_once.clone())
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

#[typetag::serde(tag = "Type", content = "Value")]
pub trait Rule: crate::__synthetic::__Synth3__is_word_separator + crate::__synthetic::__Synth1__is_sentence_separator {}
use crate::__synthetic::__Synth1__is_sentence_separator;
#[cfg(not(feature = "mock"))]
impl __Synth1__is_sentence_separator for RuleDefault {
    fn is_sentence_separator(&self, c: char) -> bool {
        for separator in &self.sentence_separators {
            if separator.chars().next().unwrap() == c {
                return true;
            }
        }
        false
    }
}
#[cfg(feature = "mock")]
impl __Synth1__is_sentence_separator for RuleDefault {
    fn is_sentence_separator(&self, c: char) -> bool {
        extern "C" {
            #[link_name = "TextRank_rule_default___is_sentence_separator__ground_truth"]
            fn RuleDefault_is_sentence_separator__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a RuleDefault, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<RuleDefault>, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, c);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                RuleDefault_is_sentence_separator__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                ),
            )
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
impl RuleDefault {
    fn is_sentence_separator__with_callees_mocked(&self, c: char) -> bool {
        for separator in &self.sentence_separators {
            if separator.chars().next().unwrap() == c {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod TextRank_rule_default___is_sentence_separator_harness {
    use super::*;
    #[test]
    fn RuleDefault_is_sentence_separator__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.RuleDefault).IsSentenceSeparator.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<RuleDefault>, char);
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
                    {
                        (&*input_state.0)
                            .is_sentence_separator__with_callees_mocked(input_state.1)
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).is_sentence_separator(input_state.1) }
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
    fn RuleDefault_is_sentence_separator__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<RuleDefault>, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.RuleDefault).IsSentenceSeparator.json",
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
use crate::__synthetic::__Synth3__is_word_separator;
use std::string::ToString;
#[cfg(not(feature = "mock"))]
impl __Synth3__is_word_separator for RuleDefault {
    fn is_word_separator(&self, c: char) -> bool {
        let chr = c.to_string();
        for val in &self.word_separators {
            if chr == *val {
                return true;
            }
        }
        self.is_sentence_separator(c)
    }
}
#[cfg(feature = "mock")]
impl __Synth3__is_word_separator for RuleDefault {
    fn is_word_separator(&self, c: char) -> bool {
        extern "C" {
            #[link_name = "TextRank_rule_default___is_word_separator__ground_truth"]
            fn RuleDefault_is_word_separator__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a RuleDefault, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<RuleDefault>, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, c);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(RuleDefault_is_word_separator__foreign(ser(&params[0]), ser(&params[1])))
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
impl RuleDefault {
    fn is_word_separator__with_callees_mocked(&self, c: char) -> bool {
        let chr = c.to_string();
        for val in &self.word_separators {
            if chr == *val {
                return true;
            }
        }
        self.is_sentence_separator(c)
    }
}
#[cfg(test)]
mod TextRank_rule_default___is_word_separator_harness {
    use super::*;
    #[test]
    fn RuleDefault_is_word_separator__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.RuleDefault).IsWordSeparator.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<RuleDefault>, char);
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
                    {
                        (&*input_state.0)
                            .is_word_separator__with_callees_mocked(input_state.1)
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).is_word_separator(input_state.1) }
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
    fn RuleDefault_is_word_separator__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<RuleDefault>, char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.RuleDefault).IsWordSeparator.json",
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

#[typetag::serde(name = "RuleDefault")]
impl crate::rule::Rule for RuleDefault {}

#[cfg(not(feature = "mock"))]
impl RuleDefault {
    pub fn new() -> Result<RuleDefault> {
        let word_separators = vec![
            " ".to_string(), ",".to_string(), "'".to_string(), "'".to_string(), "\""
            .to_string(), ")".to_string(), "(".to_string(), "[".to_string(), "]"
            .to_string(), "{".to_string(), "}".to_string(), "\"".to_string(), ";"
            .to_string(), "\n".to_string(), ">".to_string(), "<".to_string(), "%"
            .to_string(), "@".to_string(), "&".to_string(), "=".to_string(), "#"
            .to_string(),
        ];
        let sentence_separators = vec![
            "!".to_string(), ".".to_string(), "?".to_string(),
        ];
        Ok(RuleDefault {
            word_separators,
            sentence_separators,
        })
    }
}
#[cfg(feature = "mock")]
impl RuleDefault {
    pub fn new() -> Result<RuleDefault> {
        extern "C" {
            #[link_name = "TextRank_new_rule__ground_truth"]
            fn RuleDefault_new__foreign() -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(RuleDefault);
        let input_state_in = InputStateIn();
        let foreign_execution = unsafe {
            de::<ForeignExecution>(RuleDefault_new__foreign())
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 0usize);
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
impl RuleDefault {
    pub fn new__with_callees_mocked() -> Result<RuleDefault> {
        let word_separators = vec![
            " ".to_string(), ",".to_string(), "'".to_string(), "'".to_string(), "\""
            .to_string(), ")".to_string(), "(".to_string(), "[".to_string(), "]"
            .to_string(), "{".to_string(), "}".to_string(), "\"".to_string(), ";"
            .to_string(), "\n".to_string(), ">".to_string(), "<".to_string(), "%"
            .to_string(), "@".to_string(), "&".to_string(), "=".to_string(), "#"
            .to_string(),
        ];
        let sentence_separators = vec![
            "!".to_string(), ".".to_string(), "?".to_string(),
        ];
        Ok(RuleDefault {
            word_separators,
            sentence_separators,
        })
    }
}
#[cfg(test)]
mod TextRank_new_rule_harness {
    use super::*;
    #[test]
    fn RuleDefault_new__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.NewRule.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(RuleDefault);
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
                    { (RuleDefault::new__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (RuleDefault::new()).unwrap() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn RuleDefault_new__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(RuleDefault);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.NewRule.json",
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
