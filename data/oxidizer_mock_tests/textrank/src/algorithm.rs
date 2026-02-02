use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;


#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub struct AlgorithmChain {}
#[cfg(test)]
mod TextRank_AlgorithmChain_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "TextRank_AlgorithmChain_roundtrip"]
        fn AlgorithmChain__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn AlgorithmChain__weak__interoperation() {
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
                                "(*{}).", "github.com-DavidBelicza-TextRank.AlgorithmChain"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-DavidBelicza-TextRank.AlgorithmChain"
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
                                    serde_json::from_value::<AlgorithmChain>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<AlgorithmChain>(obj_once.clone())
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
use crate::rank::Rank;
#[typetag::serde(tag = "Type", content = "Value")]
pub trait Algorithm: crate::__synthetic::__Synth5__weighting_hits + crate::__synthetic::__Synth6__weighting_relation {}
use crate::__synthetic::__Synth5__weighting_hits;
use std::collections::HashMap;
use crate::rank::Word;
#[cfg(not(feature = "mock"))]
impl __Synth5__weighting_hits for AlgorithmChain {
    fn weighting_hits(&self, word_id: i32, rank: &Rank) -> f32 {
        let word = rank.words.get(&word_id).unwrap();
        let mut qty = 0;
        for (left_word_id, left_word_qty) in &word.connection_left {
            if let Some(left_word) = rank.words.get(left_word_id) {
                qty += left_word.qty * left_word_qty;
            }
        }
        for (right_word_id, right_word_qty) in &word.connection_right {
            if let Some(right_word) = rank.words.get(right_word_id) {
                qty += right_word.qty * right_word_qty;
            }
        }
        let weight = word.qty as f32 + qty as f32;
        weight
    }
}
#[cfg(feature = "mock")]
impl __Synth5__weighting_hits for AlgorithmChain {
    fn weighting_hits(&self, word_id: i32, rank: &Rank) -> f32 {
        extern "C" {
            #[link_name = "TextRank_algorithm_chain___weighting_hits__ground_truth"]
            fn AlgorithmChain_weighting_hits__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a AlgorithmChain, i32, &'b Rank);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<AlgorithmChain>, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let input_state_in = InputStateIn(self, word_id, rank);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                AlgorithmChain_weighting_hits__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 3usize);
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
impl AlgorithmChain {
    fn weighting_hits__with_callees_mocked(&self, word_id: i32, rank: &Rank) -> f32 {
        let word = rank.words.get(&word_id).unwrap();
        let mut qty = 0;
        for (left_word_id, left_word_qty) in &word.connection_left {
            if let Some(left_word) = rank.words.get(left_word_id) {
                qty += left_word.qty * left_word_qty;
            }
        }
        for (right_word_id, right_word_qty) in &word.connection_right {
            if let Some(right_word) = rank.words.get(right_word_id) {
                qty += right_word.qty * right_word_qty;
            }
        }
        let weight = word.qty as f32 + qty as f32;
        weight
    }
}
#[cfg(test)]
mod TextRank_algorithm_chain___weighting_hits_harness {
    use super::*;
    #[test]
    fn AlgorithmChain_weighting_hits__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmChain).WeightingHits.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmChain>, i32, Box<Rank>);
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
                        (&*input_state.0)
                            .weighting_hits__with_callees_mocked(
                                input_state.1,
                                &*input_state.2,
                            )
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).weighting_hits(input_state.1, &*input_state.2) }
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
    fn AlgorithmChain_weighting_hits__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmChain>, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmChain).WeightingHits.json",
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
use crate::__synthetic::__Synth6__weighting_relation;
use crate::relation::Relation;
use crate::relation::Score;
#[cfg(not(feature = "mock"))]
impl __Synth6__weighting_relation for AlgorithmChain {
    fn weighting_relation(&self, word1_id: i32, word2_id: i32, rank: &Rank) -> f32 {
        let relation_qty = rank
            .relation
            .node
            .get(&word1_id)
            .and_then(|word1_map| word1_map.get(&word2_id))
            .map(|score| score.qty)
            .unwrap_or(0);
        let word1_qty = rank.words.get(&word1_id).map(|word| word.qty).unwrap_or(0);
        let word2_qty = rank.words.get(&word2_id).map(|word| word.qty).unwrap_or(0);
        let q_diff = (f32::abs(word1_qty as f32 - word2_qty as f32)) / 100.0;
        let weight = relation_qty as f32 + q_diff;
        weight
    }
}
#[cfg(feature = "mock")]
impl __Synth6__weighting_relation for AlgorithmChain {
    fn weighting_relation(&self, word1_id: i32, word2_id: i32, rank: &Rank) -> f32 {
        extern "C" {
            #[link_name = "TextRank_algorithm_chain___weighting_relation__ground_truth"]
            fn AlgorithmChain_weighting_relation__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a AlgorithmChain, i32, i32, &'b Rank);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<AlgorithmChain>, i32, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let input_state_in = InputStateIn(self, word1_id, word2_id, rank);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                AlgorithmChain_weighting_relation__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                    ser(&params[3]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 4usize);
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
impl AlgorithmChain {
    fn weighting_relation__with_callees_mocked(
        &self,
        word1_id: i32,
        word2_id: i32,
        rank: &Rank,
    ) -> f32 {
        let relation_qty = rank
            .relation
            .node
            .get(&word1_id)
            .and_then(|word1_map| word1_map.get(&word2_id))
            .map(|score| score.qty)
            .unwrap_or(0);
        let word1_qty = rank.words.get(&word1_id).map(|word| word.qty).unwrap_or(0);
        let word2_qty = rank.words.get(&word2_id).map(|word| word.qty).unwrap_or(0);
        let q_diff = (f32::abs(word1_qty as f32 - word2_qty as f32)) / 100.0;
        let weight = relation_qty as f32 + q_diff;
        weight
    }
}
#[cfg(test)]
mod TextRank_algorithm_chain___weighting_relation_harness {
    use super::*;
    #[test]
    fn AlgorithmChain_weighting_relation__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmChain).WeightingRelation.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmChain>, i32, i32, Box<Rank>);
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
                        (&*input_state.0)
                            .weighting_relation__with_callees_mocked(
                                input_state.1,
                                input_state.2,
                                &*input_state.3,
                            )
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (&*input_state.0)
                            .weighting_relation(
                                input_state.1,
                                input_state.2,
                                &*input_state.3,
                            )
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
                        serde_json::to_value(& input_state.3).unwrap(),
                        serde_json::to_value(& input_state_mutated.3).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn AlgorithmChain_weighting_relation__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmChain>, i32, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmChain).WeightingRelation.json",
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

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub struct AlgorithmDefault {}
#[cfg(test)]
mod TextRank_AlgorithmDefault_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "TextRank_AlgorithmDefault_roundtrip"]
        fn AlgorithmDefault__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn AlgorithmDefault__weak__interoperation() {
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
                                "github.com-DavidBelicza-TextRank.AlgorithmDefault"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-DavidBelicza-TextRank.AlgorithmDefault"
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
                                    serde_json::from_value::<AlgorithmDefault>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<AlgorithmDefault>(obj_once.clone())
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

#[cfg(not(feature = "mock"))]
impl __Synth5__weighting_hits for AlgorithmDefault {
    fn weighting_hits(&self, word_id: i32, rank: &Rank) -> f32 {
        let weight = rank.words.get(&word_id).map_or(0, |word| word.qty);
        weight as f32
    }
}
#[cfg(feature = "mock")]
impl __Synth5__weighting_hits for AlgorithmDefault {
    fn weighting_hits(&self, word_id: i32, rank: &Rank) -> f32 {
        extern "C" {
            #[link_name = "TextRank_algorithm_default___weighting_hits__ground_truth"]
            fn AlgorithmDefault_weighting_hits__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a AlgorithmDefault, i32, &'b Rank);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<AlgorithmDefault>, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let input_state_in = InputStateIn(self, word_id, rank);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                AlgorithmDefault_weighting_hits__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 3usize);
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
impl AlgorithmDefault {
    fn weighting_hits__with_callees_mocked(&self, word_id: i32, rank: &Rank) -> f32 {
        let weight = rank.words.get(&word_id).map_or(0, |word| word.qty);
        weight as f32
    }
}
#[cfg(test)]
mod TextRank_algorithm_default___weighting_hits_harness {
    use super::*;
    #[test]
    fn AlgorithmDefault_weighting_hits__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmDefault).WeightingHits.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmDefault>, i32, Box<Rank>);
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
                        (&*input_state.0)
                            .weighting_hits__with_callees_mocked(
                                input_state.1,
                                &*input_state.2,
                            )
                    }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).weighting_hits(input_state.1, &*input_state.2) }
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
    fn AlgorithmDefault_weighting_hits__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmDefault>, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmDefault).WeightingHits.json",
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
impl __Synth6__weighting_relation for AlgorithmDefault {
    fn weighting_relation(&self, word1_id: i32, word2_id: i32, rank: &Rank) -> f32 {
        let relation_qty = rank
            .relation
            .node
            .get(&word1_id)
            .and_then(|word1_map| word1_map.get(&word2_id))
            .map(|score| score.qty)
            .unwrap_or(0) as f32;
        relation_qty
    }
}
#[cfg(feature = "mock")]
impl __Synth6__weighting_relation for AlgorithmDefault {
    fn weighting_relation(&self, word1_id: i32, word2_id: i32, rank: &Rank) -> f32 {
        extern "C" {
            #[link_name = "TextRank_algorithm_default___weighting_relation__ground_truth"]
            fn AlgorithmDefault_weighting_relation__foreign(
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a, 'b>(&'a AlgorithmDefault, i32, i32, &'b Rank);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<AlgorithmDefault>, i32, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let input_state_in = InputStateIn(self, word1_id, word2_id, rank);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(
                AlgorithmDefault_weighting_relation__foreign(
                    ser(&params[0]),
                    ser(&params[1]),
                    ser(&params[2]),
                    ser(&params[3]),
                ),
            )
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 4usize);
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
impl AlgorithmDefault {
    fn weighting_relation__with_callees_mocked(
        &self,
        word1_id: i32,
        word2_id: i32,
        rank: &Rank,
    ) -> f32 {
        let relation_qty = rank
            .relation
            .node
            .get(&word1_id)
            .and_then(|word1_map| word1_map.get(&word2_id))
            .map(|score| score.qty)
            .unwrap_or(0) as f32;
        relation_qty
    }
}
#[cfg(test)]
mod TextRank_algorithm_default___weighting_relation_harness {
    use super::*;
    #[test]
    fn AlgorithmDefault_weighting_relation__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmDefault).WeightingRelation.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmDefault>, i32, i32, Box<Rank>);
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
                        (&*input_state.0)
                            .weighting_relation__with_callees_mocked(
                                input_state.1,
                                input_state.2,
                                &*input_state.3,
                            )
                    }
                    #[cfg(not(feature = "mock"))]
                    {
                        (&*input_state.0)
                            .weighting_relation(
                                input_state.1,
                                input_state.2,
                                &*input_state.3,
                            )
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
                        serde_json::to_value(& input_state.3).unwrap(),
                        serde_json::to_value(& input_state_mutated.3).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn AlgorithmDefault_weighting_relation__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<AlgorithmDefault>, i32, i32, Box<Rank>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
            f32,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-DavidBelicza-TextRank.AlgorithmDefault).WeightingRelation.json",
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

#[typetag::serde(name = "AlgorithmChain")]
impl crate::algorithm::Algorithm for AlgorithmChain {}

#[typetag::serde(name = "AlgorithmDefault")]
impl crate::algorithm::Algorithm for AlgorithmDefault {}

#[cfg(not(feature = "mock"))]
pub fn new_algorithm_default() -> Result<AlgorithmDefault> {
    Ok(AlgorithmDefault::default())
}
#[cfg(feature = "mock")]
pub fn new_algorithm_default() -> Result<AlgorithmDefault> {
    extern "C" {
        #[link_name = "TextRank_new_algorithm_default__ground_truth"]
        fn new_algorithm_default__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(AlgorithmDefault);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe {
        de::<ForeignExecution>(new_algorithm_default__foreign())
    };
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
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn new_algorithm_default__with_callees_mocked() -> Result<AlgorithmDefault> {
    Ok(AlgorithmDefault::default())
}
#[cfg(test)]
mod TextRank_new_algorithm_default_harness {
    use super::*;
    #[test]
    fn new_algorithm_default__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-DavidBelicza-TextRank.NewAlgorithmDefault.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(AlgorithmDefault);
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
                    { (new_algorithm_default__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (new_algorithm_default()).unwrap() }
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
    fn new_algorithm_default__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(AlgorithmDefault);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-DavidBelicza-TextRank.NewAlgorithmDefault.json",
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
