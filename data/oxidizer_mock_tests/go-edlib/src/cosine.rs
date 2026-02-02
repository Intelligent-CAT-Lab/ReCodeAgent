use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::internal::utils::utils::equal;
#[cfg(not(feature = "mock"))]
pub(crate) fn find(slice: &[Vec<char>], val: &[char]) -> Result<i32, Error> {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, val)? {
            return Ok(i as i32);
        }
    }
    Ok(-1)
}
#[cfg(feature = "mock")]
pub(crate) fn find(slice: &[Vec<char>], val: &[char]) -> Result<i32, Error> {
    extern "C" {
        #[link_name = "go_edlib_find__ground_truth"]
        fn find__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a [Vec<char>], &'b [char]);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<[Vec<char>]>, Box<[char]>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(slice, val);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(find__foreign(ser(&params[0]), ser(&params[1])))
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
pub(crate) fn find__with_callees_mocked(
    slice: &[Vec<char>],
    val: &[char],
) -> Result<i32, Error> {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, val)? {
            return Ok(i as i32);
        }
    }
    Ok(-1)
}
#[cfg(test)]
mod go_edlib_find_harness {
    use super::*;
    #[test]
    fn find__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-hbollon-go-edlib.find.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<[Vec<char>]>, Box<[char]>);
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
                        (find__with_callees_mocked(&*input_state.0, &*input_state.1))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (find(&*input_state.0, &*input_state.1)).unwrap() }
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
    fn find__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<[Vec<char>]>, Box<[char]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-hbollon-go-edlib.find.json",
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
pub(crate) fn sum(arr: &[i32]) -> i32 {
    let mut res = 0;
    for v in arr {
        res += v;
    }
    res
}
#[cfg(feature = "mock")]
pub(crate) fn sum(arr: &[i32]) -> i32 {
    extern "C" {
        #[link_name = "go_edlib_sum__ground_truth"]
        fn sum__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a [i32]);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<[i32]>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(arr);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(sum__foreign(ser(&input_state_in)))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub(crate) fn sum__with_callees_mocked(arr: &[i32]) -> i32 {
    let mut res = 0;
    for v in arr {
        res += v;
    }
    res
}
#[cfg(test)]
mod go_edlib_sum_harness {
    use super::*;
    #[test]
    fn sum__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-hbollon-go-edlib.sum.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<[i32]>);
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
                    { sum__with_callees_mocked(&*input_state.0) }
                    #[cfg(not(feature = "mock"))] { sum(&*input_state.0) }
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
    fn sum__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<[i32]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(i32);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-hbollon-go-edlib.sum.json",
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
pub(crate) fn union(a: &[String], b: &[String]) -> Result<Vec<Vec<char>>, Error> {
    let mut set: HashSet<String> = HashSet::new();
    for item in a {
        set.insert(item.clone());
    }
    let mut out: Vec<String> = a.to_vec();
    for item in b {
        if !set.contains(item) {
            out.push(item.clone());
        }
    }
    let out: Vec<Vec<char>> = out.into_iter().map(|s| s.chars().collect()).collect();
    Ok(out)
}
#[cfg(feature = "mock")]
pub(crate) fn union(a: &[String], b: &[String]) -> Result<Vec<Vec<char>>, Error> {
    extern "C" {
        #[link_name = "go_edlib_union__ground_truth"]
        fn union__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a [String], &'b [String]);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<[String]>, Box<[String]>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Vec<char>>);
    let input_state_in = InputStateIn(a, b);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(union__foreign(ser(&params[0]), ser(&params[1])))
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
pub(crate) fn union__with_callees_mocked(
    a: &[String],
    b: &[String],
) -> Result<Vec<Vec<char>>, Error> {
    let mut set: HashSet<String> = HashSet::new();
    for item in a {
        set.insert(item.clone());
    }
    let mut out: Vec<String> = a.to_vec();
    for item in b {
        if !set.contains(item) {
            out.push(item.clone());
        }
    }
    let out: Vec<Vec<char>> = out.into_iter().map(|s| s.chars().collect()).collect();
    Ok(out)
}
#[cfg(test)]
mod go_edlib_union_harness {
    use super::*;
    #[test]
    fn union__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-hbollon-go-edlib.union.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<[String]>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Vec<char>>);
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
                        (union__with_callees_mocked(&*input_state.0, &*input_state.1))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (union(&*input_state.0, &*input_state.1)).unwrap() }
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
    fn union__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<[String]>, Box<[String]>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Vec<char>>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-hbollon-go-edlib.union.json",
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
use crate::shingle::shingle_slice;
#[cfg(not(feature = "mock"))]
pub fn cosine_similarity(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32, Error> {
    if str1.is_empty() || str2.is_empty() {
        return Ok(0.0);
    }
    let splitted_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str1, split_length)?
    };
    let splitted_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str2, split_length)?
    };
    let rune_str1: Vec<Vec<char>> = splitted_str1
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    let union_str: Vec<Vec<char>> = union(&splitted_str1, &splitted_str2)?;
    for word in &union_str {
        let fw = find(&rune_str1, &word)?;
        l1.push(if fw != -1 { 1 } else { 0 });
        let fw = find(&rune_str2, &word)?;
        l2.push(if fw != -1 { 1 } else { 0 });
    }
    let mut cosine_sim: f32 = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += (l1[i] * l2[i]) as f32;
    }
    let l1_sum: f32 = sum(&l1) as f32;
    let l2_sum: f32 = sum(&l2) as f32;
    Ok(cosine_sim / (l1_sum.sqrt() * l2_sum.sqrt()))
}
#[cfg(feature = "mock")]
pub fn cosine_similarity(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32, Error> {
    extern "C" {
        #[link_name = "go_edlib_cosine_similarity__ground_truth"]
        fn cosine_similarity__foreign(
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
        >(cosine_similarity__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn cosine_similarity__with_callees_mocked(
    str1: &str,
    str2: &str,
    split_length: usize,
) -> Result<f32, Error> {
    if str1.is_empty() || str2.is_empty() {
        return Ok(0.0);
    }
    let splitted_str1: Vec<String> = if split_length == 0 {
        str1.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str1, split_length)?
    };
    let splitted_str2: Vec<String> = if split_length == 0 {
        str2.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        shingle_slice(str2, split_length)?
    };
    let rune_str1: Vec<Vec<char>> = splitted_str1
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let rune_str2: Vec<Vec<char>> = splitted_str2
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    let union_str: Vec<Vec<char>> = union(&splitted_str1, &splitted_str2)?;
    for word in &union_str {
        let fw = find(&rune_str1, &word)?;
        l1.push(if fw != -1 { 1 } else { 0 });
        let fw = find(&rune_str2, &word)?;
        l2.push(if fw != -1 { 1 } else { 0 });
    }
    let mut cosine_sim: f32 = 0.0;
    for i in 0..union_str.len() {
        cosine_sim += (l1[i] * l2[i]) as f32;
    }
    let l1_sum: f32 = sum(&l1) as f32;
    let l2_sum: f32 = sum(&l2) as f32;
    Ok(cosine_sim / (l1_sum.sqrt() * l2_sum.sqrt()))
}
#[cfg(test)]
mod go_edlib_cosine_similarity_harness {
    use super::*;
    #[test]
    fn cosine_similarity__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-hbollon-go-edlib.CosineSimilarity.json",
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
                        (cosine_similarity__with_callees_mocked(
                            &*input_state.0,
                            &*input_state.1,
                            input_state.2,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (cosine_similarity(
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
    fn cosine_similarity__signature_check() {
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
            "./exec-snapshots/github.com-hbollon-go-edlib.CosineSimilarity.json",
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
