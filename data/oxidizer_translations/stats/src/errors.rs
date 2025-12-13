#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

//Translated from: github.com/montanaflynn/stats.statsError
#[derive(Default)]#[derive(Clone)]pub struct StatsError {
    pub(crate) err: String,
}

// Stubbed constructor and methods to mirror the translated API surface.
impl StatsError {
    pub fn new(_err: String) -> Self {
        todo!("stubbed StatsError::new")
    }

    pub fn Error(&self) -> String {
        todo!("stubbed StatsError::Error")
    }

    pub fn String(&self) -> String {
        todo!("stubbed StatsError::String")
    }
}

use once_cell::sync::Lazy;
//Translated from: github.com/montanaflynn/stats.ErrEmptyInput
pub static ERR_EMPTY_INPUT: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Input must not be empty.".to_string(),
    }
});

//Translated from: (github.com/montanaflynn/stats.statsError).Error
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

//Translated from: github.com/montanaflynn/stats.ErrSize
pub static ERR_SIZE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Must be the same length.".to_string(),
    }
});

//Translated from: github.com/montanaflynn/stats.ErrBounds
pub static ERR_BOUNDS: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Input is outside of range.".to_string(),
    }
});

//Translated from: github.com/montanaflynn/stats.ErrNegative
pub static ERR_NEGATIVE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Must not contain negative values.".to_string(),
    }
});

//Translated from: github.com/montanaflynn/stats.ErrYCoord
pub static ERR_Y_COORD: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Y Value must be greater than zero.".to_string(),
    }
});

//Translated from: github.com/montanaflynn/stats.ErrZero
pub static ERR_ZERO: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: String::from("Must not contain zero values."),
    }
});

//Translated from: github.com/montanaflynn/stats.ErrInfValue
pub static ERR_INF_VALUE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Value is infinite.".to_string(),
    }
});

//Translated from: github.com/montanaflynn/stats.ErrNaN
pub static ERR_NAN: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Not a number.".to_string(),
    }
});
