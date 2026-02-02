use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;


#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct DescriptionPercentile {
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Percentile")]
    pub percentile: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Value")]
    pub value: f64,
}
#[cfg(test)]
mod stats_descriptionPercentile_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "stats_descriptionPercentile_roundtrip"]
        fn DescriptionPercentile__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn DescriptionPercentile__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!(
                                "(*{}).",
                                "github.com-montanaflynn-stats.descriptionPercentile"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).",
                                    "github.com-montanaflynn-stats.descriptionPercentile"
                                ),
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
                                    serde_json::from_value::<DescriptionPercentile>(obj)
                                        .unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<
                                        DescriptionPercentile,
                                    >(obj_once.clone())
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

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Description {
    #[serde(rename = "Count")]
    pub count: i32,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Mean")]
    pub mean: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Std")]
    pub std: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Max")]
    pub max: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "DescriptionPercentiles")]
    pub description_percentiles: Vec<DescriptionPercentile>,
    #[serde(rename = "AllowedNaN")]
    pub allowed_nan: bool,
}
#[cfg(test)]
mod stats_Description_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "stats_Description_roundtrip"]
        fn Description__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn Description__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!(
                                "(*{}).", "github.com-montanaflynn-stats.Description"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-montanaflynn-stats.Description"
                                ),
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
                                    serde_json::from_value::<Description>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<Description>(obj_once.clone())
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
use crate::errors::ERR_EMPTY_INPUT;
use crate::deviation::standard_deviation;
use crate::mean::mean;
use crate::percentile::percentile_nearest_rank;
use crate::min::min;
use crate::errors::StatsError;
use crate::max::max;
use crate::data::Float64Data;
use crate::percentile::percentile;
pub fn describe_percentile_func(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
    percentile_func: fn(Float64Data, f64) -> Result<f64, anyhow::Error>,
) -> Result<Description, anyhow::Error> {
    let mut description = Description {
        allowed_nan: allow_nan,
        count: input.len() as i32,
        ..Default::default()
    };
    if description.count == 0 && !allow_nan {
        return Err(ERR_EMPTY_INPUT.clone().into());
    }
    description.std = standard_deviation(input.clone())?;
    description.max = max(input.clone())?;
    description.min = min(input.clone())?;
    description.mean = mean(input.clone())?;
    if let Some(percentiles) = percentiles {
        for &percentile in percentiles {
            if let Ok(value) = percentile_func(input.clone(), percentile) {
                description
                    .description_percentiles
                    .push(DescriptionPercentile {
                        percentile,
                        value,
                    });
            } else if allow_nan {}
        }
    }
    Ok(description)
}

#[cfg(not(feature = "mock"))]
pub fn describe(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
) -> Result<Description> {
    let percentile_func = |data: Float64Data, percentile: f64| -> Result<f64> {
        Ok(percentile)
    };
    describe_percentile_func(input, allow_nan, percentiles, percentile_func)
}
#[cfg(feature = "mock")]
pub fn describe(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
) -> Result<Description> {
    extern "C" {
        #[link_name = "stats_describe__ground_truth"]
        fn describe__foreign(_: JSONObject, _: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(
        Float64Data,
        bool,
        #[serde_as(as = "Option < & 'a [crate :: interoperation_utils :: MyFloat64] >")]
        Option<&'a [f64]>,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        Float64Data,
        bool,
        #[serde_as(
            as = "Option < Box < [crate :: interoperation_utils :: MyFloat64] > >"
        )]
        Option<Box<[f64]>>,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Description);
    let input_state_in = InputStateIn(input, allow_nan, percentiles);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(describe__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn describe__with_callees_mocked(
    input: Float64Data,
    allow_nan: bool,
    percentiles: Option<&[f64]>,
) -> Result<Description> {
    let percentile_func = |data: Float64Data, percentile: f64| -> Result<f64> {
        Ok(percentile)
    };
    describe_percentile_func(input, allow_nan, percentiles, percentile_func)
}
#[cfg(test)]
mod stats_describe_harness {
    use super::*;
    #[test]
    fn describe__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.Describe.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Float64Data,
            bool,
            #[serde_as(
                as = "Option < Box < [crate :: interoperation_utils :: MyFloat64] > >"
            )]
            Option<Box<[f64]>>,
        );
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(Description);
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
                        (describe__with_callees_mocked(
                            input_state.0,
                            input_state.1,
                            input_state.2.as_deref(),
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (describe(
                            input_state.0,
                            input_state.1,
                            input_state.2.as_deref(),
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
                        serde_json::to_value(& input_state.2).unwrap(),
                        serde_json::to_value(& input_state_mutated.2).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn describe__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Float64Data,
            bool,
            #[serde_as(
                as = "Option < Box < [crate :: interoperation_utils :: MyFloat64] > >"
            )]
            Option<Box<[f64]>>,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Description);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.Describe.json",
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
