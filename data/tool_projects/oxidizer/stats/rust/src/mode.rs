use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::util::sorted_copy_dif;
use crate::legacy::EMPTY_INPUT_ERR;
use std::collections::HashSet;
use crate::errors::StatsError;
use crate::data::Float64Data;
#[cfg(not(feature = "mock"))]
pub fn mode(input: Float64Data) -> Result<Vec<f64>, anyhow::Error> {
    let l = input.len();
    if l == 1 {
        return Ok(input.0);
    } else if l == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut c = sorted_copy_dif(input)?;
    let mut mode = Vec::with_capacity(5);
    let mut cnt = 1;
    let mut max_cnt = 1;
    for i in 1..l {
        if c.0[i] == c.0[i - 1] {
            cnt += 1;
        } else {
            if cnt == max_cnt && max_cnt != 1 {
                mode.push(c.0[i - 1]);
                cnt = 1;
            } else if cnt > max_cnt {
                mode.truncate(0);
                mode.push(c.0[i - 1]);
                max_cnt = cnt;
                cnt = 1;
            } else {
                cnt = 1;
            }
        }
    }
    if cnt == max_cnt {
        mode.push(c.0[l - 1]);
    } else if cnt > max_cnt {
        mode.truncate(0);
        mode.push(c.0[l - 1]);
        max_cnt = cnt;
    }
    if max_cnt == 1 || mode.len() * max_cnt == l && max_cnt != l {
        return Ok(Vec::new());
    }
    Ok(mode)
}
#[cfg(feature = "mock")]
pub fn mode(input: Float64Data) -> Result<Vec<f64>, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_mode__ground_truth"]
        fn mode__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "Vec < crate :: interoperation_utils :: MyFloat64 >")]
        Vec<f64>,
    );
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(mode__foreign(ser(&input_state_in)))
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
pub fn mode__with_callees_mocked(input: Float64Data) -> Result<Vec<f64>, anyhow::Error> {
    let l = input.len();
    if l == 1 {
        return Ok(input.0);
    } else if l == 0 {
        return Err(anyhow!(EMPTY_INPUT_ERR.err.clone()));
    }
    let mut c = sorted_copy_dif(input)?;
    let mut mode = Vec::with_capacity(5);
    let mut cnt = 1;
    let mut max_cnt = 1;
    for i in 1..l {
        if c.0[i] == c.0[i - 1] {
            cnt += 1;
        } else {
            if cnt == max_cnt && max_cnt != 1 {
                mode.push(c.0[i - 1]);
                cnt = 1;
            } else if cnt > max_cnt {
                mode.truncate(0);
                mode.push(c.0[i - 1]);
                max_cnt = cnt;
                cnt = 1;
            } else {
                cnt = 1;
            }
        }
    }
    if cnt == max_cnt {
        mode.push(c.0[l - 1]);
    } else if cnt > max_cnt {
        mode.truncate(0);
        mode.push(c.0[l - 1]);
        max_cnt = cnt;
    }
    if max_cnt == 1 || mode.len() * max_cnt == l && max_cnt != l {
        return Ok(Vec::new());
    }
    Ok(mode)
}

