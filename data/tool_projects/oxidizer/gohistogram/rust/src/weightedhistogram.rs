use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

#[cfg(not(feature = "mock"))]
pub(crate) fn ewma(
    existing_val: f64,
    new_val: f64,
    alpha: f64,
) -> Result<f64, anyhow::Error> {
    let result = new_val * (1.0 - alpha) + existing_val * alpha;
    Ok(result)
}
#[cfg(feature = "mock")]
pub(crate) fn ewma(
    existing_val: f64,
    new_val: f64,
    alpha: f64,
) -> Result<f64, anyhow::Error> {
    extern "C" {
        #[link_name = "gohistogram_ewma__ground_truth"]
        fn ewma__foreign(_: JSONObject, _: JSONObject, _: JSONObject) -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState(
        #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
        f64,
    );
    let input_state_in = InputStateIn(existing_val, new_val, alpha);
    let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
    let serde_json::Value::Array(params) = input_state_serialized else {
        panic!("expect multiple input arguments")
    };
    let foreign_execution = unsafe {
        de::<
            ForeignExecution,
        >(ewma__foreign(ser(&params[0]), ser(&params[1]), ser(&params[2])))
    };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 3usize);
        let inputs_mutation_reserialized = if foreign_execution.input_modifications.len()
            == 1
        {
            foreign_execution.input_modifications[0].clone()
        } else {
            serde_json::to_value(foreign_execution.input_modifications.clone()).unwrap()
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
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub(crate) fn ewma__with_callees_mocked(
    existing_val: f64,
    new_val: f64,
    alpha: f64,
) -> Result<f64, anyhow::Error> {
    let result = new_val * (1.0 - alpha) + existing_val * alpha;
    Ok(result)
}

use crate::histogram::Bin;
use std::vec::Vec;
#[serde_as]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct WeightedHistogram {
    #[serde_as(as = "serde_with::DefaultOnNull")]
    #[serde(rename = "bins")]
    pub(crate) bins: Vec<Bin>,
    #[serde(rename = "maxbins")]
    pub(crate) maxbins: usize,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "total")]
    pub(crate) total: f64,
    #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
    #[serde(rename = "alpha")]
    pub(crate) alpha: f64,
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub(crate) fn scale_down(&mut self, except: usize) {
        for i in 0..self.bins.len() {
            if i != except {
                self.bins[i].count = ewma(self.bins[i].count, 0.0, self.alpha).unwrap();
            }
        }
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub(crate) fn scale_down(&mut self, except: usize) {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram___scale_down__ground_truth"]
            fn WeightedHistogram_scale_down__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut WeightedHistogram, usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<WeightedHistogram>, usize);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self, except);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(WeightedHistogram_scale_down__foreign(ser(&params[0]), ser(&params[1])))
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
impl WeightedHistogram {
    pub(crate) fn scale_down__with_callees_mocked(&mut self, except: usize) {
        for i in 0..self.bins.len() {
            if i != except {
                self.bins[i].count = ewma(self.bins[i].count, 0.0, self.alpha).unwrap();
            }
        }
    }
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub(crate) fn trim(&mut self) -> Result<()> {
        let mut total = 0.0;
        for bin in &self.bins {
            total += bin.count;
        }
        self.total = total;
        while self.bins.len() > self.maxbins {
            let mut min_delta = std::f64::INFINITY;
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
            let mut head = Vec::with_capacity(min_delta_index - 1);
            head.extend_from_slice(&self.bins[..min_delta_index - 1]);
            let mut tail = Vec::with_capacity(self.bins.len() - min_delta_index);
            tail.extend_from_slice(&self.bins[min_delta_index + 1..]);
            head.push(merged_bin);
            self.bins = head;
            self.bins.extend(tail);
        }
        Ok(())
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub(crate) fn trim(&mut self) -> Result<()> {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram___trim__ground_truth"]
            fn WeightedHistogram_trim__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a mut WeightedHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<WeightedHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(WeightedHistogram_trim__foreign(ser(&input_state_in)))
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
impl WeightedHistogram {
    pub(crate) fn trim__with_callees_mocked(&mut self) -> Result<()> {
        let mut total = 0.0;
        for bin in &self.bins {
            total += bin.count;
        }
        self.total = total;
        while self.bins.len() > self.maxbins {
            let mut min_delta = std::f64::INFINITY;
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
            let mut head = Vec::with_capacity(min_delta_index - 1);
            head.extend_from_slice(&self.bins[..min_delta_index - 1]);
            let mut tail = Vec::with_capacity(self.bins.len() - min_delta_index);
            tail.extend_from_slice(&self.bins[min_delta_index + 1..]);
            head.push(merged_bin);
            self.bins = head;
            self.bins.extend(tail);
        }
        Ok(())
    }
}

use crate::__synthetic::__Synth0__add;
#[cfg(not(feature = "mock"))]
impl __Synth0__add for WeightedHistogram {
    fn add(&mut self, n: f64) {
        if let Err(_) = self.trim() {
            return;
        }
        for i in 0..self.bins.len() {
            if self.bins[i].value == n {
                self.bins[i].count += 1.0;
                self.scale_down(i);
                return;
            }
            if self.bins[i].value > n {
                let mut new_bins = self.bins[..i].to_vec();
                new_bins.push(Bin { value: n, count: 1.0 });
                new_bins.extend_from_slice(&self.bins[i..]);
                self.bins = new_bins;
                self.scale_down(i);
                return;
            }
        }
        self.bins.push(Bin { value: n, count: 1.0 });
    }
}
#[cfg(feature = "mock")]
impl __Synth0__add for WeightedHistogram {
    fn add(&mut self, n: f64) {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram___add__ground_truth"]
            fn WeightedHistogram_add__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(
            &'a mut WeightedHistogram,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(
            Box<WeightedHistogram>,
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
            >(WeightedHistogram_add__foreign(ser(&params[0]), ser(&params[1])))
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
impl WeightedHistogram {
    fn add__with_callees_mocked(&mut self, n: f64) {
        if let Err(_) = self.trim() {
            return;
        }
        for i in 0..self.bins.len() {
            if self.bins[i].value == n {
                self.bins[i].count += 1.0;
                self.scale_down(i);
                return;
            }
            if self.bins[i].value > n {
                let mut new_bins = self.bins[..i].to_vec();
                new_bins.push(Bin { value: n, count: 1.0 });
                new_bins.extend_from_slice(&self.bins[i..]);
                self.bins = new_bins;
                self.scale_down(i);
                return;
            }
        }
        self.bins.push(Bin { value: n, count: 1.0 });
    }
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub fn cdf(&self, x: f64) -> f64 {
        let mut count = 0.0;
        for bin in &self.bins {
            if bin.value <= x {
                count += bin.count;
            }
        }
        count / self.total
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn cdf(&self, x: f64) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram__cdf__ground_truth"]
            fn WeightedHistogram_cdf__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(
            &'a WeightedHistogram,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(
            Box<WeightedHistogram>,
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
            >(WeightedHistogram_cdf__foreign(ser(&params[0]), ser(&params[1])))
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
impl WeightedHistogram {
    pub fn cdf__with_callees_mocked(&self, x: f64) -> f64 {
        let mut count = 0.0;
        for bin in &self.bins {
            if bin.value <= x {
                count += bin.count;
            }
        }
        count / self.total
    }
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub fn count(&self) -> f64 {
        self.total
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn count(&self) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram___count__ground_truth"]
            fn WeightedHistogram_count__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a WeightedHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<WeightedHistogram>);
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
            >(WeightedHistogram_count__foreign(ser(&input_state_in)))
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
impl WeightedHistogram {
    pub fn count__with_callees_mocked(&self) -> f64 {
        self.total
    }
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub fn mean(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for bin in &self.bins {
            sum += bin.value * bin.count;
        }
        sum / self.total
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn mean(&self) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram___mean__ground_truth"]
            fn WeightedHistogram_mean__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a WeightedHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<WeightedHistogram>);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        let input_state_in = InputStateIn(self);
        let foreign_execution = unsafe {
            de::<ForeignExecution>(WeightedHistogram_mean__foreign(ser(&input_state_in)))
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
impl WeightedHistogram {
    pub fn mean__with_callees_mocked(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for bin in &self.bins {
            sum += bin.value * bin.count;
        }
        sum / self.total
    }
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub fn quantile(&self, q: f64) -> Result<f64, anyhow::Error> {
        let count = q * self.total;
        let mut cumulative_count = 0.0;
        for bin in &self.bins {
            cumulative_count += bin.count;
            if cumulative_count >= count {
                return Ok(bin.value);
            }
        }
        Err(anyhow!("No quantile found"))
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn quantile(&self, q: f64) -> Result<f64, anyhow::Error> {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram___quantile__ground_truth"]
            fn WeightedHistogram_quantile__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(
            &'a WeightedHistogram,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(
            Box<WeightedHistogram>,
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
            >(WeightedHistogram_quantile__foreign(ser(&params[0]), ser(&params[1])))
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
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn quantile__with_callees_mocked(&self, q: f64) -> Result<f64, anyhow::Error> {
        let count = q * self.total;
        let mut cumulative_count = 0.0;
        for bin in &self.bins {
            cumulative_count += bin.count;
            if cumulative_count >= count {
                return Ok(bin.value);
            }
        }
        Err(anyhow!("No quantile found"))
    }
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub fn variance(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        let mean = self.mean();
        for bin in &self.bins {
            sum += (bin.count * (bin.value - mean) * (bin.value - mean));
        }
        sum / self.total
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn variance(&self) -> f64 {
        extern "C" {
            #[link_name = "gohistogram_weighted_histogram___variance__ground_truth"]
            fn WeightedHistogram_variance__foreign(_: JSONObject) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn<'a>(&'a WeightedHistogram);
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(Box<WeightedHistogram>);
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
            >(WeightedHistogram_variance__foreign(ser(&input_state_in)))
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
impl WeightedHistogram {
    pub fn variance__with_callees_mocked(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        let mean = self.mean();
        for bin in &self.bins {
            sum += (bin.count * (bin.value - mean) * (bin.value - mean));
        }
        sum / self.total
    }
}

#[cfg(not(feature = "mock"))]
impl WeightedHistogram {
    pub fn new(n: usize, alpha: f64) -> Result<WeightedHistogram> {
        Ok(WeightedHistogram {
            bins: Vec::with_capacity(n),
            maxbins: n,
            total: 0.0,
            alpha,
        })
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn new(n: usize, alpha: f64) -> Result<WeightedHistogram> {
        extern "C" {
            #[link_name = "gohistogram_new_weighted_histogram__ground_truth"]
            fn WeightedHistogram_new__foreign(
                _: JSONObject,
                _: JSONObject,
            ) -> JSONObject;
        }
        #[serde_as]
        #[derive(Serialize)]
        struct InputStateIn(
            usize,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputStateOut(
            usize,
            #[serde_as(as = "crate :: interoperation_utils :: MyFloat64")]
            f64,
        );
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState(WeightedHistogram);
        let input_state_in = InputStateIn(n, alpha);
        let input_state_serialized = serde_json::to_value(input_state_in).unwrap();
        let serde_json::Value::Array(params) = input_state_serialized else {
            panic!("expect multiple input arguments")
        };
        let foreign_execution = unsafe {
            de::<
                ForeignExecution,
            >(WeightedHistogram_new__foreign(ser(&params[0]), ser(&params[1])))
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
            return Ok(output);
        } else {
            return Err(anyhow!("execution failure"));
        }
    }
}
#[cfg(feature = "mock")]
impl WeightedHistogram {
    pub fn new__with_callees_mocked(n: usize, alpha: f64) -> Result<WeightedHistogram> {
        Ok(WeightedHistogram {
            bins: Vec::with_capacity(n),
            maxbins: n,
            total: 0.0,
            alpha,
        })
    }
}

