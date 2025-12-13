#![allow(unused_imports)]
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use crate::regression::Coordinate;
use crate::describe::Description;
use crate::data::Float64Data;
use crate::outlier::Outliers;
use crate::quartile::Quartiles;
use crate::regression::Series;
use crate::correlation::auto_correlation;
use crate::distances::chebyshev_distance;
use crate::correlation::correlation;
use crate::cumulative_sum::cumulative_sum;
use crate::describe::describe;
use crate::entropy::entropy;
use crate::distances::euclidean_distance;
use crate::geometric_distribution::exp_geom;
use crate::regression::exponential_regression;
use crate::mean::geometric_mean;
use crate::mean::harmonic_mean;
use crate::quartile::inter_quartile_range;
use crate::regression::linear_regression;
use crate::regression::logarithmic_regression;
use crate::distances::manhattan_distance;
use crate::max::max;
use crate::mean::mean;
use crate::median::median;
use crate::deviation::median_absolute_deviation_population;
use crate::quartile::midhinge;
use crate::min::min;
use crate::distances::minkowski_distance;
use crate::mode::mode;
use crate::percentile::percentile;
use crate::percentile::percentile_nearest_rank;
use crate::variance::population_variance;
use crate::geometric_distribution::prob_geom;
use crate::quartile::quartile;
use crate::outlier::quartile_outliers;
use crate::round::round;
use crate::sample::sample;
use crate::variance::sample_variance;
use crate::sigmoid::sigmoid;
use crate::softmax::soft_max;
use crate::deviation::standard_deviation_population;
use crate::deviation::standard_deviation_sample;
use crate::sum::sum;
use crate::quartile::trimean;
use crate::geometric_distribution::var_geom;
//Translated from: github.com/montanaflynn/stats.Example
pub fn example() -> Result<()> {
    // d := LoadRawData([]interface{}{1.1, "2", 3.0, 4, "5"})
    // d := LoadRawData([]int{1, 2, 3, 4, 5})
    let mut d = Float64Data::from(vec![]);
    for v in &[1, 2, 3, 4, 5] {
        d.0.push(*v as f64);
    }

    let _ = min(d.clone())?;
    // Output: 1.1

    let _ = max(d.clone())?;
    // Output: 5

    let _ = sum(Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    // Output: 6.6

    let _ = cumulative_sum(&Float64Data::from(vec![1.1, 2.2, 3.3]))?;

    let _ = mean(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    // Output: 3

    let _ = median(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]))?;
    // Output: 4

    let _ = mode(Float64Data::from(vec![5.0, 5.0, 3.0, 3.0, 4.0, 2.0, 1.0]))?;
    // Output: [5 3]

    let _ = population_variance(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    // Output: 2

    let _ = sample_variance(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    // Output: 2.5

    let _ = median_absolute_deviation_population(Float64Data::from(vec![1.0, 2.0, 3.0]))?;
    // Output: 1

    let _ = standard_deviation_population(Float64Data::from(vec![1.0, 2.0, 3.0]))?;
    // Output: 0.816496580927726

    let _ = standard_deviation_sample(Float64Data::from(vec![1.0, 2.0, 3.0]))?;
    // Output: 1

    let _ = percentile(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 75.0)?;
    // Output: 4

    let _ = percentile_nearest_rank(Float64Data::from(vec![35.0, 20.0, 15.0, 40.0, 50.0]), 75.0)?;
    // Output: 40

    let c = Series::from(vec![
        Coordinate { x: 1.0, y: 2.3 },
        Coordinate { x: 2.0, y: 3.3 },
        Coordinate { x: 3.0, y: 3.7 },
        Coordinate { x: 4.0, y: 4.3 },
        Coordinate { x: 5.0, y: 5.3 },
    ]);

    let _ = linear_regression(&c)?;
    // Output: [{1 2.3800000000000026} {2 3.0800000000000014} {3 3.7800000000000002} {4 4.479999999999999} {5 5.179999999999998}]

    let _ = exponential_regression(&c)?;
    // Output: [{1 2.5150181024736638} {2 3.032084111136781} {3 3.6554544271334493} {4 4.406984298281804} {5 5.313022222665875}]

    let _ = logarithmic_regression(c)?;
    // Output: [{1 2.1520822363811702} {2 3.3305559222492214} {3 4.019918836568674} {4 4.509029608117273} {5 4.888413396683663}]

    let _ = sample(&Float64Data::from(vec![0.1, 0.2, 0.3, 0.4]), 3, false)?;
    // Output: [0.2,0.4,0.3]

    let _ = sample(&Float64Data::from(vec![0.1, 0.2, 0.3, 0.4]), 10, true)?;
    // Output: [0.2,0.2,0.4,0.1,0.2,0.4,0.3,0.2,0.2,0.1]

    let _ = quartile(Float64Data::from(vec![7.0, 15.0, 36.0, 39.0, 40.0, 41.0]))?;
    // Output: {15 37.5 40}

    let _ = inter_quartile_range(Float64Data::from(vec![102.0, 104.0, 105.0, 107.0, 108.0, 109.0, 110.0, 112.0, 115.0, 116.0, 118.0]))?;
    // Output: 10

    let _ = midhinge(Float64Data::from(vec![1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0, 9.0, 10.0, 11.0, 12.0, 13.0]))?;
    // Output: 7.5

    let _ = trimean(Float64Data::from(vec![1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0, 9.0, 10.0, 11.0, 12.0, 13.0]))?;
    // Output: 7.25

    let _ = quartile_outliers(Float64Data::from(vec![-1000.0, 1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 8.0, 15.0, 18.0, 100.0]))?;
    // Output:  {Mild:[15 18] Extreme:[-1000 100]}

    let _ = geometric_mean(&Float64Data::from(vec![10.0, 51.2, 8.0]))?;
    // Output: 15.999999999999991

    let _ = harmonic_mean(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]))?;
    // Output: 2.18978102189781

    let _ = round(2.18978102189781, 3)?;
    // Output: 2.189

    let _ = chebyshev_distance(Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]))?;
    // Output: 6

    let _ = manhattan_distance(Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]))?;
    // Output: 24

    let _ = euclidean_distance(Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]))?;
    // Output: 10.583005244258363

    let _ = minkowski_distance(Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]), 1.0)?;
    // Output: 24

    let _ = minkowski_distance(Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]), 2.0)?;
    // Output: 10.583005244258363

    let _ = minkowski_distance(Float64Data::from(vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]), Float64Data::from(vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0]), 99.0)?;
    // Output: 6

    let _ = correlation(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), Float64Data::from(vec![1.0, 2.0, 3.0, 5.0, 6.0]))?;
    // Output: 0.9912407071619302

    let _ = auto_correlation(Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 1)?;
    // Output: 0.4

    let _ = sigmoid(Float64Data::from(vec![3.0, 1.0, 2.1]))?;
    // Output: [0.9525741268224334 0.7310585786300049 0.8909031788043871]

    let _ = soft_max(Float64Data::from(vec![3.0, 1.0, 0.2]))?;
    // Output: [0.8360188027814407 0.11314284146556013 0.05083835575299916]

    let _ = entropy(Float64Data::from(vec![1.1, 2.2, 3.3]))?;
    // Output: 1.0114042647073518

    let p = 0.5;
    let begin = 1;
    let end = 2;
    let _ = prob_geom(begin, end, p)?;
    // Output: 0.25

    let prob1 = 0.5;
    let _ = exp_geom(prob1)?;
    // Output:

    let prob2 = 0.5;
    let _ = var_geom(prob2)?;
    // Output: 2

    let _ = describe(Float64Data::from(vec![1.0, 2.0, 3.0]), true, Some(&[25.0, 50.0, 75.0]))?;

    Ok(())
}

