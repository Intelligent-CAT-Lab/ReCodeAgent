use wspace::geometric_distribution::{exp_geom, prob_geom, var_geom};

#[test]
fn ExampleProbGeom() {
    let p = 0.5;
    let a = 1;
    let b = 2;
    let chance = prob_geom(a, b, p).unwrap();
    assert_eq!(chance, 0.25);
}

#[test]
fn TestProbGeomLarge() {
    let p = 0.5;
    let a = 1;
    let b = 10000;
    let chance = prob_geom(a, b, p);
    assert!(chance.is_ok(), "Returned an error");
    let chance = chance.unwrap();
    assert_eq!(chance, 0.5, "ProbGeom({}, {}, {:.01}) => {:.1} != {:.1}", a, b, p, chance, 0.5);
}

#[test]
fn TestErrBoundsProbGeom() {
    let p = 0.5;
    let a = -1;
    let b = 4;
    let chance = prob_geom(a, b, p);
    assert!(chance.is_err(), "Did not return an error when expected");
    let chance = chance.unwrap_err();
    // When error occurs, we check that an error was returned (no NaN check in Rust version)
}

#[test]
fn ExampleExpGeom() {
    let p = 0.5;
    let exp = exp_geom(p).unwrap();
    assert_eq!(exp, 2.0);
}

#[test]
fn TestExpGeom() {
    let p = 0.5;
    let exp = exp_geom(p);
    assert!(exp.is_ok(), "Returned an error when not expected");
    let exp = exp.unwrap();
    assert_eq!(exp, 2.0, "ExpGeom({:.01}) => {:.1} != {:.1}", p, exp, 2.0);
}

#[test]
fn TestErrExpGeom() {
    let p = -1.0;
    let exp = exp_geom(p);
    assert!(exp.is_err(), "Did not return an error");
    // When error occurs in Rust, we don't return NaN, we return Err
}

#[test]
fn ExampleVarGeom() {
    let p = 0.5;
    let vari = var_geom(p).unwrap();
    assert_eq!(vari, 2.0);
}

#[test]
fn TestVarGeom() {
    let p = 0.25;
    let vari = var_geom(p);
    assert!(vari.is_ok(), "Returned an error when not expected");
    let vari = vari.unwrap();
    assert_eq!(vari, 12.0, "VarGeom({:.01}) => {:.1} != {:.1}", p, vari, 12.0);
}

#[test]
fn TestErrVarGeom() {
    let p = -1.0;
    let vari = var_geom(p);
    assert!(vari.is_err(), "Did not return an error");
    // When error occurs in Rust, we don't return NaN, we return Err
}
