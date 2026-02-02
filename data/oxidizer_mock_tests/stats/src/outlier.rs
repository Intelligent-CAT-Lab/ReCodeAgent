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
#[cfg(test)]
mod stats_Outliers_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "stats_Outliers_roundtrip"]
        fn Outliers__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn Outliers__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!("(*{}).", "github.com-montanaflynn-stats.Outliers"),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!("({}).", "github.com-montanaflynn-stats.Outliers"),
                            )
                    {
                        let unittests_file: std::fs::File = std::fs::File::open(
                                entry.path(),
                            )
                            .unwrap();
                        let unittests_reader = std::io::BufReader::new(unittests_file);
                        let unittests: Vec<ExecutionData> = serde_json::from_reader(
                                unittests_reader,
                            )
                            .unwrap();
                        for unittest in unittests {
                            let obj = unittest.inputs[0].clone();
                            if obj == serde_json::Value::Null {
                                continue;
                            }
                            let obj_once = serde_json::to_value(
                                    serde_json::from_value::<Outliers>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<Outliers>(obj_once.clone())
                                        .unwrap(),
                                )
                                .unwrap();
                            assert_json_diff::assert_json_eq!(obj_once, obj_twice);
                        }
                    }
                }
            }
        }
    }
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
#[cfg(test)]
mod stats_quartile_outliers_harness {
    use super::*;
    #[test]
    fn quartile_outliers__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.QuartileOutliers.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Float64Data);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(Outliers);
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
                    { (quartile_outliers__with_callees_mocked(input_state.0)).unwrap() }
                    #[cfg(not(feature = "mock"))]
                    { (quartile_outliers(input_state.0)).unwrap() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn quartile_outliers__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Float64Data);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Outliers);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.QuartileOutliers.json",
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
