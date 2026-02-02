use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::histogram::Bin;
use std::collections::VecDeque;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct NumericHistogram {
    #[serde(rename = "bins")]
    pub(crate) bins: VecDeque<Bin>,
    #[serde(rename = "maxbins")]
    pub(crate) maxbins: usize,
    #[serde(rename = "total")]
    pub(crate) total: u64,
}
#[cfg(test)]
mod gohistogram_NumericHistogram_interoperation_tests {
    use super::*;
    extern "C" {
        #[link_name = "gohistogram_NumericHistogram_roundtrip"]
        fn NumericHistogram__roundtrip(_: JSONObject) -> JSONObject;
    }
    #[test]
    fn NumericHistogram__weak__interoperation() {
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
                                "(*{}).",
                                "github.com-VividCortex-gohistogram.NumericHistogram"
                            ),
                        )
                        || entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .starts_with(
                                &format!(
                                    "({}).",
                                    "github.com-VividCortex-gohistogram.NumericHistogram"
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
                                    serde_json::from_value::<NumericHistogram>(obj).unwrap(),
                                )
                                .unwrap();
                            let obj_twice = serde_json::to_value(
                                    serde_json::from_value::<NumericHistogram>(obj_once.clone())
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
use std::f64;
#[cfg(not(feature = "mock"))]
impl NumericHistogram {
    pub(crate) fn trim(&mut self) -> Result<()> {
        while self.bins.len() > self.maxbins {
            let mut min_delta = f64::INFINITY;
            let mut min_delta_index = 0;
            for i in 1..self.bins.len() {
                let delta = self.bins[i].value - self.bins[i - 1].value;
                if delta < min_delta {
                    min_delta = delta;
                    min_delta_index = i;
                }
            }
            let total_count = self.bins[min_delta_index - 1].count
                + self.bins[min_delta_index].count;
            let merged_bin = Bin {
                value: (self.bins[min_delta_index - 1].value
                    * self.bins[min_delta_index - 1].count
                    + self.bins[min_delta_index].value
                        * self.bins[min_delta_index].count) / total_count,
                count: total_count,
            };
            let mut head = self
                .bins
                .drain(..min_delta_index - 1)
                .collect::<VecDeque<_>>();
            let tail = self.bins.drain(min_delta_index + 1..).collect::<VecDeque<_>>();
            head.push_back(merged_bin);
            self.bins = head;
            self.bins.extend(tail);
        }
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub(crate) fn trim(&mut self) -> Result<()> {
        extern "C" {
            #[link_name = "gohistogram_numeric_histogram___trim__ground_truth"]
            fn NumericHistogram_trim__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut NumericHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(NumericHistogram_trim__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
            };
            let input_state_mutated: InputStateOut = serde_json::from_value(
                    inputs_mutation_reserialized,
                )
                .unwrap();
            *self = *input_state_mutated.0;
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
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub(crate) fn trim__with_callees_mocked(&mut self) -> Result<()> {
        while self.bins.len() > self.maxbins {
            let mut min_delta = f64::INFINITY;
            let mut min_delta_index = 0;
            for i in 1..self.bins.len() {
                let delta = self.bins[i].value - self.bins[i - 1].value;
                if delta < min_delta {
                    min_delta = delta;
                    min_delta_index = i;
                }
            }
            let total_count = self.bins[min_delta_index - 1].count
                + self.bins[min_delta_index].count;
            let merged_bin = Bin {
                value: (self.bins[min_delta_index - 1].value
                    * self.bins[min_delta_index - 1].count
                    + self.bins[min_delta_index].value
                        * self.bins[min_delta_index].count) / total_count,
                count: total_count,
            };
            let mut head = self
                .bins
                .drain(..min_delta_index - 1)
                .collect::<VecDeque<_>>();
            let tail = self.bins.drain(min_delta_index + 1..).collect::<VecDeque<_>>();
            head.push_back(merged_bin);
            self.bins = head;
            self.bins.extend(tail);
        }
        Ok(())
    }
}
#[cfg(test)]
mod gohistogram_numeric_histogram___trim_harness {
    use super::*;
    #[test]
    fn NumericHistogram_trim__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).trim.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState;
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { ((&mut *input_state.0).trim__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))]
                    { ((&mut *input_state.0).trim()).unwrap() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState;
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_trim__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).trim.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}
use crate::__synthetic::__Synth0__add;
#[cfg(not(feature = "mock"))]
impl __Synth0__add for NumericHistogram {
    fn add(&mut self, n: f64) {
        self.trim().unwrap();
        self.total += 1;
        for i in 0..self.bins.len() {
            if self.bins[i].value == n {
                self.bins[i].count += 1.0;
                return;
            }
            if self.bins[i].value > n {
                let newbin = Bin { value: n, count: 1.0 };
                let mut head = VecDeque::from_iter(self.bins.iter().take(i).cloned());
                head.push_back(newbin);
                let tail = self.bins.split_off(i);
                self.bins = head.into_iter().chain(tail.into_iter()).collect();
                return;
            }
        }
        self.bins.push_back(Bin { count: 1.0, value: n });
    }
}
#[cfg(feature = "mock")]
impl __Synth0__add for NumericHistogram {
    fn add(&mut self, n: f64) {
        extern "C" {
            #[link_name = "gohistogram_numeric_histogram___add__ground_truth"]
            fn NumericHistogram_add__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(
            &'a mut NumericHistogram,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, n);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NumericHistogram_add__foreign(ser(&params[0]), ser(&params[1])))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 2usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
            };
            let input_state_mutated: InputStateOut = serde_json::from_value(
                    inputs_mutation_reserialized,
                )
                .unwrap();
            *self = *input_state_mutated.0;
            let output_state: OutputState = serde_json::from_value(
                    foreign_execution.return_value,
                )
                .unwrap();
            let output = ();
            return output;
        } else {
            panic!("execution failure");
        }
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    fn add__with_callees_mocked(&mut self, n: f64) {
        self.trim().unwrap();
        self.total += 1;
        for i in 0..self.bins.len() {
            if self.bins[i].value == n {
                self.bins[i].count += 1.0;
                return;
            }
            if self.bins[i].value > n {
                let newbin = Bin { value: n, count: 1.0 };
                let mut head = VecDeque::from_iter(self.bins.iter().take(i).cloned());
                head.push_back(newbin);
                let tail = self.bins.split_off(i);
                self.bins = head.into_iter().chain(tail.into_iter()).collect();
                return;
            }
        }
        self.bins.push_back(Bin { count: 1.0, value: n });
    }
}
#[cfg(test)]
mod gohistogram_numeric_histogram___add_harness {
    use super::*;
    #[test]
    fn NumericHistogram_add__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Add.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState;
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (&mut *input_state.0).add__with_callees_mocked(input_state.1) }
                    #[cfg(not(feature = "mock"))]
                    { (&mut *input_state.0).add(input_state.1) }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState;
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_add__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Add.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(not(feature = "mock"))]
impl NumericHistogram {
    pub fn cdf(&self, x: f64) -> f64 {
        let mut count = 0.0;
        for bin in &self.bins {
            if bin.value <= x {
                count += bin.count;
            }
        }
        count / self.total as f64
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn cdf(&self, x: f64) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_numeric_histogram__cdf__ground_truth"]
            fn NumericHistogram_cdf__foreign(_: JSONObject, _: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(
            &'a NumericHistogram,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self, x);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NumericHistogram_cdf__foreign(ser(&params[0]), ser(&params[1])))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 2usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn cdf__with_callees_mocked(&self, x: f64) -> f64 {
        let mut count = 0.0;
        for bin in &self.bins {
            if bin.value <= x {
                count += bin.count;
            }
        }
        count / self.total as f64
    }
}
#[cfg(test)]
mod gohistogram_numeric_histogram__cdf_harness {
    use super::*;
    #[test]
    fn NumericHistogram_cdf__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).CDF.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (&*input_state.0).cdf__with_callees_mocked(input_state.1) }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).cdf(input_state.1) }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_cdf__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).CDF.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(not(feature = "mock"))]
impl NumericHistogram {
    pub fn count(&self) -> f64 {
        self.total as f64
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn count(&self) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_numeric_histogram___count__ground_truth"]
            fn NumericHistogram_count__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a NumericHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(NumericHistogram_count__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn count__with_callees_mocked(&self) -> f64 {
        self.total as f64
    }
}
#[cfg(test)]
mod gohistogram_numeric_histogram___count_harness {
    use super::*;
    #[test]
    fn NumericHistogram_count__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Count.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (&*input_state.0).count__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { (&*input_state.0).count() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_count__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Count.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(not(feature = "mock"))]
impl NumericHistogram {
    pub fn mean(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for bin in &self.bins {
            sum += bin.value * bin.count;
        }
        sum / self.total as f64
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn mean(&self) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_numeric_histogram___mean__ground_truth"]
            fn NumericHistogram_mean__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a NumericHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(NumericHistogram_mean__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn mean__with_callees_mocked(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for bin in &self.bins {
            sum += bin.value * bin.count;
        }
        sum / self.total as f64
    }
}
#[cfg(test)]
mod gohistogram_numeric_histogram___mean_harness {
    use super::*;
    #[test]
    fn NumericHistogram_mean__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Mean.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (&*input_state.0).mean__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { (&*input_state.0).mean() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_mean__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Mean.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(not(feature = "mock"))]
impl NumericHistogram {
    pub fn quantile(&self, q: f64) -> f64 {
        let mut count = q * (self.total as f64);
        for bin in &self.bins {
            count -= bin.count;
            if count <= 0.0 {
                return bin.value;
            }
        }
        -1.0
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn quantile(&self, q: f64) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_numeric_histogram___quantile__ground_truth"]
            fn NumericHistogram_quantile__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(
            &'a NumericHistogram,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self, q);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NumericHistogram_quantile__foreign(ser(&params[0]), ser(&params[1])))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 2usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn quantile__with_callees_mocked(&self, q: f64) -> f64 {
        let mut count = q * (self.total as f64);
        for bin in &self.bins {
            count -= bin.count;
            if count <= 0.0 {
                return bin.value;
            }
        }
        -1.0
    }
}
#[cfg(test)]
mod gohistogram_numeric_histogram___quantile_harness {
    use super::*;
    #[test]
    fn NumericHistogram_quantile__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Quantile.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (&*input_state.0).quantile__with_callees_mocked(input_state.1) }
                    #[cfg(not(feature = "mock"))]
                    { (&*input_state.0).quantile(input_state.1) }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_quantile__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(
            Box<NumericHistogram>,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Quantile.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(not(feature = "mock"))]
impl NumericHistogram {
    pub fn variance(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        let mut sum = 0.0;
        let mean = self.mean();
        for bin in &self.bins {
            sum += bin.count * (bin.value - mean).powi(2);
        }
        sum / self.total as f64
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn variance(&self) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_numeric_histogram___variance__ground_truth"]
            fn NumericHistogram_variance__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a NumericHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NumericHistogram_variance__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn variance__with_callees_mocked(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        let mut sum = 0.0;
        let mean = self.mean();
        for bin in &self.bins {
            sum += bin.count * (bin.value - mean).powi(2);
        }
        sum / self.total as f64
    }
}
#[cfg(test)]
mod gohistogram_numeric_histogram___variance_harness {
    use super::*;
    #[test]
    fn NumericHistogram_variance__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Variance.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    { (&*input_state.0).variance__with_callees_mocked() }
                    #[cfg(not(feature = "mock"))] { (&*input_state.0).variance() }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(& input_state.0).unwrap(),
                        serde_json::to_value(& input_state_mutated.0).unwrap(),
                    );
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_variance__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(Box<NumericHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/(*github.com-VividCortex-gohistogram.NumericHistogram).Variance.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}

#[cfg(not(feature = "mock"))]
impl NumericHistogram {
    pub fn new_histogram(n: usize) -> NumericHistogram {
        NumericHistogram {
            bins: VecDeque::new(),
            maxbins: n,
            total: 0,
        }
    }
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn new_histogram(n: usize) -> NumericHistogram {
        extern "C" {
            #[link_name = "gohistogram_new_histogram__ground_truth"]
            fn NumericHistogram_new_histogram__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn(usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(NumericHistogram);
        let input_state_in = InputStateIn(n);
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(NumericHistogram_new_histogram__foreign(ser(&input_state_in)))
        };
        if foreign_execution.execution_success {
            assert_eq!(foreign_execution.input_modifications.len(), 1usize);
            let inputs_mutation_reserialized = if foreign_execution
                .input_modifications
                .len() == 1
            {
                foreign_execution.input_modifications[0].clone()
            } else {
                serde_json::to_value(foreign_execution.input_modifications.clone())
                    .unwrap()
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
}
#[cfg(feature = "mock")]
impl NumericHistogram {
    pub fn new_histogram__with_callees_mocked(n: usize) -> NumericHistogram {
        NumericHistogram {
            bins: VecDeque::new(),
            maxbins: n,
            total: 0,
        }
    }
}
#[cfg(test)]
mod gohistogram_new_histogram_harness {
    use super::*;
    #[test]
    fn NumericHistogram_new_histogram__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-VividCortex-gohistogram.NewHistogram.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(usize);
        #[serde_as]
        #[derive(Serialize)]
        struct OutputState(NumericHistogram);
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let mut input_state: InputState = serde_json::from_value(inputs_reserialized)
                .unwrap();
            struct NonCopyableMarker;
            let force_fn_once: NonCopyableMarker = NonCopyableMarker;
            let return_value = std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _force_fn_once = force_fn_once;
                    #[cfg(feature = "mock")]
                    {
                        NumericHistogram::new_histogram__with_callees_mocked(
                            input_state.0,
                        )
                    }
                    #[cfg(not(feature = "mock"))]
                    { NumericHistogram::new_histogram(input_state.0) }
                }),
            );
            match return_value {
                Ok(mut return_value) => {
                    assert!(execution.result.execution_success);
                    let output_state = OutputState(return_value);
                    assert_json_diff::assert_json_eq!(
                        serde_json::to_value(output_state).unwrap(), execution.result
                        .return_value.clone()
                    );
                    let inputs_mutation_reserialized = if execution
                        .result
                        .input_modifications
                        .len() == 1
                    {
                        execution.result.input_modifications[0].clone()
                    } else {
                        serde_json::to_value(
                                execution.result.input_modifications.clone(),
                            )
                            .unwrap()
                    };
                    let input_state_mutated: InputState = serde_json::from_value(
                            inputs_mutation_reserialized,
                        )
                        .unwrap();
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn NumericHistogram_new_histogram__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState(usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(NumericHistogram);
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-VividCortex-gohistogram.NewHistogram.json",
        ) else { return };
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        for execution in unittests {
            let inputs_reserialized = if execution.inputs.len() == 1 {
                execution.inputs[0].clone()
            } else {
                serde_json::to_value(execution.inputs.clone()).unwrap()
            };
            let _: InputState = serde_json::from_value(inputs_reserialized).unwrap();
            if execution.result.execution_success {
                let _: OutputState = serde_json::from_value(
                        execution.result.return_value.clone(),
                    )
                    .unwrap();
            }
        }
    }
}
