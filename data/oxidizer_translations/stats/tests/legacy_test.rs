use wspace::data::Float64Data;
use wspace::regression::Coordinate;
use wspace::legacy::{VarP, VarS, StdDevP, StdDevS, LinReg, ExpReg, LogReg};

// Create working sample data to test if the legacy
// functions cause a runtime crash or return an error
#[test]
fn TestLegacy() {
    // Slice of data
    let s = Float64Data(vec![-10.0, -10.001, 5.0, 1.1, 2.0, 3.0, 4.20, 5.0]);

    // Slice of coordinates
    let d = vec![
        Coordinate { x: 1.0, y: 2.3 },
        Coordinate { x: 2.0, y: 3.3 },
        Coordinate { x: 3.0, y: 3.7 },
        Coordinate { x: 4.0, y: 4.3 },
        Coordinate { x: 5.0, y: 5.3 },
    ];

    // VarP rename compatibility
    match VarP(&s) {
        Ok(_) => {},
        Err(_) => panic!("VarP not successfully returning PopulationVariance."),
    }

    // VarS rename compatibility
    match VarS(&s) {
        Ok(_) => {},
        Err(_) => panic!("VarS not successfully returning SampleVariance."),
    }

    // StdDevP rename compatibility
    match StdDevP(&s) {
        Ok(_) => {},
        Err(_) => panic!("StdDevP not successfully returning StandardDeviationPopulation."),
    }

    // StdDevS rename compatibility
    match StdDevS(&s) {
        Ok(_) => {},
        Err(_) => panic!("StdDevS not successfully returning StandardDeviationSample."),
    }

    // LinReg rename compatibility
    match LinReg(&d) {
        Ok(_) => {},
        Err(_) => panic!("LinReg not successfully returning LinearRegression."),
    }

    // ExpReg rename compatibility
    match ExpReg(&d) {
        Ok(_) => {},
        Err(_) => panic!("ExpReg not successfully returning ExponentialRegression."),
    }

    // LogReg rename compatibility
    match LogReg(&d) {
        Ok(_) => {},
        Err(_) => panic!("LogReg not successfully returning LogarithmicRegression."),
    }
}
