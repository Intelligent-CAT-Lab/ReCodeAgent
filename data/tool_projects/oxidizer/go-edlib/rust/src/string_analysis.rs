use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[cfg(not(feature = "mock"))]
pub(crate) fn matching_index(
    str1: &str,
    str2: &str,
    distance: usize,
) -> Result<f32, anyhow::Error> {
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    let longer_len = chars1.len().max(chars2.len());
    let shorter_len = chars1.len().min(chars2.len());
    let matching_percentage = if longer_len >= shorter_len + distance {
        (shorter_len as f32) / (longer_len as f32)
    } else {
        ((longer_len - distance) as f32) / (longer_len as f32)
    };
    Ok(matching_percentage)
}
#[cfg(feature = "mock")]
pub(crate) fn matching_index(
    str1: &str,
    str2: &str,
    distance: usize,
) -> Result<f32, anyhow::Error> {
    extern "C" {
        #[link_name = "go_edlib_matching_index__ground_truth"]
        fn matching_index__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>, usize);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    let input_state_in = InputStateIn(str1, str2, distance);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(matching_index__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub(crate) fn matching_index__with_callees_mocked(
    str1: &str,
    str2: &str,
    distance: usize,
) -> Result<f32, anyhow::Error> {
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    let longer_len = chars1.len().max(chars2.len());
    let shorter_len = chars1.len().min(chars2.len());
    let matching_percentage = if longer_len >= shorter_len + distance {
        (shorter_len as f32) / (longer_len as f32)
    } else {
        ((longer_len - distance) as f32) / (longer_len as f32)
    };
    Ok(matching_percentage)
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Ord, Eq, Hash)]
#[derive(PartialOrd, PartialEq, Clone, derive_more::Add, derive_more::Sub)]
#[derive(derive_more::From, derive_more::Into)]
#[derive(Default)]
pub struct Algorithm(pub u8);

pub const COSINE: Algorithm = Algorithm(7);

pub const DAMERAU_LEVENSHTEIN: Algorithm = Algorithm(1);

pub const Hamming: Algorithm = Algorithm(4);

pub const JACCARD: Algorithm = Algorithm(8);

pub const Jaro: Algorithm = Algorithm(5);

pub const JARO_WINKLER: Algorithm = Algorithm(6);

pub const Lcs: Algorithm = Algorithm(3);

pub const LEVENSHTEIN: Algorithm = Algorithm(0);

pub const OSADamerauLevenshtein: Algorithm = Algorithm(2);

pub const QGRAM: Algorithm = Algorithm(10);

pub const SorensenDice: Algorithm = Algorithm(9);
use crate::jaro::jaro_winkler_similarity;
use crate::jaccard::jaccard_similarity;
use crate::levenshtein::damerau_levenshtein_distance;
use crate::cosine::cosine_similarity;
use crate::lcs::lcs_edit_distance;
use crate::sorensen_dice::sorensen_dice_coefficient;
use crate::levenshtein::osa_damerau_levenshtein_distance;
use crate::qgram::qgram_similarity;
use crate::hamming::hamming_distance;
use crate::levenshtein::levenshtein_distance;
use crate::jaro::jaro_similarity;
#[cfg(not(feature = "mock"))]
pub fn strings_similarity(str1: &str, str2: &str, algo: Algorithm) -> Result<f32> {
    match algo {
        LEVENSHTEIN => {
            Ok(matching_index(str1, str2, levenshtein_distance(str1, str2)? as usize)?)
        }
        DAMERAU_LEVENSHTEIN => {
            Ok(
                matching_index(
                    str1,
                    str2,
                    damerau_levenshtein_distance(str1, str2)? as usize,
                )?,
            )
        }
        OSADamerauLevenshtein => {
            Ok(
                matching_index(
                    str1,
                    str2,
                    osa_damerau_levenshtein_distance(str1, str2)? as usize,
                )?,
            )
        }
        Lcs => Ok(matching_index(str1, str2, lcs_edit_distance(str1, str2)? as usize)?),
        Hamming => {
            match hamming_distance(str1, str2) {
                Ok(distance) => matching_index(str1, str2, distance.into()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        Jaro => jaro_similarity(str1, str2).map_err(anyhow::Error::from),
        JARO_WINKLER => jaro_winkler_similarity(str1, str2).map_err(anyhow::Error::from),
        COSINE => cosine_similarity(str1, str2, 2).map_err(anyhow::Error::from),
        JACCARD => jaccard_similarity(str1, str2, 2).map_err(anyhow::Error::from),
        SorensenDice => {
            sorensen_dice_coefficient(str1, str2, 2).map_err(anyhow::Error::from)
        }
        QGRAM => Ok(qgram_similarity(str1, str2, 2)),
        _ => Err(anyhow!("Illegal argument for algorithm method")),
    }
}
#[cfg(feature = "mock")]
pub fn strings_similarity(str1: &str, str2: &str, algo: Algorithm) -> Result<f32> {
    extern "C" {
        #[link_name = "go_edlib_strings_similarity__ground_truth"]
        fn strings_similarity__foreign(
            _: JSONObject,
            _: JSONObject,
            _: JSONObject,
        ) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a str, &'b str, Algorithm);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>, Box<str>, Algorithm);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat32")]
        f32,
    );
    let input_state_in = InputStateIn(str1, str2, algo);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(strings_similarity__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
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
pub fn strings_similarity__with_callees_mocked(
    str1: &str,
    str2: &str,
    algo: Algorithm,
) -> Result<f32> {
    match algo {
        LEVENSHTEIN => {
            Ok(matching_index(str1, str2, levenshtein_distance(str1, str2)? as usize)?)
        }
        DAMERAU_LEVENSHTEIN => {
            Ok(
                matching_index(
                    str1,
                    str2,
                    damerau_levenshtein_distance(str1, str2)? as usize,
                )?,
            )
        }
        OSADamerauLevenshtein => {
            Ok(
                matching_index(
                    str1,
                    str2,
                    osa_damerau_levenshtein_distance(str1, str2)? as usize,
                )?,
            )
        }
        Lcs => Ok(matching_index(str1, str2, lcs_edit_distance(str1, str2)? as usize)?),
        Hamming => {
            match hamming_distance(str1, str2) {
                Ok(distance) => matching_index(str1, str2, distance.into()),
                Err(err) => return Err(anyhow!(err)),
            }
        }
        Jaro => jaro_similarity(str1, str2).map_err(anyhow::Error::from),
        JARO_WINKLER => jaro_winkler_similarity(str1, str2).map_err(anyhow::Error::from),
        COSINE => cosine_similarity(str1, str2, 2).map_err(anyhow::Error::from),
        JACCARD => jaccard_similarity(str1, str2, 2).map_err(anyhow::Error::from),
        SorensenDice => {
            sorensen_dice_coefficient(str1, str2, 2).map_err(anyhow::Error::from)
        }
        QGRAM => Ok(qgram_similarity(str1, str2, 2)),
        _ => Err(anyhow!("Illegal argument for algorithm method")),
    }
}

