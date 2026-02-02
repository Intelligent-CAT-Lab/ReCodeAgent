use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::__synthetic::__Synth3__is_word_separator;
use crate::rule::Rule;
#[cfg(not(feature = "mock"))]
pub(crate) fn find_words(raw_sentence: &str, rule: &dyn Rule) -> Result<Vec<String>> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut i = 0;
    let slen = raw_sentence.len();
    for (j, chr) in raw_sentence.char_indices() {
        let chrlen = chr.len_utf8();
        let j = j + chrlen;
        if rule.is_word_separator(chr) || j == slen {
            if rule.is_word_separator(chr) {
                word = raw_sentence[i..j - chrlen].to_string();
            } else {
                word = raw_sentence[i..j].to_string();
            }
            if !word.is_empty() {
                words.push(word.to_lowercase());
            }
            word.clear();
            i = j;
        }
    }
    Ok(words)
}
#[cfg(feature = "mock")]
pub(crate) fn find_words(raw_sentence: &str, rule: &dyn Rule) -> Result<Vec<String>> {
    extern "C" {
        #[link_name = "TextRank_find_words__ground_truth"]
        fn find_words__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b dyn Rule);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<dyn Rule>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
    let input_state_in = InputStateIn(raw_sentence, rule);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(find_words__foreign(ser(&params[0]), ser(&params[1])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 2usize);
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
pub(crate) fn find_words__with_callees_mocked(
    raw_sentence: &str,
    rule: &dyn Rule,
) -> Result<Vec<String>> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut i = 0;
    let slen = raw_sentence.len();
    for (j, chr) in raw_sentence.char_indices() {
        let chrlen = chr.len_utf8();
        let j = j + chrlen;
        if rule.is_word_separator(chr) || j == slen {
            if rule.is_word_separator(chr) {
                word = raw_sentence[i..j - chrlen].to_string();
            } else {
                word = raw_sentence[i..j].to_string();
            }
            if !word.is_empty() {
                words.push(word.to_lowercase());
            }
            word.clear();
            i = j;
        }
    }
    Ok(words)
}
#[cfg(test)]
mod TextRank_find_words_harness {
    use super::*;
    #[test]
    fn find_words__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.findWords.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<dyn Rule>);
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
                    {
                        (find_words__with_callees_mocked(
                            &*input_state.0,
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (find_words(&*input_state.0, &*input_state.1)).unwrap() }
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
    fn find_words__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<dyn Rule>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<String>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.findWords.json",
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
use crate::text::Text;
use crate::__synthetic::__Synth1__is_sentence_separator;
#[cfg(not(feature = "mock"))]
pub fn tokenize_text(raw_text: &str, rule: &dyn Rule) -> Result<Text> {
    let mut text = Text::default();
    let mut sentence = String::new();
    let mut i = 0;
    let slen = raw_text.len();
    for (j, chr) in raw_text.char_indices() {
        let j = j + chr.len_utf8();
        if rule.is_sentence_separator(chr) || j == slen {
            sentence = raw_text[i..j].to_string();
            if !sentence.is_empty() {
                text.append(&sentence, &find_words(&sentence, rule)?)?;
            }
            sentence.clear();
            i = j;
        }
    }
    Ok(text)
}
#[cfg(feature = "mock")]
pub fn tokenize_text(raw_text: &str, rule: &dyn Rule) -> Result<Text> {
    extern "C" {
        #[link_name = "TextRank_tokenize_text__ground_truth"]
        fn tokenize_text__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b dyn Rule);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<dyn Rule>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Text);
    let input_state_in = InputStateIn(raw_text, rule);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(tokenize_text__foreign(ser(&params[0]), ser(&params[1])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 2usize);
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
pub fn tokenize_text__with_callees_mocked(
    raw_text: &str,
    rule: &dyn Rule,
) -> Result<Text> {
    let mut text = Text::default();
    let mut sentence = String::new();
    let mut i = 0;
    let slen = raw_text.len();
    for (j, chr) in raw_text.char_indices() {
        let j = j + chr.len_utf8();
        if rule.is_sentence_separator(chr) || j == slen {
            sentence = raw_text[i..j].to_string();
            if !sentence.is_empty() {
                text.append(&sentence, &find_words(&sentence, rule)?)?;
            }
            sentence.clear();
            i = j;
        }
    }
    Ok(text)
}
#[cfg(test)]
mod TextRank_tokenize_text_harness {
    use super::*;
    #[test]
    fn tokenize_text__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.TokenizeText.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<dyn Rule>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(Text);
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
                        (tokenize_text__with_callees_mocked(
                            &*input_state.0,
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (tokenize_text(&*input_state.0, &*input_state.1)).unwrap() }
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
    fn tokenize_text__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<dyn Rule>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Text);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.TokenizeText.json",
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
