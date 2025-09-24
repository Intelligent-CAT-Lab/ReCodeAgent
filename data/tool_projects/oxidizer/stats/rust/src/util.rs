use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::data::Float64Data;
use std::clone::Clone;
#[cfg(not(feature = "mock"))]
pub(crate) fn copyslice(input: Float64Data) -> Float64Data {
    let mut s: Float64Data = Float64Data(Vec::with_capacity(input.len()));
    s.0.clone_from(&input.0);
    s
}
#[cfg(feature = "mock")]
pub(crate) fn copyslice(input: Float64Data) -> Float64Data {
    extern "C" {
        #[link_name = "stats_copyslice__ground_truth"]
        fn copyslice__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Float64Data);
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(copyslice__foreign(ser(&input_state_in)))
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
pub(crate) fn copyslice__with_callees_mocked(input: Float64Data) -> Float64Data {
    let mut s: Float64Data = Float64Data(Vec::with_capacity(input.len()));
    s.0.clone_from(&input.0);
    s
}

use std::vec::Vec;
#[cfg(not(feature = "mock"))]
pub(crate) fn sorted_copy(input: Float64Data) -> Float64Data {
    let mut copy = copyslice(input);
    copy.0.sort_by(|a, b| a.partial_cmp(b).unwrap());
    copy
}
#[cfg(feature = "mock")]
pub(crate) fn sorted_copy(input: Float64Data) -> Float64Data {
    extern "C" {
        #[link_name = "stats_sorted_copy__ground_truth"]
        fn sorted_copy__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Float64Data);
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(sorted_copy__foreign(ser(&input_state_in)))
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
pub(crate) fn sorted_copy__with_callees_mocked(input: Float64Data) -> Float64Data {
    let mut copy = copyslice(input);
    copy.0.sort_by(|a, b| a.partial_cmp(b).unwrap());
    copy
}

#[cfg(not(feature = "mock"))]
pub(crate) fn sorted_copy_dif(input: Float64Data) -> Result<Float64Data, anyhow::Error> {
    if input.0.windows(2).all(|w| w[0] <= w[1]) {
        Ok(input)
    } else {
        let mut copy = copyslice(input);
        copy.0.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(copy)
    }
}
#[cfg(feature = "mock")]
pub(crate) fn sorted_copy_dif(input: Float64Data) -> Result<Float64Data, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_sorted_copy_dif__ground_truth"]
        fn sorted_copy_dif__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Float64Data);
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(sorted_copy_dif__foreign(ser(&input_state_in)))
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
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub(crate) fn sorted_copy_dif__with_callees_mocked(
    input: Float64Data,
) -> Result<Float64Data, anyhow::Error> {
    if input.0.windows(2).all(|w| w[0] <= w[1]) {
        Ok(input)
    } else {
        let mut copy = copyslice(input);
        copy.0.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(copy)
    }
}

use std::time::SystemTime;
use std::time::UNIX_EPOCH;
#[cfg(not(feature = "mock"))]
pub(crate) fn unixnano() -> i64 {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since Unix epoch");
    duration_since_epoch.as_nanos() as i64
}
#[cfg(feature = "mock")]
pub(crate) fn unixnano() -> i64 {
    extern "C" {
        #[link_name = "stats_unixnano__ground_truth"]
        fn unixnano__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(i64);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(unixnano__foreign()) };
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
        return output;
    } else {
        panic!("execution failure");
    }
}
#[cfg(feature = "mock")]
pub(crate) fn unixnano__with_callees_mocked() -> i64 {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since Unix epoch");
    duration_since_epoch.as_nanos() as i64
}

