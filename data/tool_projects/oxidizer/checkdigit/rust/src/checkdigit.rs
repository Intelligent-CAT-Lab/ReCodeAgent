use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[cfg(not(feature = "mock"))]
pub(crate) fn is_not_number(n: char) -> bool {
    n < '0' || n > '9'
}
#[cfg(feature = "mock")]
pub(crate) fn is_not_number(n: char) -> bool {
    extern "C" {
        #[link_name = "checkdigit_is_not_number__ground_truth"]
        fn is_not_number__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(char);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(char);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(bool);
    let input_state_in = InputStateIn(n);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(is_not_number__foreign(ser(&input_state_in)))
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
pub(crate) fn is_not_number__with_callees_mocked(n: char) -> bool {
    n < '0' || n > '9'
}

pub static ERR_INVALID_ARGUMENT: &'static str = "checkdigit: invalid argument";

#[typetag::serde(tag = "Type", content = "Value")]
pub trait Generator: crate::__synthetic::__Synth0__generate {}

#[typetag::serde(tag = "Type", content = "Value")]
pub trait Provider: crate::__synthetic::__Synth1__verify + crate::__synthetic::__Synth0__generate {}

#[typetag::serde(tag = "Type", content = "Value")]
pub trait Verifier: crate::__synthetic::__Synth1__verify {}
use crate::damm::Damm;
#[cfg(not(feature = "mock"))]
impl Damm {
    pub fn new_damm() -> Result<Box<dyn Provider>> {
        let matrix = vec![
            vec![0, 3, 1, 7, 5, 9, 8, 6, 4, 2], vec![7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
            vec![4, 2, 0, 6, 8, 7, 1, 3, 5, 9], vec![1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
            vec![6, 1, 2, 3, 0, 4, 5, 9, 7, 8], vec![3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
            vec![5, 8, 6, 9, 7, 2, 0, 1, 3, 4], vec![8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
            vec![9, 4, 3, 8, 6, 1, 7, 2, 0, 5], vec![2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
        ];
        let damm = Damm { matrix };
        Ok(Box::new(damm) as Box<dyn Provider>)
    }
}
#[cfg(feature = "mock")]
impl Damm {
    pub fn new_damm() -> Result<Box<dyn Provider>> {
        extern "C" {
            #[link_name = "checkdigit_new_damm__ground_truth"]
            fn Damm_new_damm__foreign() -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let input_state_in = InputStateIn();
        let foreign_execution = unsafe {
            de::<ForeignExecution>(Damm_new_damm__foreign())
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 0usize);
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
impl Damm {
    pub fn new_damm__with_callees_mocked() -> Result<Box<dyn Provider>> {
        let matrix = vec![
            vec![0, 3, 1, 7, 5, 9, 8, 6, 4, 2], vec![7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
            vec![4, 2, 0, 6, 8, 7, 1, 3, 5, 9], vec![1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
            vec![6, 1, 2, 3, 0, 4, 5, 9, 7, 8], vec![3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
            vec![5, 8, 6, 9, 7, 2, 0, 1, 3, 4], vec![8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
            vec![9, 4, 3, 8, 6, 1, 7, 2, 0, 5], vec![2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
        ];
        let damm = Damm { matrix };
        Ok(Box::new(damm) as Box<dyn Provider>)
    }
}

use crate::gtin::gtin;
#[cfg(not(feature = "mock"))]
pub fn new_ean13() -> Box<dyn Provider> {
    Box::new(gtin { digit: 13, pos_corr: true })
}
#[cfg(feature = "mock")]
pub fn new_ean13() -> Box<dyn Provider> {
    extern "C" {
        #[link_name = "checkdigit_new_ean13__ground_truth"]
        fn new_ean13__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_ean13__foreign()) };
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
pub fn new_ean13__with_callees_mocked() -> Box<dyn Provider> {
    Box::new(gtin { digit: 13, pos_corr: true })
}

#[cfg(not(feature = "mock"))]
pub fn new_ean8() -> Result<Box<dyn Provider>, anyhow::Error> {
    Ok(Box::new(gtin { digit: 8, pos_corr: true }))
}
#[cfg(feature = "mock")]
pub fn new_ean8() -> Result<Box<dyn Provider>, anyhow::Error> {
    extern "C" {
        #[link_name = "checkdigit_new_ean8__ground_truth"]
        fn new_ean8__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_ean8__foreign()) };
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
pub fn new_ean8__with_callees_mocked() -> Result<Box<dyn Provider>, anyhow::Error> {
    Ok(Box::new(gtin { digit: 8, pos_corr: true }))
}

use crate::isbn::isbn10;
#[cfg(not(feature = "mock"))]
pub fn NewISBN10() -> Box<dyn Provider> {
    Box::new(isbn10::default())
}
#[cfg(feature = "mock")]
pub fn NewISBN10() -> Box<dyn Provider> {
    extern "C" {
        #[link_name = "checkdigit_new_isbn10__ground_truth"]
        fn NewISBN10__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(NewISBN10__foreign()) };
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
pub fn NewISBN10__with_callees_mocked() -> Box<dyn Provider> {
    Box::new(isbn10::default())
}

use crate::isbn::Isbn13;
#[cfg(not(feature = "mock"))]
pub fn new_isbn13() -> Result<Box<dyn Provider>, anyhow::Error> {
    Ok(Box::new(Isbn13::default()))
}
#[cfg(feature = "mock")]
pub fn new_isbn13() -> Result<Box<dyn Provider>, anyhow::Error> {
    extern "C" {
        #[link_name = "checkdigit_new_isbn13__ground_truth"]
        fn new_isbn13__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_isbn13__foreign()) };
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
pub fn new_isbn13__with_callees_mocked() -> Result<Box<dyn Provider>, anyhow::Error> {
    Ok(Box::new(Isbn13::default()))
}

#[cfg(not(feature = "mock"))]
pub fn new_itf() -> Result<Box<dyn Provider>> {
    Ok(Box::new(gtin { digit: 14, pos_corr: false }))
}
#[cfg(feature = "mock")]
pub fn new_itf() -> Result<Box<dyn Provider>> {
    extern "C" {
        #[link_name = "checkdigit_new_itf__ground_truth"]
        fn new_itf__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_itf__foreign()) };
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
pub fn new_itf__with_callees_mocked() -> Result<Box<dyn Provider>> {
    Ok(Box::new(gtin { digit: 14, pos_corr: false }))
}

#[cfg(not(feature = "mock"))]
pub fn new_jan13() -> Box<dyn Provider> {
    Box::new(gtin { digit: 13, pos_corr: true })
}
#[cfg(feature = "mock")]
pub fn new_jan13() -> Box<dyn Provider> {
    extern "C" {
        #[link_name = "checkdigit_new_jan13__ground_truth"]
        fn new_jan13__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_jan13__foreign()) };
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
pub fn new_jan13__with_callees_mocked() -> Box<dyn Provider> {
    Box::new(gtin { digit: 13, pos_corr: true })
}

#[cfg(not(feature = "mock"))]
impl gtin {
    pub fn new_jan8() -> gtin {
        gtin { digit: 8, pos_corr: true }
    }
}
#[cfg(feature = "mock")]
impl gtin {
    pub fn new_jan8() -> gtin {
        extern "C" {
            #[link_name = "checkdigit_new_jan8__ground_truth"]
            fn gtin_new_jan8__foreign() -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(gtin);
        let input_state_in = InputStateIn();
        let foreign_execution = unsafe {
            de::<ForeignExecution>(gtin_new_jan8__foreign())
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 0usize);
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
impl gtin {
    pub fn new_jan8__with_callees_mocked() -> gtin {
        gtin { digit: 8, pos_corr: true }
    }
}

use crate::luhn::Luhn;
use std::boxed::Box;
#[cfg(not(feature = "mock"))]
pub fn new_luhn() -> Box<dyn Provider> {
    Box::new(Luhn {})
}
#[cfg(feature = "mock")]
pub fn new_luhn() -> Box<dyn Provider> {
    extern "C" {
        #[link_name = "checkdigit_new_luhn__ground_truth"]
        fn new_luhn__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_luhn__foreign()) };
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
pub fn new_luhn__with_callees_mocked() -> Box<dyn Provider> {
    Box::new(Luhn {})
}

#[cfg(not(feature = "mock"))]
pub fn new_sscc() -> Box<dyn Provider> {
    Box::new(gtin { digit: 18, pos_corr: false })
}
#[cfg(feature = "mock")]
pub fn new_sscc() -> Box<dyn Provider> {
    extern "C" {
        #[link_name = "checkdigit_new_sscc__ground_truth"]
        fn new_sscc__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_sscc__foreign()) };
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
pub fn new_sscc__with_callees_mocked() -> Box<dyn Provider> {
    Box::new(gtin { digit: 18, pos_corr: false })
}

#[cfg(not(feature = "mock"))]
pub fn new_upc() -> Box<dyn Provider> {
    Box::new(gtin { digit: 12, pos_corr: true })
}
#[cfg(feature = "mock")]
pub fn new_upc() -> Box<dyn Provider> {
    extern "C" {
        #[link_name = "checkdigit_new_upc__ground_truth"]
        fn new_upc__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(new_upc__foreign()) };
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
pub fn new_upc__with_callees_mocked() -> Box<dyn Provider> {
    Box::new(gtin { digit: 12, pos_corr: true })
}

use crate::verhoeff::Verhoeff;
#[cfg(not(feature = "mock"))]
impl Verhoeff {
    pub fn new_verhoeff() -> Verhoeff {
        Verhoeff {
            multiplication: vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
                vec![2, 3, 4, 0, 1, 7, 8, 9, 5, 6], vec![3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
                vec![4, 0, 1, 2, 3, 9, 5, 6, 7, 8], vec![5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
                vec![6, 5, 9, 8, 7, 1, 0, 4, 3, 2], vec![7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
                vec![8, 7, 6, 5, 9, 3, 2, 1, 0, 4], vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            ],
            permutation: vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
                vec![5, 8, 0, 3, 7, 9, 6, 1, 4, 2], vec![8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
                vec![9, 4, 5, 3, 1, 2, 6, 8, 7, 0], vec![4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
                vec![2, 7, 9, 3, 8, 0, 6, 4, 1, 5], vec![7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
            ],
            inverse: vec![0, 4, 3, 2, 1, 5, 6, 7, 8, 9],
        }
    }
}
#[cfg(feature = "mock")]
impl Verhoeff {
    pub fn new_verhoeff() -> Verhoeff {
        extern "C" {
            #[link_name = "checkdigit_new_verhoeff__ground_truth"]
            fn Verhoeff_new_verhoeff__foreign() -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Verhoeff);
        let input_state_in = InputStateIn();
        let foreign_execution = unsafe {
            de::<ForeignExecution>(Verhoeff_new_verhoeff__foreign())
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 0usize);
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
impl Verhoeff {
    pub fn new_verhoeff__with_callees_mocked() -> Verhoeff {
        Verhoeff {
            multiplication: vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
                vec![2, 3, 4, 0, 1, 7, 8, 9, 5, 6], vec![3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
                vec![4, 0, 1, 2, 3, 9, 5, 6, 7, 8], vec![5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
                vec![6, 5, 9, 8, 7, 1, 0, 4, 3, 2], vec![7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
                vec![8, 7, 6, 5, 9, 3, 2, 1, 0, 4], vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            ],
            permutation: vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
                vec![5, 8, 0, 3, 7, 9, 6, 1, 4, 2], vec![8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
                vec![9, 4, 5, 3, 1, 2, 6, 8, 7, 0], vec![4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
                vec![2, 7, 9, 3, 8, 0, 6, 4, 1, 5], vec![7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
            ],
            inverse: vec![0, 4, 3, 2, 1, 5, 6, 7, 8, 9],
        }
    }
}

