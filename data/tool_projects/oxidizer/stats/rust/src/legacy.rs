use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::ERR_EMPTY_INPUT;
use crate::errors::StatsError;
use once_cell::sync::Lazy;
pub static EMPTY_INPUT_ERR: Lazy<StatsError> = Lazy::new(|| ERR_EMPTY_INPUT.clone());

use crate::errors::ERR_SIZE;
pub static SIZE_ERR: Lazy<StatsError> = Lazy::new(|| ERR_SIZE.clone());

pub static EMPTY_INPUT: Lazy<StatsError> = Lazy::new(|| ERR_EMPTY_INPUT.clone());

use crate::errors::ERR_BOUNDS;
pub static BOUNDS_ERR: Lazy<StatsError> = Lazy::new(|| ERR_BOUNDS.clone());

use crate::errors::ERR_Y_COORD;
pub static Y_COORD_ERR: Lazy<StatsError> = Lazy::new(|| ERR_Y_COORD.clone());

use crate::errors::ERR_NEGATIVE;
pub static NEGATIVE_ERR: Lazy<StatsError> = Lazy::new(|| { ERR_NEGATIVE.clone() });

use crate::errors::ERR_ZERO;
pub static ZERO_ERR: Lazy<StatsError> = Lazy::new(|| ERR_ZERO.clone());

use crate::errors::ERR_INF_VALUE;
pub static INF_VALUE: Lazy<StatsError> = Lazy::new(|| ERR_INF_VALUE.clone());

use crate::errors::ERR_NAN;
pub static NAN_ERR: Lazy<StatsError> = Lazy::new(|| { ERR_NAN.clone() });

