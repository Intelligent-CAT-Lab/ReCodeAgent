use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;


#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct StatsError {
    #[serde(rename = "err")]
    pub(crate) err: String,
}
#[cfg(test)]
mod stats_statsError_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "stats_statsError_roundtrip"]
        fn StatsError__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn StatsError__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!(
                                "(*{}).", "github.com-montanaflynn-stats.statsError"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).", "github.com-montanaflynn-stats.statsError"
                                ),
                            )
                    {
                        let unittests_file: std::fs::File = std::fs::File::open(
                                entry.path(),
                            )
                            .unwrap();
                        let unittests_reader = std::io::BufReader::new(unittests_file);
                        let unittests: Vec<ExecutionData> = serde_json::from_reader(
                                unittests_reader,
                            )
                            .unwrap();
                        for unittest in unittests {
                            let obj = unittest.inputs[0].clone();
                            if obj == serde_json::Value::Null {
                                continue;
                            }
                            let obj_once = serde_json::to_value(
                                    serde_json::from_value::<StatsError>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<StatsError>(obj_once.clone())
                                        .unwrap(),
                                )
                                .unwrap();
                            assert_json_diff::assert_json_eq!(obj_once, obj_twice);
                        }
                    }
                }
            }
        }
    }
}
use once_cell::sync::Lazy;
pub static ERR_EMPTY_INPUT: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Input must not be empty.".to_string(),
    }
});
#[cfg(test)]
mod ERR_EMPTY_INPUT__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrEmptyInput__initial_value"]
        fn ERR_EMPTY_INPUT__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_EMPTY_INPUT__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ERR_EMPTY_INPUT__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_EMPTY_INPUT__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_EMPTY_INPUT));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_EMPTY_INPUT__initial_value_foreign()) }
        );
    }
}

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
#[cfg(test)]
mod ERR_SIZE__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrSize__initial_value"]
        fn ERR_SIZE__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_SIZE__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ERR_SIZE__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_SIZE__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_SIZE));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_SIZE__initial_value_foreign()) }
        );
    }
}

pub static ERR_BOUNDS: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Input is outside of range.".to_string(),
    }
});
#[cfg(test)]
mod ERR_BOUNDS__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrBounds__initial_value"]
        fn ERR_BOUNDS__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_BOUNDS__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ERR_BOUNDS__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_BOUNDS__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_BOUNDS));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_BOUNDS__initial_value_foreign()) }
        );
    }
}

pub static ERR_NEGATIVE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Must not contain negative values.".to_string(),
    }
});
#[cfg(test)]
mod ERR_NEGATIVE__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrNegative__initial_value"]
        fn ERR_NEGATIVE__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_NEGATIVE__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ERR_NEGATIVE__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_NEGATIVE__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_NEGATIVE));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_NEGATIVE__initial_value_foreign()) }
        );
    }
}

pub static ERR_Y_COORD: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Y Value must be greater than zero.".to_string(),
    }
});
#[cfg(test)]
mod ERR_Y_COORD__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrYCoord__initial_value"]
        fn ERR_Y_COORD__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_Y_COORD__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ERR_Y_COORD__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_Y_COORD__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_Y_COORD));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_Y_COORD__initial_value_foreign()) }
        );
    }
}

pub static ERR_ZERO: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: String::from("Must not contain zero values."),
    }
});
#[cfg(test)]
mod ERR_ZERO__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrZero__initial_value"]
        fn ERR_ZERO__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_ZERO__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ERR_ZERO__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_ZERO__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_ZERO));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_ZERO__initial_value_foreign()) }
        );
    }
}

pub static ERR_INF_VALUE: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Value is infinite.".to_string(),
    }
});
#[cfg(test)]
mod ERR_INF_VALUE__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrInfValue__initial_value"]
        fn ERR_INF_VALUE__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_INF_VALUE__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe {
            de(ERR_INF_VALUE__initial_value_foreign())
        };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_INF_VALUE__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_INF_VALUE));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_INF_VALUE__initial_value_foreign()) }
        );
    }
}

pub static ERR_NAN: Lazy<StatsError> = Lazy::new(|| {
    StatsError {
        err: "Not a number.".to_string(),
    }
});
#[cfg(test)]
mod ERR_NAN__initialization_check {
    use super::*;
    extern "C" {
        #[link_name = "stats_ErrNaN__initial_value"]
        fn ERR_NAN__initial_value_foreign() -> JSONObject;
    }
    #[test]
    fn ERR_NAN__static_type_compatibility() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(StatsError);
        let foreign_value: OutputState = unsafe { de(ERR_NAN__initial_value_foreign()) };
        let _ = foreign_value;
    }
    #[test]
    fn ERR_NAN__initialization_check() {
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState<'a>(&'a StatsError);
        let initial_value = OutputState(once_cell::sync::Lazy::force(&ERR_NAN));
        assert_json_diff::assert_json_eq!(
            serde_json::to_value(initial_value).unwrap(), unsafe { de:: <
            serde_json::Value > (ERR_NAN__initial_value_foreign()) }
        );
    }
}
