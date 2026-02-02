use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::internal::utils::utils::max;
use crate::internal::utils::utils::min;
use crate::internal::utils::utils::equal;
#[cfg(not(feature = "mock"))]
pub fn jaro_similarity(str1: &str, str2: &str) -> Result<f32> {
    let runevec_str1: Vec<char> = str1.chars().collect();
    let runevec_str2: Vec<char> = str2.chars().collect();
    let runevec_str1len = runevec_str1.len();
    let runevec_str2len = runevec_str2.len();
    if runevec_str1len == 0 || runevec_str2len == 0 {
        return Ok(0.0);
    } else if equal(&runevec_str1, &runevec_str2)? {
        return Ok(1.0);
    }
    let mut match_count = 0;
    let max_dist = max(
        runevec_str1len.try_into().unwrap(),
        runevec_str2len.try_into().unwrap(),
    ) / 2 - 1;
    let mut str1_table = vec![0; runevec_str1len];
    let mut str2_table = vec![0; runevec_str2len];
    for i in 0..runevec_str1len {
        for j in max(
            0,
            i as i32 - max_dist as i32,
        )..min(runevec_str2len as i32, i as i32 + max_dist as i32 + 1) {
            let j = j as usize;
            if runevec_str1[i] == runevec_str2[j] && str2_table[j] == 0 {
                str1_table[i] = 1;
                str2_table[j] = 1;
                match_count += 1;
                break;
            }
        }
    }
    if match_count == 0 {
        return Ok(0.0);
    }
    let mut t = 0.0;
    let mut p = 0;
    for i in 0..runevec_str1len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if runevec_str1[i] != runevec_str2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;
    Ok(
        (match_count as f32 / runevec_str1len as f32
            + match_count as f32 / runevec_str2len as f32
            + (match_count as f32 - t) / match_count as f32) / 3.0,
    )
}
#[cfg(feature = "mock")]
pub fn jaro_similarity(str1: &str, str2: &str) -> Result<f32> {
    extern "C" {
        #[link_name = "go_edlib_jaro_similarity__ground_truth"]
        fn jaro_similarity__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    let input_state_in = InputStateIn(str1, str2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(jaro_similarity__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn jaro_similarity__with_callees_mocked(str1: &str, str2: &str) -> Result<f32> {
    let runevec_str1: Vec<char> = str1.chars().collect();
    let runevec_str2: Vec<char> = str2.chars().collect();
    let runevec_str1len = runevec_str1.len();
    let runevec_str2len = runevec_str2.len();
    if runevec_str1len == 0 || runevec_str2len == 0 {
        return Ok(0.0);
    } else if equal(&runevec_str1, &runevec_str2)? {
        return Ok(1.0);
    }
    let mut match_count = 0;
    let max_dist = max(
        runevec_str1len.try_into().unwrap(),
        runevec_str2len.try_into().unwrap(),
    ) / 2 - 1;
    let mut str1_table = vec![0; runevec_str1len];
    let mut str2_table = vec![0; runevec_str2len];
    for i in 0..runevec_str1len {
        for j in max(
            0,
            i as i32 - max_dist as i32,
        )..min(runevec_str2len as i32, i as i32 + max_dist as i32 + 1) {
            let j = j as usize;
            if runevec_str1[i] == runevec_str2[j] && str2_table[j] == 0 {
                str1_table[i] = 1;
                str2_table[j] = 1;
                match_count += 1;
                break;
            }
        }
    }
    if match_count == 0 {
        return Ok(0.0);
    }
    let mut t = 0.0;
    let mut p = 0;
    for i in 0..runevec_str1len {
        if str1_table[i] == 1 {
            while str2_table[p] == 0 {
                p += 1;
            }
            if runevec_str1[i] != runevec_str2[p] {
                t += 1.0;
            }
            p += 1;
        }
    }
    t /= 2.0;
    Ok(
        (match_count as f32 / runevec_str1len as f32
            + match_count as f32 / runevec_str2len as f32
            + (match_count as f32 - t) / match_count as f32) / 3.0,
    )
}
#[cfg(test)]
mod go_edlib_jaro_similarity_harness {
    use super::*;
    #[test]
    fn jaro_similarity__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-hbollon-go-edlib.JaroSimilarity.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<str>);
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
                        (jaro_similarity__with_callees_mocked(
                            &*input_state.0,
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (jaro_similarity(&*input_state.0, &*input_state.1)).unwrap() }
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
    fn jaro_similarity__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-hbollon-go-edlib.JaroSimilarity.json",
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
pub fn jaro_winkler_similarity(str1: &str, str2: &str) -> Result<f32, Error> {
    let jaro_sim = jaro_similarity(str1, str2)?;
    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        let str1_chars: Vec<char> = str1.chars().collect();
        let str2_chars: Vec<char> = str2.chars().collect();
        let str1_len = str1_chars.len();
        let str2_len = str2_chars.len();
        let mut prefix = 0;
        for (i, (c1, c2)) in str1_chars.iter().zip(str2_chars.iter()).enumerate() {
            if c1 == c2 {
                prefix += 1;
            } else {
                break;
            }
        }
        let prefix = min(prefix, 4);
        Ok(jaro_sim + 0.1 * (prefix as f32) * (1.0 - jaro_sim))
    } else {
        Ok(jaro_sim)
    }
}
#[cfg(feature = "mock")]
pub fn jaro_winkler_similarity(str1: &str, str2: &str) -> Result<f32, Error> {
    extern "C" {
        #[link_name = "go_edlib_jaro_winkler_similarity__ground_truth"]
        fn jaro_winkler_similarity__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    let input_state_in = InputStateIn(str1, str2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(jaro_winkler_similarity__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn jaro_winkler_similarity__with_callees_mocked(
    str1: &str,
    str2: &str,
) -> Result<f32, Error> {
    let jaro_sim = jaro_similarity(str1, str2)?;
    if jaro_sim != 0.0 && jaro_sim != 1.0 {
        let str1_chars: Vec<char> = str1.chars().collect();
        let str2_chars: Vec<char> = str2.chars().collect();
        let str1_len = str1_chars.len();
        let str2_len = str2_chars.len();
        let mut prefix = 0;
        for (i, (c1, c2)) in str1_chars.iter().zip(str2_chars.iter()).enumerate() {
            if c1 == c2 {
                prefix += 1;
            } else {
                break;
            }
        }
        let prefix = min(prefix, 4);
        Ok(jaro_sim + 0.1 * (prefix as f32) * (1.0 - jaro_sim))
    } else {
        Ok(jaro_sim)
    }
}
#[cfg(test)]
mod go_edlib_jaro_winkler_similarity_harness {
    use super::*;
    #[test]
    fn jaro_winkler_similarity__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-hbollon-go-edlib.JaroWinklerSimilarity.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<str>);
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
                        (jaro_winkler_similarity__with_callees_mocked(
                            &*input_state.0,
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (jaro_winkler_similarity(&*input_state.0, &*input_state.1))
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
    fn jaro_winkler_similarity__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-hbollon-go-edlib.JaroWinklerSimilarity.json",
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
