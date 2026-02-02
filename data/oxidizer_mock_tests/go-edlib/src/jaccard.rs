use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::cosine::union;
use crate::shingle::shingle_slice;
#[cfg(not(feature = "mock"))]
pub fn jaccard_similarity(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32, Error> {
    if str1.is_empty() || str2.is_empty() {
        return Ok(0.0);
    }
    let split_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str1, split_length)?
    };
    let split_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str2, split_length)?
    };
    let rune_str1: Vec<Vec<char>> = split_str1
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let rune_str2: Vec<Vec<char>> = split_str2
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let union_str = union(&split_str1, &split_str2)?;
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;
    Ok(jacc / union_str.len() as f32)
}
#[cfg(feature = "mock")]
pub fn jaccard_similarity(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32, Error> {
    extern "C" {
        #[link_name = "go_edlib_jaccard_similarity__ground_truth"]
        fn jaccard_similarity__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    let input_state_in = InputStateIn(str1, str2, split_length);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(jaccard_similarity__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn jaccard_similarity__with_callees_mocked(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32, Error> {
    if str1.is_empty() || str2.is_empty() {
        return Ok(0.0);
    }
    let split_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str1, split_length)?
    };
    let split_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(String::from).collect()
    } else {
        shingle_slice(str2, split_length)?
    };
    let rune_str1: Vec<Vec<char>> = split_str1
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let rune_str2: Vec<Vec<char>> = split_str2
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let union_str = union(&split_str1, &split_str2)?;
    let jacc = (rune_str1.len() + rune_str2.len() - union_str.len()) as f32;
    Ok(jacc / union_str.len() as f32)
}
#[cfg(test)]
mod go_edlib_jaccard_similarity_harness {
    use super::*;
    #[test]
    fn jaccard_similarity__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-hbollon-go-edlib.JaccardSimilarity.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<str>, usize);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
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
                        (jaccard_similarity__with_callees_mocked(
                            &*input_state.0,
                            &*input_state.1,
                            input_state.2,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (jaccard_similarity(
                            &*input_state.0,
                            &*input_state.1,
                            input_state.2,
                        ))
                            .unwrap()
                    }
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
    fn jaccard_similarity__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<str>, usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-hbollon-go-edlib.JaccardSimilarity.json",
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
