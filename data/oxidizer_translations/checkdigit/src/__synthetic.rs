#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: InterfaceMethod0.__Synthetic.github.com/osamingo/checkdigit.Generator
pub trait __Synth0__generate {
    fn generate(&self, _: &str) -> Result<i32, &'static str>;
}
macro_rules! delegate___Synth0__generate {
    ($type_name:path, $field_name:ident) => {
        impl __Synth0__generate for $type_name {
            delegate::delegate! {
                to self.$field_name {
                    fn generate(&self, input0: &str) -> Result<i32, &'static str>;
                }
            }
        }
    };
}
macro_rules! use_delegate___Synth0__generate {
    () => {
        pub(crate) use delegate___Synth0__generate;
    }
}
use_delegate___Synth0__generate!();

//Translated from: InterfaceMethod1.__Synthetic.github.com/osamingo/checkdigit.Verifier
pub trait __Synth1__verify {
    fn verify(&self, _: &str) -> bool;
}
macro_rules! delegate___Synth1__verify {
    ($type_name:path, $field_name:ident) => {
        impl __Synth1__verify for $type_name {
            delegate::delegate! {
                to self.$field_name {
                    fn verify(&self, input0: &str) -> bool;
                }
            }
        }
    };
}
macro_rules! use_delegate___Synth1__verify {
    () => {
        pub(crate) use delegate___Synth1__verify;
    }
}
use_delegate___Synth1__verify!();

