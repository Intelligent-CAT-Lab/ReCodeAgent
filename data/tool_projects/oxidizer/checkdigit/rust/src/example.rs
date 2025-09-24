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

