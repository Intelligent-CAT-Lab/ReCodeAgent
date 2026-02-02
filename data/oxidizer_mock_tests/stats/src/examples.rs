use crate::careful::*;use serde::{Deserialize, Serialize};use serde_with::serde_as;use arbitrary::Arbitrary;use anyhow::Context;use anyhow::Error;use anyhow::Result;use anyhow::anyhow;

use crate::quartile::trimean;
use crate::min::min;
use crate::regression::exponential_regression;
use crate::correlation::correlation;
use crate::distances::manhattan_distance;
use crate::describe::describe;
use crate::sigmoid::sigmoid;
use crate::regression::linear_regression;
use crate::variance::sample_variance;
use crate::mean::geometric_mean;
use crate::describe::Description;
use crate::sample::sample;
use crate::quartile::midhinge;
use crate::geometric_distribution::prob_geom;
use crate::deviation::standard_deviation_population;
use crate::mode::mode;
use crate::percentile::percentile;
use crate::variance::population_variance;
use crate::distances::chebyshev_distance;
use crate::distances::minkowski_distance;
use crate::mean::mean;
use crate::outlier::quartile_outliers;
use crate::mean::harmonic_mean;
use crate::percentile::percentile_nearest_rank;
use crate::distances::euclidean_distance;
use crate::quartile::Quartiles;
use crate::max::max;
use crate::quartile::quartile;
use crate::regression::Coordinate;
use crate::deviation::standard_deviation_sample;
use crate::regression::Series;
use crate::sum::sum;
use crate::quartile::inter_quartile_range;
use crate::softmax::soft_max;
use crate::cumulative_sum::cumulative_sum;
use crate::data::Float64Data;
use crate::entropy::entropy;
use crate::median::median;
use crate::deviation::median_absolute_deviation_population;
use crate::outlier::Outliers;
use crate::geometric_distribution::var_geom;
use crate::geometric_distribution::exp_geom;
use crate::round::round;
use crate::correlation::auto_correlation;
use crate::regression::logarithmic_regression;
#[cfg(not(feature = "mock"))]
pub fn example() -> Result<()> {
    let mut d = Float64Data::from(vec![]);
    for v in &[1, 2, 3, 4, 5] {
        d.0.push(*v as f64);
    }
    let _ = min(d.clone())?;
    let _ = max(d.clone())?;
    let _ = sum(Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    let _ = cumulative_sum(&Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    let _ = mean(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = median(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]))?;
    let _ = mode(Float64Data::from(vec![5.0, 5.0, 3.0, 3.0, 4.0, 2.0, 1.0]))?;
    let _ = population_variance(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = sample_variance(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = median_absolute_deviation_population(
        Float64Data::from(vec![1.0, 2.0, 3.0]),
    )?;
    let _ = standard_deviation_population(Float64Data::from(vec![1.0, 2.0, 3.0]))?;
    let _ = standard_deviation_sample(Float64Data::from(vec![1.0, 2.0, 3.0]))?;
    let _ = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 75.0)?;
    let _ = percentile_nearest_rank(
        Float64Data::from(vec![35.0, 20.0, 15.0, 40.0, 50.0]),
        75.0,
    )?;
    let c = Series::from(
        vec![
            Coordinate { x : 1.0, y : 2.3 }, Coordinate { x : 2.0, y : 3.3 }, Coordinate
            { x : 3.0, y : 3.7 }, Coordinate { x : 4.0, y : 4.3 }, Coordinate { x : 5.0,
            y : 5.3 },
        ],
    );
    let _ = linear_regression(&c)?;
    let _ = exponential_regression(&c)?;
    let _ = logarithmic_regression(c)?;
    let _ = sample(&Float64Data::from(vec![0.1, 0.2, 0.3, 0.4]), 3, false)?;
    let _ = sample(&Float64Data::from(vec![0.1, 0.2, 0.3, 0.4]), 10, true)?;
    let _ = quartile(Float64Data::from(vec![7.0, 15.0, 36.0, 39.0, 40.0, 41.0]))?;
    let _ = inter_quartile_range(
        Float64Data::from(
            vec![
                102.0, 104.0, 105.0, 107.0, 108.0, 109.0, 110.0, 112.0, 115.0, 116.0,
                118.0
            ],
        ),
    )?;
    let _ = midhinge(
        Float64Data::from(
            vec![
                1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0,
                9.0, 10.0, 11.0, 12.0, 13.0
            ],
        ),
    )?;
    let _ = trimean(
        Float64Data::from(
            vec![
                1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0,
                9.0, 10.0, 11.0, 12.0, 13.0
            ],
        ),
    )?;
    let _ = quartile_outliers(
        Float64Data::from(
            vec![
                - 1000.0, 1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 8.0, 15.0, 18.0,
                100.0
            ],
        ),
    )?;
    let _ = geometric_mean(&Float64Data::from(vec![10.0, 51.2, 8.0]))?;
    let _ = harmonic_mean(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = round(2.18978102189781, 3)?;
    let _ = chebyshev_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
    )?;
    let _ = manhattan_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
    )?;
    let _ = euclidean_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
    )?;
    let _ = minkowski_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
        1.0,
    )?;
    let _ = minkowski_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
        2.0,
    )?;
    let _ = minkowski_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
        99.0,
    )?;
    let _ = correlation(
        Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
        Float64Data::from(vec![1.0, 2.0, 3.0, 5.0, 6.0]),
    )?;
    let _ = auto_correlation(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 1)?;
    let _ = sigmoid(Float64Data::from(vec![3.0, 1.0, 2.1]))?;
    let _ = soft_max(Float64Data::from(vec![3.0, 1.0, 0.2]))?;
    let _ = entropy(Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    let p = 0.5;
    let begin = 1;
    let end = 2;
    let _ = prob_geom(begin, end, p)?;
    let prob1 = 0.5;
    let _ = exp_geom(prob1)?;
    let prob2 = 0.5;
    let _ = var_geom(prob2)?;
    let _ = describe(
        Float64Data::from(vec![1.0, 2.0, 3.0]),
        true,
        Some(&[25.0, 50.0, 75.0]),
    )?;
    Ok(())
}
#[cfg(feature = "mock")]
pub fn example() -> Result<()> {
    extern "C" {
        #[link_name = "stats_example__ground_truth"]
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
    let mut d = Float64Data::from(vec![]);
    for v in &[1, 2, 3, 4, 5] {
        d.0.push(*v as f64);
    }
    let _ = min(d.clone())?;
    let _ = max(d.clone())?;
    let _ = sum(Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    let _ = cumulative_sum(&Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    let _ = mean(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = median(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]))?;
    let _ = mode(Float64Data::from(vec![5.0, 5.0, 3.0, 3.0, 4.0, 2.0, 1.0]))?;
    let _ = population_variance(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = sample_variance(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = median_absolute_deviation_population(
        Float64Data::from(vec![1.0, 2.0, 3.0]),
    )?;
    let _ = standard_deviation_population(Float64Data::from(vec![1.0, 2.0, 3.0]))?;
    let _ = standard_deviation_sample(Float64Data::from(vec![1.0, 2.0, 3.0]))?;
    let _ = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 75.0)?;
    let _ = percentile_nearest_rank(
        Float64Data::from(vec![35.0, 20.0, 15.0, 40.0, 50.0]),
        75.0,
    )?;
    let c = Series::from(
        vec![
            Coordinate { x : 1.0, y : 2.3 }, Coordinate { x : 2.0, y : 3.3 }, Coordinate
            { x : 3.0, y : 3.7 }, Coordinate { x : 4.0, y : 4.3 }, Coordinate { x : 5.0,
            y : 5.3 },
        ],
    );
    let _ = linear_regression(&c)?;
    let _ = exponential_regression(&c)?;
    let _ = logarithmic_regression(c)?;
    let _ = sample(&Float64Data::from(vec![0.1, 0.2, 0.3, 0.4]), 3, false)?;
    let _ = sample(&Float64Data::from(vec![0.1, 0.2, 0.3, 0.4]), 10, true)?;
    let _ = quartile(Float64Data::from(vec![7.0, 15.0, 36.0, 39.0, 40.0, 41.0]))?;
    let _ = inter_quartile_range(
        Float64Data::from(
            vec![
                102.0, 104.0, 105.0, 107.0, 108.0, 109.0, 110.0, 112.0, 115.0, 116.0,
                118.0
            ],
        ),
    )?;
    let _ = midhinge(
        Float64Data::from(
            vec![
                1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0,
                9.0, 10.0, 11.0, 12.0, 13.0
            ],
        ),
    )?;
    let _ = trimean(
        Float64Data::from(
            vec![
                1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0,
                9.0, 10.0, 11.0, 12.0, 13.0
            ],
        ),
    )?;
    let _ = quartile_outliers(
        Float64Data::from(
            vec![
                - 1000.0, 1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 8.0, 15.0, 18.0,
                100.0
            ],
        ),
    )?;
    let _ = geometric_mean(&Float64Data::from(vec![10.0, 51.2, 8.0]))?;
    let _ = harmonic_mean(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    let _ = round(2.18978102189781, 3)?;
    let _ = chebyshev_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
    )?;
    let _ = manhattan_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
    )?;
    let _ = euclidean_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
    )?;
    let _ = minkowski_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
        1.0,
    )?;
    let _ = minkowski_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
        2.0,
    )?;
    let _ = minkowski_distance(
        Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
        Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]),
        99.0,
    )?;
    let _ = correlation(
        Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
        Float64Data::from(vec![1.0, 2.0, 3.0, 5.0, 6.0]),
    )?;
    let _ = auto_correlation(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 1)?;
    let _ = sigmoid(Float64Data::from(vec![3.0, 1.0, 2.1]))?;
    let _ = soft_max(Float64Data::from(vec![3.0, 1.0, 0.2]))?;
    let _ = entropy(Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    let p = 0.5;
    let begin = 1;
    let end = 2;
    let _ = prob_geom(begin, end, p)?;
    let prob1 = 0.5;
    let _ = exp_geom(prob1)?;
    let prob2 = 0.5;
    let _ = var_geom(prob2)?;
    let _ = describe(
        Float64Data::from(vec![1.0, 2.0, 3.0]),
        true,
        Some(&[25.0, 50.0, 75.0]),
    )?;
    Ok(())
}
#[cfg(test)]
mod stats_example_harness {
    use super::*;
    #[test]
    fn example__unit_test() {
        let unittests_file: std::fs::File = std::fs::File::open(
                "./exec-snapshots/github.com-montanaflynn-stats.Example.json",
            )
            .unwrap();
        let unittests_reader = std::io::BufReader::new(unittests_file);
        let unittests: Vec<ExecutionData> = serde_json::from_reader(unittests_reader)
            .unwrap();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
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
                    { (example__with_callees_mocked()).unwrap() }
                    #[cfg(not(feature = "mock"))] { (example()).unwrap() }
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
                }
                Err(_) => {
                    assert!(! execution.result.execution_success);
                }
            }
        }
    }
    #[test]
    fn example__signature_check() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct InputState();
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct OutputState;
        let Ok(unittests_file) = std::fs::File::open(
            "./exec-snapshots/github.com-montanaflynn-stats.Example.json",
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
