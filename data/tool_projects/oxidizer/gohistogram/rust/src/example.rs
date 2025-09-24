use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::__synthetic::__Synth0__add;
use crate::numerichistogram::NumericHistogram;
use crate::weightedhistogram::WeightedHistogram;
#[cfg(not(feature = "mock"))]
pub fn example() -> Result<()> {
    let mut h = NumericHistogram::new_histogram(160);
    h.add(160.0);
    let _ = h.quantile(0.25);
    let _ = h.cdf(18.0);
    let _ = h.count();
    let _ = h.mean();
    let _ = h.variance();
    let mut w = WeightedHistogram::new(160, 1.0)?;
    w.add(160.0);
    let _ = w.quantile(0.25)?;
    let _ = w.cdf(18.0);
    let _ = w.count();
    let _ = w.mean();
    let _ = w.variance();
    Ok(())
}
#[cfg(feature = "mock")]
pub fn example() -> Result<()> {
    extern "C" {
        #[link_name = "gohistogram_example__ground_truth"]
        fn example__foreign() -> JSONObject;
    }
    #[serde_as]
    #[derive(Serialize)]
    struct InputStateIn();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct InputStateOut();
    #[serde_as]
    #[derive(Serialize, Deserialize)]
    struct OutputState;
    let input_state_in = InputStateIn();
    let foreign_execution = unsafe { de::<ForeignExecution>(example__foreign()) };
    if foreign_execution.execution_success {
        assert_eq!(foreign_execution.input_modifications.len(), 0usize);
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
        let output = ();
        return Ok(output);
    } else {
        return Err(anyhow!("execution failure"));
    }
}
#[cfg(feature = "mock")]
pub fn example__with_callees_mocked() -> Result<()> {
    let mut h = NumericHistogram::new_histogram(160);
    h.add(160.0);
    let _ = h.quantile(0.25);
    let _ = h.cdf(18.0);
    let _ = h.count();
    let _ = h.mean();
    let _ = h.variance();
    let mut w = WeightedHistogram::new(160, 1.0)?;
    w.add(160.0);
    let _ = w.quantile(0.25)?;
    let _ = w.cdf(18.0);
    let _ = w.count();
    let _ = w.mean();
    let _ = w.variance();
    Ok(())
}

