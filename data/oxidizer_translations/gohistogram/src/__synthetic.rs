#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: InterfaceMethod0.__Synthetic.github.com/VividCortex/gohistogram.Histogram
pub trait __Synth0__add {
    fn add(&mut self, _: f64);
}
macro_rules! delegate___Synth0__add {
    ($type_name:path, $field_name:ident) => {
        impl __Synth0__add for $type_name {
            delegate::delegate! {
                to self.$field_name {
                    fn add(&mut self, input0: f64);
                }
            }
        }
    };
}
macro_rules! use_delegate___Synth0__add {
    () => {
        pub(crate) use delegate___Synth0__add;
    }
}
use_delegate___Synth0__add!();

//Translated from: InterfaceMethod1.__Synthetic.github.com/VividCortex/gohistogram.Histogram
pub trait __Synth1__quantile {
    fn quantile(&self, n: f64) -> f64;                
}

macro_rules! delegate___Synth1__quantile {
    ($type_name:path, $field_name:ident) => {
        impl __Synth1__quantile for $type_name {
            delegate::delegate! {
                to self.$field_name {
                    fn quantile(&self, input0: f64) -> f64;
                }
            }
        }
    };
}
macro_rules! use_delegate___Synth1__quantile {
    () => {
        pub(crate) use delegate___Synth1__quantile;
    }
}
use_delegate___Synth1__quantile!();

