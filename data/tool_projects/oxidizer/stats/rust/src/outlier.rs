use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::data::Float64Data;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Outliers {
    #[serde(rename = "Mild")]
    pub mild: Float64Data,
    #[serde(rename = "Extreme")]
    pub extreme: Float64Data,
}

use crate::legacy::EMPTY_INPUT_ERR;
use crate::util::sorted_copy;
use crate::errors::StatsError;
use crate::quartile::Quartiles;
use crate::quartile::quartile;
use crate::quartile::inter_quartile_range;
#[cfg(not(feature = "mock"))]
pub fn quartile_outliers(input: Float64Data) -> Result<Outliers, anyhow::Error> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let copy = sorted_copy(input);
    let qs = quartile(copy.clone())?;
    let iqr = inter_quartile_range(copy.clone())?;
    let lif = qs.q1 - 1.5 * iqr;
    let uif = qs.q3 + 1.5 * iqr;
    let lof = qs.q1 - 3.0 * iqr;
    let uof = qs.q3 + 3.0 * iqr;
    let mut mild = Float64Data::default();
    let mut extreme = Float64Data::default();
    for v in copy.0 {
        if v < lof || v > uof {
            extreme.0.push(v);
        } else if v < lif || v > uif {
            mild.0.push(v);
        }
    }
    Ok(Outliers { mild, extreme })
}
#[cfg(feature = "mock")]
pub fn quartile_outliers(input: Float64Data) -> Result<Outliers, anyhow::Error> {
    extern "C" {
        #[link_name = "stats_quartile_outliers__ground_truth"]
        fn quartile_outliers__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Float64Data);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Outliers);
    let input_state_in = InputStateIn(input);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(quartile_outliers__foreign(ser(&input_state_in)))
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
pub fn quartile_outliers__with_callees_mocked(
    input: Float64Data,
) -> Result<Outliers, anyhow::Error> {
    if input.len() == 0 {
        return Err(EMPTY_INPUT_ERR.clone().into());
    }
    let copy = sorted_copy(input);
    let qs = quartile(copy.clone())?;
    let iqr = inter_quartile_range(copy.clone())?;
    let lif = qs.q1 - 1.5 * iqr;
    let uif = qs.q3 + 1.5 * iqr;
    let lof = qs.q1 - 3.0 * iqr;
    let uof = qs.q3 + 3.0 * iqr;
    let mut mild = Float64Data::default();
    let mut extreme = Float64Data::default();
    for v in copy.0 {
        if v < lof || v > uof {
            extreme.0.push(v);
        } else if v < lif || v > uif {
            mild.0.push(v);
        }
    }
    Ok(Outliers { mild, extreme })
}

