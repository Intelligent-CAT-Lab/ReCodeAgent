use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::internal::utils::utils::max;
#[cfg(not(feature = "mock"))]
pub(crate) fn lcs_process(
    rune_str1: &[char],
    rune_str2: &[char],
) -> Result<Vec<Vec<i32>>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];
    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }
    Ok(lcs_matrix)
}
#[cfg(feature = "mock")]
pub(crate) fn lcs_process(
    rune_str1: &[char],
    rune_str2: &[char],
) -> Result<Vec<Vec<i32>>> {
    extern "C" {
        #[link_name = "go_edlib_lcs_process__ground_truth"]
        fn lcs_process__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a [char], &'b [char]);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<[char]>, Box<[char]>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(#[serde_as(as = "serde_with::DefaultOnNull")] Vec<Vec<i32>>);
    let input_state_in = InputStateIn(rune_str1, rune_str2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(lcs_process__foreign(ser(&params[0]), ser(&params[1])))
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
pub(crate) fn lcs_process__with_callees_mocked(
    rune_str1: &[char],
    rune_str2: &[char],
) -> Result<Vec<Vec<i32>>> {
    let mut lcs_matrix = vec![vec![0; rune_str2.len() + 1]; rune_str1.len() + 1];
    for i in 1..=rune_str1.len() {
        for j in 1..=rune_str2.len() {
            if rune_str1[i - 1] == rune_str2[j - 1] {
                lcs_matrix[i][j] = lcs_matrix[i - 1][j - 1] + 1;
            } else {
                lcs_matrix[i][j] = max(lcs_matrix[i][j - 1], lcs_matrix[i - 1][j]);
            }
        }
    }
    Ok(lcs_matrix)
}

use crate::internal::utils::utils::equal;
#[cfg(not(feature = "mock"))]
pub fn lcs(str1: &str, str2: &str) -> Result<i32> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();
    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Ok(0);
    } else if equal(&rune_str1, &rune_str2)? {
        return Ok(rune_str1.len() as i32);
    }
    let lcs_matrix = lcs_process(&rune_str1, &rune_str2)?;
    Ok(lcs_matrix[rune_str1.len()][rune_str2.len()])
}
#[cfg(feature = "mock")]
pub fn lcs(str1: &str, str2: &str) -> Result<i32> {
    extern "C" {
        #[link_name = "go_edlib_lcs__ground_truth"]
        fn lcs__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(str1, str2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(lcs__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn lcs__with_callees_mocked(str1: &str, str2: &str) -> Result<i32> {
    let rune_str1: Vec<char> = str1.chars().collect();
    let rune_str2: Vec<char> = str2.chars().collect();
    if rune_str1.is_empty() || rune_str2.is_empty() {
        return Ok(0);
    } else if equal(&rune_str1, &rune_str2)? {
        return Ok(rune_str1.len() as i32);
    }
    let lcs_matrix = lcs_process(&rune_str1, &rune_str2)?;
    Ok(lcs_matrix[rune_str1.len()][rune_str2.len()])
}

#[cfg(not(feature = "mock"))]
pub fn lcs_edit_distance(str1: &str, str2: &str) -> Result<i32> {
    if str1.is_empty() {
        return Ok(str2.len() as i32);
    } else if str2.is_empty() {
        return Ok(str1.len() as i32);
    } else if str1 == str2 {
        return Ok(0);
    }
    let lcs_len = lcs(str1, str2)?;
    let edit_distance = (str1.len() as i32 - lcs_len) + (str2.len() as i32 - lcs_len);
    Ok(edit_distance)
}
#[cfg(feature = "mock")]
pub fn lcs_edit_distance(str1: &str, str2: &str) -> Result<i32> {
    extern "C" {
        #[link_name = "go_edlib_lcs_edit_distance__ground_truth"]
        fn lcs_edit_distance__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(str1, str2);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(lcs_edit_distance__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn lcs_edit_distance__with_callees_mocked(str1: &str, str2: &str) -> Result<i32> {
    if str1.is_empty() {
        return Ok(str2.len() as i32);
    } else if str2.is_empty() {
        return Ok(str1.len() as i32);
    } else if str1 == str2 {
        return Ok(0);
    }
    let lcs_len = lcs(str1, str2)?;
    let edit_distance = (str1.len() as i32 - lcs_len) + (str2.len() as i32 - lcs_len);
    Ok(edit_distance)
}

