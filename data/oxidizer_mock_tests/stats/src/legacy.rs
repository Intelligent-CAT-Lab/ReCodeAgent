use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::errors::ERR_EMPTY_INPUT;
use crate::errors::StatsError;
use once_cell::sync::Lazy;
pub static EMPTY_INPUT_ERR: Lazy<StatsError> = Lazy::new(|| ERR_EMPTY_INPUT.clone());
#[cfg(test)]
mod EMPTY_INPUT_ERR__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_EmptyInputErr__initial_value"]
        fn EMPTY_INPUT_ERR__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn EMPTY_INPUT_ERR__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(EMPTY_INPUT_ERR__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn EMPTY_INPUT_ERR__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&EMPTY_INPUT_ERR));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (EMPTY_INPUT_ERR__initial_value_foreign()) }
        );
    }
}
use crate::errors::ERR_SIZE;
pub static SIZE_ERR: Lazy<StatsError> = Lazy::new(|| ERR_SIZE.clone());
#[cfg(test)]
mod SIZE_ERR__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_SizeErr__initial_value"]
        fn SIZE_ERR__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn SIZE_ERR__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(SIZE_ERR__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn SIZE_ERR__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&SIZE_ERR));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (SIZE_ERR__initial_value_foreign()) }
        );
    }
}

pub static EMPTY_INPUT: Lazy<StatsError> = Lazy::new(|| ERR_EMPTY_INPUT.clone());
#[cfg(test)]
mod EMPTY_INPUT__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_EmptyInput__initial_value"]
        fn EMPTY_INPUT__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn EMPTY_INPUT__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(EMPTY_INPUT__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn EMPTY_INPUT__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&EMPTY_INPUT));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (EMPTY_INPUT__initial_value_foreign()) }
        );
    }
}
use crate::errors::ERR_BOUNDS;
pub static BOUNDS_ERR: Lazy<StatsError> = Lazy::new(|| ERR_BOUNDS.clone());
#[cfg(test)]
mod BOUNDS_ERR__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_BoundsErr__initial_value"]
        fn BOUNDS_ERR__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn BOUNDS_ERR__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(BOUNDS_ERR__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn BOUNDS_ERR__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&BOUNDS_ERR));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (BOUNDS_ERR__initial_value_foreign()) }
        );
    }
}
use crate::errors::ERR_Y_COORD;
pub static Y_COORD_ERR: Lazy<StatsError> = Lazy::new(|| ERR_Y_COORD.clone());
#[cfg(test)]
mod Y_COORD_ERR__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_YCoordErr__initial_value"]
        fn Y_COORD_ERR__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn Y_COORD_ERR__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(Y_COORD_ERR__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn Y_COORD_ERR__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&Y_COORD_ERR));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (Y_COORD_ERR__initial_value_foreign()) }
        );
    }
}
use crate::errors::ERR_NEGATIVE;
pub static NEGATIVE_ERR: Lazy<StatsError> = Lazy::new(|| { ERR_NEGATIVE.clone() });
#[cfg(test)]
mod NEGATIVE_ERR__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_NegativeErr__initial_value"]
        fn NEGATIVE_ERR__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn NEGATIVE_ERR__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(NEGATIVE_ERR__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn NEGATIVE_ERR__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&NEGATIVE_ERR));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (NEGATIVE_ERR__initial_value_foreign()) }
        );
    }
}
use crate::errors::ERR_ZERO;
pub static ZERO_ERR: Lazy<StatsError> = Lazy::new(|| ERR_ZERO.clone());
#[cfg(test)]
mod ZERO_ERR__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ZeroErr__initial_value"]
        fn ZERO_ERR__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ZERO_ERR__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ZERO_ERR__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ZERO_ERR__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ZERO_ERR));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ZERO_ERR__initial_value_foreign()) }
        );
    }
}
use crate::errors::ERR_INF_VALUE;
pub static INF_VALUE: Lazy<StatsError> = Lazy::new(|| ERR_INF_VALUE.clone());
#[cfg(test)]
mod INF_VALUE__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_InfValue__initial_value"]
        fn INF_VALUE__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn INF_VALUE__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(INF_VALUE__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn INF_VALUE__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&INF_VALUE));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (INF_VALUE__initial_value_foreign()) }
        );
    }
}
use crate::errors::ERR_NAN;
pub static NAN_ERR: Lazy<StatsError> = Lazy::new(|| { ERR_NAN.clone() });
#[cfg(test)]
mod NAN_ERR__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_NaNErr__initial_value"]
        fn NAN_ERR__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn NAN_ERR__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe { de(NAN_ERR__initial_value_foreign()) };
        let _ = foreign_value;
    }
    #[test]
    fn NAN_ERR__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&NAN_ERR));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (NAN_ERR__initial_value_foreign()) }
        );
    }
}
