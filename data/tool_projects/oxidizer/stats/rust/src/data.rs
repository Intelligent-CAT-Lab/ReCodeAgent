use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(derive_more::From, derive_more::Into)]
#[derive(Default)]
#[derive(Clone)]
pub struct Float64Data(
    #[serde_as(as = "Vec < crate :: interoperation_utils :: MyFloat64 >")]
    pub Vec<f64>,
);

#[cfg(not(feature = "mock"))]
impl Float64Data {
    pub fn get(&self, i: usize) -> f64 {
        self.0[i]
    }
}
#[cfg(feature = "mock")]
impl Float64Data {
    pub fn get(&self, i: usize) -> f64 {
        extern "C" {
            #[link_name = "stats_float64_data___get__ground_truth"]
            fn Float64Data_get__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a Float64Data, usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Float64Data>, usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self, i);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(Float64Data_get__foreign(ser(&params[0]), ser(&params[1])))
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
impl Float64Data {
    pub fn get__with_callees_mocked(&self, i: usize) -> f64 {
        self.0[i]
    }
}

#[cfg(not(feature = "mock"))]
impl Float64Data {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
#[cfg(feature = "mock")]
impl Float64Data {
    pub fn len(&self) -> usize {
        extern "C" {
            #[link_name = "stats_float64_data___len__ground_truth"]
            fn Float64Data_len__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a Float64Data);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Float64Data>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(usize);
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(Float64Data_len__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
impl Float64Data {
    pub fn len__with_callees_mocked(&self) -> usize {
        self.0.len()
    }
}

use crate::sum::sum;
#[cfg(not(feature = "mock"))]
impl Float64Data {
    pub fn sum(&self) -> Result<f64, Error> {
        sum(self.clone())
    }
}
#[cfg(feature = "mock")]
impl Float64Data {
    pub fn sum(&self) -> Result<f64, Error> {
        extern "C" {
            #[link_name = "stats_float64_data___sum__ground_truth"]
            fn Float64Data_sum__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a Float64Data);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<Float64Data>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(Float64Data_sum__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
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
impl Float64Data {
    pub fn sum__with_callees_mocked(&self) -> Result<f64, Error> {
        sum(self.clone())
    }
}

