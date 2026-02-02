use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;


#[cfg(not(feature = "mock"))]
pub(crate) fn normalize(weight: f32, min: f32, max: f32) -> Result<f32, anyhow::Error> {
    if max <= min {
        return Err(
            anyhow::anyhow!(
                "Invalid range: max ({}) must be greater than min ({})", max, min
            ),
        );
    }
    let normalized_weight = (weight - min) / (max - min);
    Ok(normalized_weight)
}
#[cfg(feature = "mock")]
pub(crate) fn normalize(weight: f32, min: f32, max: f32) -> Result<f32, anyhow::Error> {
    extern "C" {
        #[link_name = "TextRank_normalize__ground_truth"]
        fn normalize__foreign(_: JSONObject, _: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    let input_state_in = InputStateIn(weight, min, max);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(normalize__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub(crate) fn normalize__with_callees_mocked(
    weight: f32,
    min: f32,
    max: f32,
) -> Result<f32, anyhow::Error> {
    if max <= min {
        return Err(
            anyhow::anyhow!(
                "Invalid range: max ({}) must be greater than min ({})", max, min
            ),
        );
    }
    let normalized_weight = (weight - min) / (max - min);
    Ok(normalized_weight)
}
#[cfg(test)]
mod TextRank_normalize_harness {
    use super::*;
    #[test]
    fn normalize__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.normalize.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
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
                        (normalize__with_callees_mocked(
                            input_state.0,
                            input_state.1,
                            input_state.2,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (normalize(input_state.0, input_state.1, input_state.2)).unwrap() }
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
    fn normalize__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.normalize.json",
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
use crate::__synthetic::__Synth5__weighting_hits;
use crate::algorithm::Algorithm;
use crate::rank::Word;
use crate::relation::Score;
use crate::relation::Relation;
use crate::__synthetic::__Synth6__weighting_relation;
use std::collections::HashMap;
use crate::rank::Rank;
#[cfg(not(feature = "mock"))]
pub(crate) fn update_ranks(ranks: &mut Rank, algorithm: &dyn Algorithm) -> Result<()> {
    mock::mock_body!(
        { for word in ranks.words.values_mut() { let weight = algorithm
        .weighting_hits(word.id, ranks) ?; word.weight = weight; ranks.max = ranks.max
        .max(word.weight); ranks.min = ranks.min.min(word.weight); } for word in ranks
        .words.values_mut() { word.weight = normalize(word.weight, ranks.min, ranks.max)
        ?; } for (x, x_map) in ranks.relation.node.iter_mut() { for (y, score) in x_map
        .iter_mut() { let weight = algorithm.weighting_relation(x, y, ranks) ?; score
        .weight = weight; ranks.relation.max = ranks.relation.max.max(weight); ranks
        .relation.min = ranks.relation.min.min(weight); } } for x_map in ranks.relation
        .node.values_mut() { for score in x_map.values_mut() { score.weight =
        normalize(score.weight, ranks.relation.min, ranks.relation.max) ?; } } Ok(()) }
    );
}
#[cfg(feature = "mock")]
pub(crate) fn update_ranks(ranks: &mut Rank, algorithm: &dyn Algorithm) -> Result<()> {
    extern "C" {
        #[link_name = "TextRank_update_ranks__ground_truth"]
        fn update_ranks__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a mut Rank, &'b dyn Algorithm);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, Box<dyn Algorithm>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(ranks, algorithm);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(update_ranks__foreign(ser(&params[0]), ser(&params[1])))
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
        *ranks = *input_state_mutated.0;
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub(crate) fn update_ranks__with_callees_mocked(
    ranks: &mut Rank,
    algorithm: &dyn Algorithm,
) -> Result<()> {
    mock::mock_body!(
        { for word in ranks.words.values_mut() { let weight = algorithm
        .weighting_hits(word.id, ranks) ?; word.weight = weight; ranks.max = ranks.max
        .max(word.weight); ranks.min = ranks.min.min(word.weight); } for word in ranks
        .words.values_mut() { word.weight = normalize(word.weight, ranks.min, ranks.max)
        ?; } for (x, x_map) in ranks.relation.node.iter_mut() { for (y, score) in x_map
        .iter_mut() { let weight = algorithm.weighting_relation(x, y, ranks) ?; score
        .weight = weight; ranks.relation.max = ranks.relation.max.max(weight); ranks
        .relation.min = ranks.relation.min.min(weight); } } for x_map in ranks.relation
        .node.values_mut() { for score in x_map.values_mut() { score.weight =
        normalize(score.weight, ranks.relation.min, ranks.relation.max) ?; } } Ok(()) }
    );
}
#[cfg(test)]
mod TextRank_update_ranks_harness {
    use super::*;
    #[test]
    fn update_ranks__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.updateRanks.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, Box<dyn Algorithm>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState;
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
                        (update_ranks__with_callees_mocked(
                            &mut *input_state.0,
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (update_ranks(&mut *input_state.0, &*input_state.1)).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState;
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
    fn update_ranks__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, Box<dyn Algorithm>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.updateRanks.json",
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
pub fn calculate(ranks: &mut Rank, algorithm: &dyn Algorithm) -> Result<()> {
    update_ranks(ranks, algorithm)
}
#[cfg(feature = "mock")]
pub fn calculate(ranks: &mut Rank, algorithm: &dyn Algorithm) -> Result<()> {
    extern "C" {
        #[link_name = "TextRank_calculate__ground_truth"]
        fn calculate__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a mut Rank, &'b dyn Algorithm);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<Rank>, Box<dyn Algorithm>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(ranks, algorithm);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(calculate__foreign(ser(&params[0]), ser(&params[1])))
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
        *ranks = *input_state_mutated.0;
        let output_state: OutputState = serde_json::from_value(
                foreign_execution.return_value,
            )
            .unwrap();
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn calculate__with_callees_mocked(
    ranks: &mut Rank,
    algorithm: &dyn Algorithm,
) -> Result<()> {
    update_ranks(ranks, algorithm)
}
#[cfg(test)]
mod TextRank_calculate_harness {
    use super::*;
    #[test]
    fn calculate__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.Calculate.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, Box<dyn Algorithm>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState;
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
                        (calculate__with_callees_mocked(
                            &mut *input_state.0,
                            &*input_state.1,
                        ))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (calculate(&mut *input_state.0, &*input_state.1)).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState;
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
    fn calculate__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<Rank>, Box<dyn Algorithm>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.Calculate.json",
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
