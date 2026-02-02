use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashMap;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct LanguageDefault {
    #[serde(rename = "defaultLang")]
    pub(crate) default_lang: String,
    #[serde(rename = "languages")]
    pub(crate) languages: HashMap<String, Vec<String>>,
}
#[cfg(test)]
mod TextRank_LanguageDefault_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "TextRank_LanguageDefault_roundtrip"]
        fn LanguageDefault__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn LanguageDefault__weak__interoperation() {
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
                                "(*{}).", "github.com-DavidBelicza-TextRank.LanguageDefault"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-DavidBelicza-TextRank.LanguageDefault"
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
                                    serde_json::from_value::<LanguageDefault>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<LanguageDefault>(obj_once.clone())
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
pub trait Language: crate::__synthetic::__Synth2__is_stop_word + crate::__synthetic::__Synth0__find_root_word + crate::__synthetic::__Synth4__set_words {}
use crate::__synthetic::__Synth0__find_root_word;
#[cfg(not(feature = "mock"))]
impl __Synth0__find_root_word for LanguageDefault {
    fn find_root_word(&self, _input1: &str) -> (bool, String) {
        (false, String::new())
    }
}
#[cfg(feature = "mock")]
impl __Synth0__find_root_word for LanguageDefault {
    fn find_root_word(&self, _input1: &str) -> (bool, String) {
        extern "C" {
            #[link_name = "TextRank_language_default___find_root_word__ground_truth"]
            fn LanguageDefault_find_root_word__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a LanguageDefault, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<LanguageDefault>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool, String);
        let input_state_in = InputStateIn(self, _input1);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(LanguageDefault_find_root_word__foreign(ser(&params[0]), ser(&params[1])))
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
            let output = (output_state.0, output_state.1);
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl LanguageDefault {
    fn find_root_word__with_callees_mocked(&self, _input1: &str) -> (bool, String) {
        (false, String::new())
    }
}
#[cfg(test)]
mod TextRank_language_default___find_root_word_harness {
    use super::*;
    #[test]
    fn LanguageDefault_find_root_word__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.LanguageDefault).FindRootWord.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<LanguageDefault>, Box<str>);
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
                    {
                        (&*input_state.0)
                            .find_root_word__with_callees_mocked(&*input_state.1)
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).find_root_word(&*input_state.1) }
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
    fn LanguageDefault_find_root_word__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<LanguageDefault>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool, String);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.LanguageDefault).FindRootWord.json",
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
use crate::__synthetic::__Synth2__is_stop_word;
#[cfg(not(feature = "mock"))]
impl __Synth2__is_stop_word for LanguageDefault {
    fn is_stop_word(&self, word: &str) -> bool {
        if word.chars().count() <= 2 {
            return true;
        }
        if let Some(stop_words) = self.languages.get(&self.default_lang) {
            if stop_words.contains(&word.to_string()) {
                return true;
            }
        }
        false
    }
}
#[cfg(feature = "mock")]
impl __Synth2__is_stop_word for LanguageDefault {
    fn is_stop_word(&self, word: &str) -> bool {
        extern "C" {
            #[link_name = "TextRank_language_default___is_stop_word__ground_truth"]
            fn LanguageDefault_is_stop_word__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a LanguageDefault, &'b str);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<LanguageDefault>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let input_state_in = InputStateIn(self, word);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(LanguageDefault_is_stop_word__foreign(ser(&params[0]), ser(&params[1])))
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
impl LanguageDefault {
    fn is_stop_word__with_callees_mocked(&self, word: &str) -> bool {
        if word.chars().count() <= 2 {
            return true;
        }
        if let Some(stop_words) = self.languages.get(&self.default_lang) {
            if stop_words.contains(&word.to_string()) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod TextRank_language_default___is_stop_word_harness {
    use super::*;
    #[test]
    fn LanguageDefault_is_stop_word__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.LanguageDefault).IsStopWord.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<LanguageDefault>, Box<str>);
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
                            .is_stop_word__with_callees_mocked(&*input_state.1)
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).is_stop_word(&*input_state.1) }
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
    fn LanguageDefault_is_stop_word__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<LanguageDefault>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.LanguageDefault).IsStopWord.json",
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
use crate::__synthetic::__Synth4__set_words;
#[cfg(not(feature = "mock"))]
impl __Synth4__set_words for LanguageDefault {
    fn set_words(&mut self, code: &str, words: &[String]) {
        self.languages.insert(code.to_string(), words.to_vec());
    }
}
#[cfg(feature = "mock")]
impl __Synth4__set_words for LanguageDefault {
    fn set_words(&mut self, code: &str, words: &[String]) {
        extern "C" {
            #[link_name = "TextRank_language_default___set_words__ground_truth"]
            fn LanguageDefault_set_words__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b, 'c>(&'a mut LanguageDefault, &'b str, &'c [String]);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<LanguageDefault>, Box<str>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, code, words);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                LanguageDefault_set_words__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 3usize);
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
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl LanguageDefault {
    fn set_words__with_callees_mocked(&mut self, code: &str, words: &[String]) {
        self.languages.insert(code.to_string(), words.to_vec());
    }
}
#[cfg(test)]
mod TextRank_language_default___set_words_harness {
    use super::*;
    #[test]
    fn LanguageDefault_set_words__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.LanguageDefault).SetWords.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<LanguageDefault>, Box<str>, Box<[String]>);
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
                        (&mut *input_state.0)
                            .set_words__with_callees_mocked(
                                &*input_state.1,
                                &*input_state.2,
                            )
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&mut *input_state.0).set_words(&*input_state.1, &*input_state.2) }
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
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.2).unwrap(),
                        serde_json::to_value(& input_state_mutated.2).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn LanguageDefault_set_words__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<LanguageDefault>, Box<str>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.LanguageDefault).SetWords.json",
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

#[typetag::serde(name = "LanguageDefault")]
impl crate::language::Language for LanguageDefault {}
use crate::stop_word::get_default_english;
#[cfg(not(feature = "mock"))]
pub fn new_language() -> Result<LanguageDefault, Error> {
    let mut lang = LanguageDefault {
        default_lang: "en".to_string(),
        languages: HashMap::new(),
    };
    let words = get_default_english();
    lang.set_words("en", &words);
    Ok(lang)
}
#[cfg(feature = "mock")]
pub fn new_language() -> Result<LanguageDefault, Error> {
    extern "C" {
        #[link_name = "TextRank_new_language__ground_truth"]
        fn new_language__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(LanguageDefault);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_language__foreign()) };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 0usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
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
#[cfg(feature = "mock")]
pub fn new_language__with_callees_mocked() -> Result<LanguageDefault, Error> {
    let mut lang = LanguageDefault {
        default_lang: "en".to_string(),
        languages: HashMap::new(),
    };
    let words = get_default_english();
    lang.set_words("en", &words);
    Ok(lang)
}
#[cfg(test)]
mod TextRank_new_language_harness {
    use super::*;
    #[test]
    fn new_language__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.NewLanguage.json",
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
        struct OutputState(LanguageDefault);
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
                    { (new_language__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (new_language()).unwrap() }
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
    fn new_language__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(LanguageDefault);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.NewLanguage.json",
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
