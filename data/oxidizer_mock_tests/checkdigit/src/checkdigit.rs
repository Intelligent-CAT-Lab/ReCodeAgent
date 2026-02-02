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
#[cfg(test)]
mod checkdigit_is_not_number_harness {
    use super::*;
    #[test]
    fn is_not_number__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.isNotNumber.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(char);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(bool);
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
                    { is_not_number__with_callees_mocked(input_state.0) }
                    #[cfg(not(feature = "mock"))] { is_not_number(input_state.0) }
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
    fn is_not_number__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(char);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.isNotNumber.json",
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
#[cfg(test)]
mod checkdigit_new_damm_harness {
    use super::*;
    #[test]
    fn Damm_new_damm__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewDamm.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    { (Damm::new_damm__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (Damm::new_damm()).unwrap() }
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
    fn Damm_new_damm__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewDamm.json",
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
#[cfg(test)]
mod checkdigit_new_ean13_harness {
    use super::*;
    #[test]
    fn new_ean13__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewEAN13.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    #[cfg(feature = "mock")] { new_ean13__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { new_ean13() }
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
    fn new_ean13__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewEAN13.json",
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
#[cfg(test)]
mod checkdigit_new_ean8_harness {
    use super::*;
    #[test]
    fn new_ean8__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewEAN8.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    { (new_ean8__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (new_ean8()).unwrap() }
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
    fn new_ean8__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewEAN8.json",
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
#[cfg(test)]
mod checkdigit_new_isbn10_harness {
    use super::*;
    #[test]
    fn NewISBN10__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewISBN10.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    #[cfg(feature = "mock")] { NewISBN10__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { NewISBN10() }
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
    fn NewISBN10__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewISBN10.json",
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
#[cfg(test)]
mod checkdigit_new_isbn13_harness {
    use super::*;
    #[test]
    fn new_isbn13__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewISBN13.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    { (new_isbn13__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (new_isbn13()).unwrap() }
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
    fn new_isbn13__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewISBN13.json",
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
#[cfg(test)]
mod checkdigit_new_itf_harness {
    use super::*;
    #[test]
    fn new_itf__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewITF.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    { (new_itf__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (new_itf()).unwrap() }
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
    fn new_itf__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewITF.json",
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
#[cfg(test)]
mod checkdigit_new_jan13_harness {
    use super::*;
    #[test]
    fn new_jan13__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewJAN13.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    #[cfg(feature = "mock")] { new_jan13__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { new_jan13() }
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
    fn new_jan13__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewJAN13.json",
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
#[cfg(test)]
mod checkdigit_new_jan8_harness {
    use super::*;
    #[test]
    fn gtin_new_jan8__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewJAN8.json",
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
        struct OutputState(gtin);
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
                    #[cfg(feature = "mock")] { gtin::new_jan8__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { gtin::new_jan8() }
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
    fn gtin_new_jan8__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(gtin);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewJAN8.json",
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
#[cfg(test)]
mod checkdigit_new_luhn_harness {
    use super::*;
    #[test]
    fn new_luhn__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewLuhn.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    #[cfg(feature = "mock")] { new_luhn__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { new_luhn() }
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
    fn new_luhn__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewLuhn.json",
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
#[cfg(test)]
mod checkdigit_new_sscc_harness {
    use super::*;
    #[test]
    fn new_sscc__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewSSCC.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    #[cfg(feature = "mock")] { new_sscc__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { new_sscc() }
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
    fn new_sscc__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewSSCC.json",
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
#[cfg(test)]
mod checkdigit_new_upc_harness {
    use super::*;
    #[test]
    fn new_upc__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewUPC.json",
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
        struct OutputState(Box<dyn Provider>);
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
                    #[cfg(feature = "mock")] { new_upc__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { new_upc() }
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
    fn new_upc__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewUPC.json",
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
#[cfg(test)]
mod checkdigit_new_verhoeff_harness {
    use super::*;
    #[test]
    fn Verhoeff_new_verhoeff__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.NewVerhoeff.json",
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
        struct OutputState(Verhoeff);
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
                    { Verhoeff::new_verhoeff__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { Verhoeff::new_verhoeff() }
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
    fn Verhoeff_new_verhoeff__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Verhoeff);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.NewVerhoeff.json",
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
