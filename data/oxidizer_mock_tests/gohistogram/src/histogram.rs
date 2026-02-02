use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;


#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[derive(Default)]
#[derive(Clone)]
pub(crate) struct Bin {
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "value")]
    pub(crate) value: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "count")]
    pub(crate) count: f64,
}
#[cfg(test)]
mod gohistogram_bin_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "gohistogram_bin_roundtrip"]
        fn Bin__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn Bin__weak__interoperation() {
        let testexecs = "./exec-snapshots";
        if let Ok(entries) = std::fs::read_dir(testexecs) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(
                            &format!("(*{}).", "github.com-VividCortex-gohistogram.bin"),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!("({}).", "github.com-VividCortex-gohistogram.bin"),
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
                                    serde_json::from_value::<Bin>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<Bin>(obj_once.clone()).unwrap(),
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

#[typetag::serde(tag = "Type", content = "Value")]
pub trait Histogram: crate::__synthetic::__Synth0__add + crate::__synthetic::__Synth1__quantile {}
