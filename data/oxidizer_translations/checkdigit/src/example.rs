#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use std::io::stderr;
use std::io::Write;
use crate::checkdigit::Provider;
use crate::__synthetic::__Synth0__generate;
//Translated from: github.com/osamingo/checkdigit.generate
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
use crate::checkdigit::new_ean13;
use crate::checkdigit::new_ean8;
use crate::checkdigit::NewISBN10;
use crate::checkdigit::new_isbn13;
use crate::checkdigit::new_itf;
use crate::checkdigit::new_jan13;
use crate::checkdigit::new_luhn;
use crate::checkdigit::new_sscc;
use crate::checkdigit::new_upc;
//Translated from: github.com/osamingo/checkdigit.takeProvider
// mock commented out
// pub(crate) fn take_provider(name: &str) -> Result<Box<dyn Provider>, Error> { mock::mock_body!({
//     let sanitized_name = name.to_lowercase().replace("-", "");
//
//     match sanitized_name.as_str() {
//         "luhn" => Ok(new_luhn()),
//         "verhoeff" => Ok(Box::new(Verhoeff::new_verhoeff())),
//         "damm" => Damm::new_damm().map(|p| Box::new(p) as Box<dyn Provider>),
//         "isbn10" => Ok(NewISBN10()),
//         "isbn13" | "isbn" => new_isbn13().map(|p| Box::new(p) as Box<dyn Provider>),
//         "ean8" => new_ean8().map(|p| Box::new(p) as Box<dyn Provider>),
//         "ean13" | "ean" => Ok(new_ean13()),
//         "jan8" => Ok(Box::new(gtin::new_jan8())),
//         "jan13" | "jan" => Ok(new_jan13()),
//         "itf" => new_itf().map(|p| Box::new(p) as Box<dyn Provider>),
//         "upc" => Ok(new_upc()),
//         "sscc" => Ok(new_sscc()),
//         _ => Err(Error::msg("Invalid provider name")),
//     }
// });}
use crate::__synthetic::__Synth1__verify;
//Translated from: github.com/osamingo/checkdigit.verify
pub(crate) fn verify(v: &dyn Provider, target: &str) -> bool {
    let ret = v.verify(target);
    eprintln!("{}", ret);
    ret
}

//Translated from: github.com/osamingo/checkdigit.Example
// mock commented out - uses take_provider
// pub fn example() -> Result<()> {
//     let provider = take_provider("isbn10")?;
//
//     generate(&*provider, "xyz")?;
//
//     if !verify(&*provider, "abc") {
//         return Err(anyhow!("failed to verify"));
//     }
//
//     Ok(())
// }
