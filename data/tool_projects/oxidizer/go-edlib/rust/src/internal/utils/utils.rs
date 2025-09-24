use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[cfg(not(feature = "mock"))]
pub fn equal(a: &[char], b: &[char]) -> Result<bool> {
    if a.len() != b.len() {
        return Ok(false);
    }
    for (i, v) in a.iter().enumerate() {
        if v != &b[i] {
            return Ok(false);
        }
    }
    Ok(true)
}
#[cfg(feature = "mock")]
pub fn equal(a: &[char], b: &[char]) -> Result<bool> {
    extern "C" {
        #[link_name = "utils_equal__ground_truth"]
        fn equal__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a [char], &'b [char]);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<[char]>, Box<[char]>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(bool);
    let input_state_in = InputStateIn(a, b);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(equal__foreign(ser(&params[0]), ser(&params[1])))
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
pub fn equal__with_callees_mocked(a: &[char], b: &[char]) -> Result<bool> {
    if a.len() != b.len() {
        return Ok(false);
    }
    for (i, v) in a.iter().enumerate() {
        if v != &b[i] {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(not(feature = "mock"))]
pub fn min(a: i32, b: i32) -> i32 {
    if b < a { b } else { a }
}
#[cfg(feature = "mock")]
pub fn min(a: i32, b: i32) -> i32 {
    extern "C" {
        #[link_name = "utils_min__ground_truth"]
        fn min__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(a, b);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(min__foreign(ser(&params[0]), ser(&params[1])))
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
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub fn min__with_callees_mocked(a: i32, b: i32) -> i32 {
    if b < a { b } else { a }
}

#[cfg(not(feature = "mock"))]
pub fn max(a: i32, b: i32) -> i32 {
    if b > a { b } else { a }
}
#[cfg(feature = "mock")]
pub fn max(a: i32, b: i32) -> i32 {
    extern "C" {
        #[link_name = "utils_max__ground_truth"]
        fn max__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(i32, i32);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i32);
    let input_state_in = InputStateIn(a, b);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(max__foreign(ser(&params[0]), ser(&params[1])))
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
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub fn max__with_callees_mocked(a: i32, b: i32) -> i32 {
    if b > a { b } else { a }
}

