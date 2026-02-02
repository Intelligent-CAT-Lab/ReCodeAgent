use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::__synthetic::__Synth0__generate;
use crate::checkdigit::Provider;
use std::io::Write;
use std::io::stderr;
#[cfg(not(feature = "mock"))]
pub(crate) fn generate(g: &dyn Provider, seed: &str) -> Result<()> {
    let cd = match g.generate(seed) {
        Ok(cd) => cd,
        Err(err) => return Err(anyhow!("failed to generate with seed, message: {}", err)),
    };
    if cd > 9 {
        writeln!(stderr(), "X")?;
    } else {
        writeln!(stderr(), "{}", cd)?;
    }
    Ok(())
}
#[cfg(feature = "mock")]
pub(crate) fn generate(g: &dyn Provider, seed: &str) -> Result<()> {
    extern "C" {
        #[link_name = "checkdigit_generate__ground_truth"]
        fn generate__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a dyn Provider, &'b str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<dyn Provider>, Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn(g, seed);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(generate__foreign(ser(&params[0]), ser(&params[1])))
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
pub(crate) fn generate__with_callees_mocked(g: &dyn Provider, seed: &str) -> Result<()> {
    let cd = match g.generate(seed) {
        Ok(cd) => cd,
        Err(err) => return Err(anyhow!("failed to generate with seed, message: {}", err)),
    };
    if cd > 9 {
        writeln!(stderr(), "X")?;
    } else {
        writeln!(stderr(), "{}", cd)?;
    }
    Ok(())
}
#[cfg(test)]
mod checkdigit_generate_harness {
    use super::*;
    #[test]
    fn generate__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.generate.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<dyn Provider>, Box<str>);
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
                        (generate__with_callees_mocked(&*input_state.0, &*input_state.1))
                            .unwrap()
                    }
                    #[cfg(not(feature = "mock"))]
                    { (generate(&*input_state.0, &*input_state.1)).unwrap() }
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
    fn generate__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<dyn Provider>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.generate.json",
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
use crate::checkdigit::new_isbn13;
use crate::checkdigit::new_upc;
use crate::checkdigit::new_ean8;
use crate::checkdigit::new_itf;
use crate::checkdigit::new_luhn;
use crate::checkdigit::new_jan13;
use crate::checkdigit::new_sscc;
use crate::checkdigit::new_ean13;
use crate::checkdigit::NewISBN10;
#[cfg(not(feature = "mock"))]
pub(crate) fn take_provider(name: &str) -> Result<Box<dyn Provider>, Error> {
    mock::mock_body!(
        { let sanitized_name = name.to_lowercase().replace("-", ""); match sanitized_name
        .as_str() { "luhn" => Ok(new_luhn()), "verhoeff" =>
        Ok(Box::new(Verhoeff::new_verhoeff())), "damm" => Damm::new_damm().map(| p |
        Box::new(p) as Box < dyn Provider >), "isbn10" => Ok(NewISBN10()), "isbn13" |
        "isbn" => new_isbn13().map(| p | Box::new(p) as Box < dyn Provider >), "ean8" =>
        new_ean8().map(| p | Box::new(p) as Box < dyn Provider >), "ean13" | "ean" =>
        Ok(new_ean13()), "jan8" => Ok(Box::new(gtin::new_jan8())), "jan13" | "jan" =>
        Ok(new_jan13()), "itf" => new_itf().map(| p | Box::new(p) as Box < dyn Provider
        >), "upc" => Ok(new_upc()), "sscc" => Ok(new_sscc()), _ =>
        Err(Error::msg("Invalid provider name")), } }
    );
}
#[cfg(feature = "mock")]
pub(crate) fn take_provider(name: &str) -> Result<Box<dyn Provider>, Error> {
    extern "C" {
        #[link_name = "checkdigit_take_provider__ground_truth"]
        fn take_provider__foreign(_: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a>(&'a str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(Box<dyn Provider>);
    let input_state_in = InputStateIn(name);
    let foreign_execution = unsafe {
        de::<ForeignExecution>(take_provider__foreign(ser(&input_state_in)))
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
pub(crate) fn take_provider__with_callees_mocked(
    name: &str,
) -> Result<Box<dyn Provider>, Error> {
    mock::mock_body!(
        { let sanitized_name = name.to_lowercase().replace("-", ""); match sanitized_name
        .as_str() { "luhn" => Ok(new_luhn()), "verhoeff" =>
        Ok(Box::new(Verhoeff::new_verhoeff())), "damm" => Damm::new_damm().map(| p |
        Box::new(p) as Box < dyn Provider >), "isbn10" => Ok(NewISBN10()), "isbn13" |
        "isbn" => new_isbn13().map(| p | Box::new(p) as Box < dyn Provider >), "ean8" =>
        new_ean8().map(| p | Box::new(p) as Box < dyn Provider >), "ean13" | "ean" =>
        Ok(new_ean13()), "jan8" => Ok(Box::new(gtin::new_jan8())), "jan13" | "jan" =>
        Ok(new_jan13()), "itf" => new_itf().map(| p | Box::new(p) as Box < dyn Provider
        >), "upc" => Ok(new_upc()), "sscc" => Ok(new_sscc()), _ =>
        Err(Error::msg("Invalid provider name")), } }
    );
}
#[cfg(test)]
mod checkdigit_take_provider_harness {
    use super::*;
    #[test]
    fn take_provider__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.takeProvider.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>);
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
                    { (take_provider__with_callees_mocked(&*input_state.0)).unwrap() }
                    #[cfg(not(feature = "mock"))]
                    { (take_provider(&*input_state.0)).unwrap() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn take_provider__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(Box<dyn Provider>);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.takeProvider.json",
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
use crate::__synthetic::__Synth1__verify;
#[cfg(not(feature = "mock"))]
pub(crate) fn verify(v: &dyn Provider, target: &str) -> bool {
    let ret = v.verify(target);
    eprintln!("{}", ret);
    ret
}
#[cfg(feature = "mock")]
pub(crate) fn verify(v: &dyn Provider, target: &str) -> bool {
    extern "C" {
        #[link_name = "checkdigit_verify__ground_truth"]
        fn verify__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn<'a, 'b>(&'a dyn Provider, &'b str);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(Box<dyn Provider>, Box<str>);
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(bool);
    let input_state_in = InputStateIn(v, target);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<ForeignExecution>(verify__foreign(ser(&params[0]), ser(&params[1])))
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
pub(crate) fn verify__with_callees_mocked(v: &dyn Provider, target: &str) -> bool {
    let ret = v.verify(target);
    eprintln!("{}", ret);
    ret
}
#[cfg(test)]
mod checkdigit_verify_harness {
    use super::*;
    #[test]
    fn verify__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.verify.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<dyn Provider>, Box<str>);
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
                    { verify__with_callees_mocked(&*input_state.0, &*input_state.1) }
                    #[cfg(not(feature = "mock"))]
                    { verify(&*input_state.0, &*input_state.1) }
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
    fn verify__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<dyn Provider>, Box<str>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(bool);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.verify.json",
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
pub fn example() -> Result<()> {
    let provider = take_provider("isbn10")?;
    generate(&*provider, "xyz")?;
    if !verify(&*provider, "abc") {
        return Err(anyhow!("failed to verify"));
    }
    Ok(())
}
#[cfg(feature = "mock")]
pub fn example() -> Result<()> {
    extern "C" {
        #[link_name = "checkdigit_example__ground_truth"]
        fn example__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(example__foreign()) };
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
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn example__with_callees_mocked() -> Result<()> {
    let provider = take_provider("isbn10")?;
    generate(&*provider, "xyz")?;
    if !verify(&*provider, "abc") {
        return Err(anyhow!("failed to verify"));
    }
    Ok(())
}
#[cfg(test)]
mod checkdigit_example_harness {
    use super::*;
    #[test]
    fn example__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-osamingo-checkdigit.Example.json",
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
                    { (example__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (example()).unwrap() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn example__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-osamingo-checkdigit.Example.json",
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
