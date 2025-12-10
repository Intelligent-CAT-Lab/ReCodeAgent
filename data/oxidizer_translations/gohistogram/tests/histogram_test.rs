// Integration test for gohistogram

mod sample_data_test;

use wspace::numerichistogram::NumericHistogram;
use wspace::weightedhistogram::WeightedHistogram;
// Note: The __Synth0__add trait provides the `add` method but is in a private module.
// If this import fails, you may need to make `__synthetic` module public in lib.rs
use wspace::__synthetic::__Synth0__add;

use crate::sample_data_test::TEST_DATA;

// Helper function
fn approx(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.2
}

#[test]
fn test_histogram() {
    let mut h = NumericHistogram::new_histogram(160);
    for &val in &TEST_DATA {
        h.add(val as f64);
    }

    assert_eq!(h.count(), 14999.0, "Expected h.count() to be 14999, got {}", h.count());

    let first_q = h.quantile(0.25);
    assert!(approx(first_q, 14.0), "Expected 25th percentile to be {}, got {}", 14, first_q);

    let median = h.quantile(0.5);
    assert!(approx(median, 18.0), "Expected 50th percentile to be {}, got {}", 18, median);

    let third_q = h.quantile(0.75);
    assert!(approx(third_q, 22.0), "Expected 75th percentile to be {}, got {}", 22, third_q);

    let cdf = h.cdf(18.0);
    assert!(approx(cdf, 0.5), "Expected CDF(median) to be {}, got {}", 0.5, cdf);

    let cdf = h.cdf(22.0);
    assert!(approx(cdf, 0.75), "Expected CDF(3rd quartile) to be {}, got {}", 0.75, cdf);
}

#[test]
fn test_weighted_histogram() {
    let mut h = WeightedHistogram::new(160, 1.0).unwrap();
    for &val in &TEST_DATA {
        h.add(val as f64);
    }

    let first_q = h.quantile(0.25).unwrap();
    assert!(approx(first_q, 14.0), "Expected 25th percentile to be {}, got {}", 14, first_q);

    let median = h.quantile(0.5).unwrap();
    assert!(approx(median, 18.0), "Expected 50th percentile to be {}, got {}", 18, median);

    let third_q = h.quantile(0.75).unwrap();
    assert!(approx(third_q, 22.0), "Expected 75th percentile to be {}, got {}", 22, third_q);

    let cdf = h.cdf(18.0);
    assert!(approx(cdf, 0.5), "Expected CDF(median) to be {}, got {}", 0.5, cdf);

    let cdf = h.cdf(22.0);
    assert!(approx(cdf, 0.75), "Expected CDF(3rd quartile) to be {}, got {}", 0.75, cdf);
}
