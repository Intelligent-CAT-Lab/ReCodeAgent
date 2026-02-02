use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use std::collections::HashMap;
use crate::text::ParsedSentence;
use crate::rank::Rank;
#[cfg(not(feature = "mock"))]
pub(crate) fn add_sentence(
    ranks: &mut Rank,
    sentence: ParsedSentence,
) -> Result<i32, Error> {
    let sentence_id = ranks.sentence_map.len() as i32;
    ranks.sentence_map.insert(sentence_id, sentence.original.clone());
    Ok(sentence_id)
}
#[cfg(feature = "mock")]
pub(crate) fn add_sentence(
    ranks: &mut Rank,
    sentence: ParsedSentence,
) -> Result<i32, Error> {
    extern "C" {
        #[link_name = "TextRank_add_sentence__ground_truth"]
        fn add_sentence__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a mut Rank, ParsedSentence);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, ParsedSentence);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(ranks, sentence);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(add_sentence__foreign(ser(&params[0]), ser(&params[1])))
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
        *ranks = *input_state_mutated.0;
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
pub(crate) fn add_sentence__with_callees_mocked(
    ranks: &mut Rank,
    sentence: ParsedSentence,
) -> Result<i32, Error> {
    let sentence_id = ranks.sentence_map.len() as i32;
    ranks.sentence_map.insert(sentence_id, sentence.original.clone());
    Ok(sentence_id)
}
#[cfg(test)]
mod TextRank_add_sentence_harness {
    use super::*;
    #[test]
    fn add_sentence__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.addSentence.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, ParsedSentence);
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
                        (add_sentence__with_callees_mocked(
                            &mut *input_state.0,
                            input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (add_sentence(&mut *input_state.0, input_state.1)).unwrap() }
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
    fn add_sentence__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, ParsedSentence);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.addSentence.json",
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
use crate::__synthetic::__Synth0__find_root_word;
use crate::language::Language;
use crate::relation::Relation;
use crate::__synthetic::__Synth2__is_stop_word;
#[cfg(not(feature = "mock"))]
pub(crate) fn add_word(
    ranks: &mut Rank,
    words: &[String],
    lang: &dyn Language,
    sentence_id: i32,
) -> Result<(), Error> {
    let mut prev_word_id = -1;
    for word in words.iter().filter(|w| !lang.is_stop_word(w.as_str())) {
        let (found, root_word) = lang.find_root_word(word.as_str());
        let word = if found { root_word } else { word.clone() };
        let cur_word_id = if ranks.is_word_exist(&word) {
            ranks.update_word(&word, prev_word_id, sentence_id)?
        } else {
            ranks.add_new_word(&word, prev_word_id, sentence_id)
        };
        ranks.relation.add_relation(cur_word_id, prev_word_id, sentence_id)?;
        ranks.update_right_connection(prev_word_id, cur_word_id)?;
        prev_word_id = cur_word_id;
    }
    Ok(())
}
#[cfg(feature = "mock")]
pub(crate) fn add_word(
    ranks: &mut Rank,
    words: &[String],
    lang: &dyn Language,
    sentence_id: i32,
) -> Result<(), Error> {
    extern "C" {
        #[link_name = "TextRank_add_word__ground_truth"]
        fn add_word__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b, 'c>(&'a mut Rank, &'b [String], &'c dyn Language, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, Box<[String]>, Box<dyn Language>, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(ranks, words, lang, sentence_id);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(
            add_word__foreign(
                ser(&params[0]),
                ser(&params[1]),
                ser(&params[2]),
                ser(&params[3]),
            ),
        )
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 4usize);
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
        *ranks = *input_state_mutated.0;
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
#[cfg(feature = "mock")]
pub(crate) fn add_word__with_callees_mocked(
    ranks: &mut Rank,
    words: &[String],
    lang: &dyn Language,
    sentence_id: i32,
) -> Result<(), Error> {
    let mut prev_word_id = -1;
    for word in words.iter().filter(|w| !lang.is_stop_word(w.as_str())) {
        let (found, root_word) = lang.find_root_word(word.as_str());
        let word = if found { root_word } else { word.clone() };
        let cur_word_id = if ranks.is_word_exist(&word) {
            ranks.update_word(&word, prev_word_id, sentence_id)?
        } else {
            ranks.add_new_word(&word, prev_word_id, sentence_id)
        };
        ranks.relation.add_relation(cur_word_id, prev_word_id, sentence_id)?;
        ranks.update_right_connection(prev_word_id, cur_word_id)?;
        prev_word_id = cur_word_id;
    }
    Ok(())
}
#[cfg(test)]
mod TextRank_add_word_harness {
    use super::*;
    #[test]
    fn add_word__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.addWord.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, Box<[String]>, Box<dyn Language>, i32);
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
                        (add_word__with_callees_mocked(
                            &mut *input_state.0,
                            &*input_state.1,
                            &*input_state.2,
                            input_state.3,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (add_word(
                            &mut *input_state.0,
                            &*input_state.1,
                            &*input_state.2,
                            input_state.3,
                        ))
                            .unwrap()
                    }
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
    fn add_word__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, Box<[String]>, Box<dyn Language>, i32);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.addWord.json",
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
pub fn text_to_rank(
    sentence: ParsedSentence,
    lang: &dyn Language,
    ranks: &mut Rank,
) -> Result<(), Error> {
    let sentence_id = add_sentence(ranks, sentence.clone())?;
    add_word(ranks, sentence.get_words()?, lang, sentence_id)?;
    Ok(())
}
#[cfg(feature = "mock")]
pub fn text_to_rank(
    sentence: ParsedSentence,
    lang: &dyn Language,
    ranks: &mut Rank,
) -> Result<(), Error> {
    extern "C" {
        #[link_name = "TextRank_text_to_rank__ground_truth"]
        fn text_to_rank__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(ParsedSentence, &'a dyn Language, &'b mut Rank);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(ParsedSentence, Box<dyn Language>, Box<Rank>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(sentence, lang, ranks);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(text_to_rank__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 3usize);
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
        *ranks = *input_state_mutated.2;
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
#[cfg(feature = "mock")]
pub fn text_to_rank__with_callees_mocked(
    sentence: ParsedSentence,
    lang: &dyn Language,
    ranks: &mut Rank,
) -> Result<(), Error> {
    let sentence_id = add_sentence(ranks, sentence.clone())?;
    add_word(ranks, sentence.get_words()?, lang, sentence_id)?;
    Ok(())
}
#[cfg(test)]
mod TextRank_text_to_rank_harness {
    use super::*;
    #[test]
    fn text_to_rank__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.TextToRank.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(ParsedSentence, Box<dyn Language>, Box<Rank>);
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
                        (text_to_rank__with_callees_mocked(
                            input_state.0,
                            &*input_state.1,
                            &mut *input_state.2,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (text_to_rank(
                            input_state.0,
                            &*input_state.1,
                            &mut *input_state.2,
                        ))
                            .unwrap()
                    }
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
    fn text_to_rank__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(ParsedSentence, Box<dyn Language>, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.TextToRank.json",
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
