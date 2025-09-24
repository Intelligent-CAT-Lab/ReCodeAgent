use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct StatsError {
    #[serde(rename = "err")]
    pub(crate) err: String,
}

use once_cell::sync::Lazy;
pub static ERR_EMPTY_INPUT: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Input must not be empty.".to_string(),
    }
});

impl std::fmt::Display for StatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}
impl std::error::Error for StatsError {}
impl std::fmt::Debug for StatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatsError {{ err: {} }}", self.err)
    }
}

pub static ERR_SIZE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Must be the same length.".to_string(),
    }
});

pub static ERR_BOUNDS: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Input is outside of range.".to_string(),
    }
});

pub static ERR_NEGATIVE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Must not contain negative values.".to_string(),
    }
});

pub static ERR_Y_COORD: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Y Value must be greater than zero.".to_string(),
    }
});

pub static ERR_ZERO: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: String::from("Must not contain zero values."),
    }
});

pub static ERR_INF_VALUE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Value is infinite.".to_string(),
    }
});

pub static ERR_NAN: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Not a number.".to_string(),
    }
});

