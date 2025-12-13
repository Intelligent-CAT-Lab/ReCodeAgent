// Test suite for variance module functions
use wspace::variance::{population_variance, sample_variance, covariance, covariance_population};
use wspace::round::round;
use wspace::errors::ERR_EMPTY_INPUT;
use wspace::data::Float64Data;

#[test]
fn TestVariance() {
    let input = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let result = population_variance(input);
    if result.is_err() {
        panic!("Returned an error");
    }
}

#[test]
fn TestPopulationVariance() {
    let empty_input = Float64Data::from(vec![]);
    let e = population_variance(empty_input);
    if let Ok(val) = e {
        if !val.is_nan() {
            panic!("{:.1} != NaN", val);
        }
    }
    if let Err(err) = e {
        if err.to_string() != ERR_EMPTY_INPUT.to_string() {
            panic!("{:?} != {:?}", err, ERR_EMPTY_INPUT);
        }
    }

    let input = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let pv = population_variance(input).unwrap();
    let a = round(pv, 1);
    if a.is_err() {
        panic!("Returned an error");
    }
    let a_val = a.unwrap();
    if a_val != 0.7 {
        panic!("{:.1} != {:.1}", a_val, 0.7);
    }
}

#[test]
fn TestSampleVariance() {
    let empty_input = Float64Data::from(vec![]);
    let m = sample_variance(empty_input);
    if let Ok(val) = m {
        if !val.is_nan() {
            panic!("{:.1} != NaN", val);
        }
    }
    if let Err(err) = m {
        if err.to_string() != ERR_EMPTY_INPUT.to_string() {
            panic!("{:?} != {:?}", err, ERR_EMPTY_INPUT);
        }
    }

    let input = Float64Data::from(vec![1.0, 2.0, 3.0]);
    let m = sample_variance(input).unwrap();
    if m != 1.0 {
        panic!("{:.1} != {:.1}", m, 1.0);
    }
}

#[test]
fn TestCovariance() {
    let s1 = Float64Data::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let s2 = Float64Data::from(vec![10.0, -51.2, 8.0]);
    let s3 = Float64Data::from(vec![1.0, 2.0, 3.0, 5.0, 6.0]);
    let s4 = Float64Data::from(vec![]);

    let result = covariance(s1.clone(), s2.clone());
    if result.is_ok() {
        panic!("Mismatched slice lengths should have returned an error");
    }

    let a = covariance(s1.clone(), s3.clone());
    if a.is_err() {
        panic!("Should not have returned an error");
    }

    let a_val = a.unwrap();
    if a_val != 3.2499999999999996 {
        panic!("Covariance {:?} != {:?}", a_val, 3.2499999999999996);
    }

    let result = covariance(s1.clone(), s4.clone());
    if result.is_ok() {
        panic!("Empty slice should have returned an error");
    }
}

#[test]
fn TestCovariancePopulation() {
    let s1 = Float64Data::from(vec![1.0, 2.0, 3.5, 3.7, 8.0, 12.0]);
    let s2 = Float64Data::from(vec![10.0, -51.2, 8.0]);
    let s3 = Float64Data::from(vec![0.5, 1.0, 2.1, 3.4, 3.4, 4.0]);
    let s4 = Float64Data::from(vec![]);

    let result = covariance_population(s1.clone(), s2.clone());
    if result.is_ok() {
        panic!("Mismatched slice lengths should have returned an error");
    }

    let a = covariance_population(s1.clone(), s3);
    if a.is_err() {
        panic!("Should not have returned an error");
    }

    let a_val = a.unwrap();
    if a_val != 4.191666666666666 {
        panic!("CovariancePopulation {:?} != {:?}", a_val, 4.191666666666666);
    }

    let result = covariance_population(s1.clone(), s4.clone());
    if result.is_ok() {
        panic!("Empty slice should have returned an error");
    }
}
