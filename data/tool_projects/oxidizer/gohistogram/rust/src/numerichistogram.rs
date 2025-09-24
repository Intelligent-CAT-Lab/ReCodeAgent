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

