use Math_Library_in_C::castom_math;

fn assert_approx_eq(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() < tol, "Expected {} to be approximately equal to {}", a, b);
}

#[test]
fn castom_test_abs() {
    for i in -1000i32..=1000 {
        assert_eq!(i.abs(), castom_math::castom_abs(i));
    }
}

#[test]
fn castom_test_fabs() {
    assert_approx_eq(15.5_f64.abs(), castom_math::castom_fabs(15.5), castom_math::EPS_6);
    assert_approx_eq((-15.3_f64).abs(), castom_math::castom_fabs(-15.3), castom_math::EPS_6);
    assert_approx_eq(0.0_f64.abs(), castom_math::castom_fabs(0.0), castom_math::EPS_6);
    assert_approx_eq(0.000004_f64.abs(), castom_math::castom_fabs(0.000004), castom_math::EPS_6);
    assert_approx_eq((-0.000004_f64).abs(), castom_math::castom_fabs(-0.000004), castom_math::EPS_6);
    for i in (-1000..=1000).map(|x| x as f64 * 0.01) {
        assert_approx_eq(i.abs(), castom_math::castom_fabs(i), castom_math::EPS_6);
    }
}

#[test]
fn scastom_test_pow() {
    for i in (0..=10).map(|x| x as f64 * 0.5) {
        for j in (1..=24).map(|x| x as f64 * 0.4) {
            assert_approx_eq(i.powf(j), castom_math::castom_pow(i, j), castom_math::EPS_6);
        }
    }
    assert_approx_eq(1_f64.powf(1.0), castom_math::castom_pow(1.0, 1.0), castom_math::EPS_6);
    assert_approx_eq(3_f64.powf(0.0), castom_math::castom_pow(3.0, 0.0), castom_math::EPS_6);
    assert_approx_eq(3_f64.powf(2.0), castom_math::castom_pow(3.0, 2.0), castom_math::EPS_6);
    assert_approx_eq(3.1_f64.powf(4.0), castom_math::castom_pow(3.1, 4.0), castom_math::EPS_6);
    assert_approx_eq(3.1_f64.powf(4.2), castom_math::castom_pow(3.1, 4.2), castom_math::EPS_6);
    assert_approx_eq(75_f64.powf(16.0), castom_math::castom_pow(75.0, 16.0), castom_math::EPS_6);
    assert_approx_eq(35.1_f64.powf(7.0), castom_math::castom_pow(35.1, 7.0), castom_math::EPS_6);
    assert_approx_eq(5.1_f64.powf(4.51), castom_math::castom_pow(5.1, 4.51), castom_math::EPS_6);
    assert_approx_eq(5.1_f64.powf(1.51), castom_math::castom_pow(5.1, 1.51), castom_math::EPS_6);
    assert_approx_eq(5.1_f64.powf(0.511), castom_math::castom_pow(5.1, 0.511), castom_math::EPS_6);
    assert_approx_eq(0_f64.powf(0.511), castom_math::castom_pow(0.0, 0.511), castom_math::EPS_6);
    assert_approx_eq(1.5_f64.powf(-100.0), castom_math::castom_pow(1.5, -100.0), castom_math::EPS_6);
    assert_approx_eq((-1.5_f64).powf(8.0), castom_math::castom_pow(-1.5, 8.0), castom_math::EPS_6);
    assert_approx_eq((-100.0_f64).powf(2.0), castom_math::castom_pow(-100.0, 2.0), castom_math::EPS_6);
    assert_approx_eq(0_f64.powf(0.0), castom_math::castom_pow(0.0, 0.0), castom_math::EPS_6);
    assert_approx_eq(2.3_f64.powf(300.0), castom_math::castom_pow(2.3, 300.0), castom_math::EPS_6);
}

#[test]
fn castom_test_log() {
    for i in (-1000..0).map(|x| x as f64 * 0.01) {
        assert!(castom_math::castom_log(i).is_nan());
    }
    for i in (1..=10000).map(|x| x as f64 * 0.1) {
        assert_approx_eq(i.ln(), castom_math::castom_log(i), castom_math::EPS_6);
    }
    assert_approx_eq(1_f64.ln(), castom_math::castom_log(1.0), castom_math::EPS_6);
    assert_approx_eq(8_f64.ln(), castom_math::castom_log(8.0), castom_math::EPS_6);
    assert_approx_eq(0.1_f64.ln(), castom_math::castom_log(0.1), castom_math::EPS_6);
    assert_approx_eq(500_f64.ln(), castom_math::castom_log(500.0), castom_math::EPS_6);
    assert_approx_eq(1_000_000_f64.ln(), castom_math::castom_log(1_000_000.0), castom_math::EPS_6);
    assert_approx_eq(0.0000001_f64.ln(), castom_math::castom_log(0.0000001), castom_math::EPS_6);
    assert_approx_eq(1.5_f64.ln(), castom_math::castom_log(1.5), castom_math::EPS_6);
    assert_approx_eq(2_f64.ln(), castom_math::castom_log(2.0), castom_math::EPS_6);
}

#[test]
fn castom_test_exp() {
    for j in (-200..=800).map(|x| x as f64 * 0.1) {
        assert_approx_eq(j.exp(), castom_math::castom_exp(j), castom_math::EPS_6);
    }
    assert_approx_eq(5_f64.exp(), castom_math::castom_exp(5.0), castom_math::EPS_6);
    assert_approx_eq(19_f64.exp(), castom_math::castom_exp(19.0), castom_math::EPS_6);
    assert_approx_eq((-5_f64).exp(), castom_math::castom_exp(-5.0), castom_math::EPS_6);
    assert_approx_eq((-15_f64).exp(), castom_math::castom_exp(-15.0), castom_math::EPS_6);
    assert_approx_eq(0.0000001_f64.exp(), castom_math::castom_exp(0.0000001), castom_math::EPS_6);
    assert_approx_eq(0_f64.exp(), castom_math::castom_exp(0.0), castom_math::EPS_6);
    assert_approx_eq((-20_f64).exp(), castom_math::castom_exp(-20.0), castom_math::EPS_6);
    assert_approx_eq(50_f64.exp(), castom_math::castom_exp(50.0), castom_math::EPS_6);
    assert_approx_eq(1_f64.exp(), castom_math::castom_exp(1.0), castom_math::EPS_6);
}

#[test]
fn castom_test_sqrt() {
    for j in (1..=2401).map(|x| x as f64 * 0.05) {
        assert_approx_eq(j.sqrt(), castom_math::castom_sqrt(j), castom_math::EPS_6);
    }
    assert_approx_eq(1_f64.sqrt(), castom_math::castom_sqrt(1.0), castom_math::EPS_6);
    assert_approx_eq(8_f64.sqrt(), castom_math::castom_sqrt(8.0), castom_math::EPS_6);
    assert_approx_eq(0.1_f64.sqrt(), castom_math::castom_sqrt(0.1), castom_math::EPS_6);
    assert_approx_eq(500_f64.sqrt(), castom_math::castom_sqrt(500.0), castom_math::EPS_6);
    assert_approx_eq(1_000_000_f64.sqrt(), castom_math::castom_sqrt(1_000_000.0), castom_math::EPS_6);
    assert_approx_eq(0.0000001_f64.sqrt(), castom_math::castom_sqrt(0.0000001), castom_math::EPS_6);
    assert_approx_eq(1.5_f64.sqrt(), castom_math::castom_sqrt(1.5), castom_math::EPS_6);
    assert_approx_eq(2_f64.sqrt(), castom_math::castom_sqrt(2.0), castom_math::EPS_6);
    assert!(castom_math::castom_sqrt(-2.0).is_nan());
}

#[test]
fn castom_test_ceil() {
    assert_eq!(1.000001_f64.ceil(), castom_math::castom_ceil(1.000001));
    assert_eq!(8.44_f64.ceil(), castom_math::castom_ceil(8.44));
    assert_eq!(0.1_f64.ceil(), castom_math::castom_ceil(0.1));
    assert_eq!(500.55_f64.ceil(), castom_math::castom_ceil(500.55));
    assert_eq!(1_000_000.0_f64.ceil(), castom_math::castom_ceil(1_000_000.0));
    assert_eq!(0.0000001_f64.ceil(), castom_math::castom_ceil(0.0000001));
    assert_eq!(1.5_f64.ceil(), castom_math::castom_ceil(1.5));
    assert_eq!(0.0_f64.ceil(), castom_math::castom_ceil(0.0));
}

#[test]
fn castom_test_floor() {
    assert_eq!(1.000001_f64.floor(), castom_math::castom_floor(1.000001));
    assert_eq!(8.44_f64.floor(), castom_math::castom_floor(8.44));
    assert_eq!(0.1_f64.floor(), castom_math::castom_floor(0.1));
    assert_eq!(500.55_f64.floor(), castom_math::castom_floor(500.55));
    assert_eq!(1_000_000.0_f64.floor(), castom_math::castom_floor(1_000_000.0));
    assert_eq!(0.0000001_f64.floor(), castom_math::castom_floor(0.0000001));
    assert_eq!(1.5_f64.floor(), castom_math::castom_floor(1.5));
    assert_eq!(0.0_f64.floor(), castom_math::castom_floor(0.0));
    assert_eq!((-1.000001_f64).floor(), castom_math::castom_floor(-1.000001));
    assert_eq!((-8.44_f64).floor(), castom_math::castom_floor(-8.44));
    assert_eq!((-0.1_f64).floor(), castom_math::castom_floor(-0.1));
    assert_eq!((-500.55_f64).floor(), castom_math::castom_floor(-500.55));
    assert_eq!((-1_000_000.0_f64).floor(), castom_math::castom_floor(-1_000_000.0));
    assert_eq!((-0.0000001_f64).floor(), castom_math::castom_floor(-0.0000001));
    assert_eq!((-1.5_f64).floor(), castom_math::castom_floor(-1.5));
    assert_eq!((-0.0_f64).floor(), castom_math::castom_floor(-0.0));
}

#[test]
fn castom_test_trunc() {
    assert_eq!(1.000001_f64.trunc(), castom_math::castom_trunc(1.000001));
    assert_eq!(8.44_f64.trunc(), castom_math::castom_trunc(8.44));
    assert_eq!(0.1_f64.trunc(), castom_math::castom_trunc(0.1));
    assert_eq!(500.55_f64.trunc(), castom_math::castom_trunc(500.55));
    assert_eq!(1_000_000.0_f64.trunc(), castom_math::castom_trunc(1_000_000.0));
    assert_eq!(0.0000001_f64.trunc(), castom_math::castom_trunc(0.0000001));
    assert_eq!(1.5_f64.trunc(), castom_math::castom_trunc(1.5));
    assert_eq!(0.0_f64.trunc(), castom_math::castom_trunc(0.0));
    assert_eq!((-1.000001_f64).trunc(), castom_math::castom_trunc(-1.000001));
    assert_eq!((-8.44_f64).trunc(), castom_math::castom_trunc(-8.44));
    assert_eq!((-0.1_f64).trunc(), castom_math::castom_trunc(-0.1));
    assert_eq!((-500.55_f64).trunc(), castom_math::castom_trunc(-500.55));
    assert_eq!((-1_000_000.0_f64).trunc(), castom_math::castom_trunc(-1_000_000.0));
    assert_eq!((-0.0000001_f64).trunc(), castom_math::castom_trunc(-0.0000001));
    assert_eq!((-1.5_f64).trunc(), castom_math::castom_trunc(-1.5));
    assert_eq!((-0.0_f64).trunc(), castom_math::castom_trunc(-0.0));
}

#[test]
fn castom_test_fmod() {
    assert_approx_eq(10.5_f64 % -3.0, castom_math::castom_fmod(10.5, -3.0), castom_math::EPS_6);
    assert_approx_eq(-8.1_f64 % 4.0, castom_math::castom_fmod(-8.1, 4.0), castom_math::EPS_6);
    assert_approx_eq(-0.0_f64 % 4.4, castom_math::castom_fmod(-0.0, 4.4), castom_math::EPS_6);
    assert_approx_eq(0.0_f64 % 1.4, castom_math::castom_fmod(0.0, 1.4), castom_math::EPS_6);
    assert_approx_eq(10.1_f64 % 0.051, castom_math::castom_fmod(10.1, 0.051), castom_math::EPS_6);
    assert_approx_eq(100_000.1_f64 % -0.1, castom_math::castom_fmod(100_000.1, -0.1), castom_math::EPS_6);
    assert_approx_eq(100.1_f64 % 0.1, castom_math::castom_fmod(100.1, 0.1), castom_math::EPS_6);
    assert_approx_eq(0.00005001_f64 % 0.0000001, castom_math::castom_fmod(0.00005001, 0.0000001), castom_math::EPS_6);
    assert_approx_eq(1.5_f64 % 100.0, castom_math::castom_fmod(1.5, 100.0), castom_math::EPS_6);
    assert_approx_eq(0.0_f64 % 1.0, castom_math::castom_fmod(0.0, 1.0), castom_math::EPS_6);
}

#[test]
fn castom_test_asin() {
    for j in (-99..=99).map(|x| x as f64 * 0.01) {
        assert_approx_eq(j.asin(), castom_math::castom_asin(j), castom_math::EPS_6);
    }
    assert_approx_eq((castom_math::S21_M_PI / 4.0).asin(), castom_math::castom_asin(castom_math::S21_M_PI / 4.0), castom_math::EPS_6);
    assert_approx_eq((-0.99_f64).asin(), castom_math::castom_asin(-0.99), castom_math::EPS_6);
    assert_approx_eq(0.3_f64.asin(), castom_math::castom_asin(0.3), castom_math::EPS_6);
    assert_approx_eq(0.45_f64.asin(), castom_math::castom_asin(0.45), castom_math::EPS_6);
    assert_approx_eq(0.0_f64.asin(), castom_math::castom_asin(0.0), castom_math::EPS_6);
    assert_approx_eq((-0.18_f64).asin(), castom_math::castom_asin(-0.18), castom_math::EPS_6);
    assert_approx_eq(0.360_f64.asin(), castom_math::castom_asin(0.360), castom_math::EPS_6);
    assert_approx_eq(0.255_f64.asin(), castom_math::castom_asin(0.255), castom_math::EPS_6);
    assert_approx_eq((-0.50_f64).asin(), castom_math::castom_asin(-0.50), castom_math::EPS_6);
    assert_approx_eq(0.000001_f64.asin(), castom_math::castom_asin(0.000001), castom_math::EPS_6);
    assert_approx_eq(1.0_f64.asin(), castom_math::castom_asin(1.0), castom_math::EPS_6);
    assert_approx_eq((-1.0_f64).asin(), castom_math::castom_asin(-1.0), castom_math::EPS_6);
}

#[test]
fn castom_test_cos() {
    for j in (-7210..=7210).map(|x| x as f64 * 0.1) {
        assert_approx_eq(j.cos(), castom_math::castom_cos(j), castom_math::EPS_6);
    }
    assert_approx_eq(30.0_f64.cos(), castom_math::castom_cos(30.0), castom_math::EPS_6);
    assert_approx_eq(45.0_f64.cos(), castom_math::castom_cos(45.0), castom_math::EPS_6);
    assert_approx_eq(0.0_f64.cos(), castom_math::castom_cos(0.0), castom_math::EPS_6);
    assert_approx_eq(180.0_f64.cos(), castom_math::castom_cos(180.0), castom_math::EPS_6);
    assert_approx_eq(360.0_f64.cos(), castom_math::castom_cos(360.0), castom_math::EPS_6);
    assert_approx_eq(25.5_f64.cos(), castom_math::castom_cos(25.5), castom_math::EPS_6);
    assert_approx_eq((-5.0_f64).cos(), castom_math::castom_cos(-5.0), castom_math::EPS_6);
    assert_approx_eq(500.21_f64.cos(), castom_math::castom_cos(500.21), castom_math::EPS_6);
    assert_approx_eq(0.000001_f64.cos(), castom_math::castom_cos(0.000001), castom_math::EPS_6);
    assert_approx_eq(0.0001_f64.cos(), castom_math::castom_cos(0.0001), castom_math::EPS_6);
}

#[test]
fn castom_test_acos() {
    for j in (-99..=99).map(|x| x as f64 * 0.01) {
        assert_approx_eq(j.acos(), castom_math::castom_acos(j), castom_math::EPS_6);
    }
    assert_approx_eq((castom_math::S21_M_PI / 4.0).acos(), castom_math::castom_acos(castom_math::S21_M_PI / 4.0), castom_math::EPS_6);
    assert_approx_eq((-0.99_f64).acos(), castom_math::castom_acos(-0.99), castom_math::EPS_6);
    assert_approx_eq(0.3_f64.acos(), castom_math::castom_acos(0.3), castom_math::EPS_6);
    assert_approx_eq(0.45_f64.acos(), castom_math::castom_acos(0.45), castom_math::EPS_6);
    assert_approx_eq(0.0_f64.acos(), castom_math::castom_acos(0.0), castom_math::EPS_6);
    assert_approx_eq((-0.18_f64).acos(), castom_math::castom_acos(-0.18), castom_math::EPS_6);
    assert_approx_eq(0.360_f64.acos(), castom_math::castom_acos(0.360), castom_math::EPS_6);
    assert_approx_eq(0.255_f64.acos(), castom_math::castom_acos(0.255), castom_math::EPS_6);
    assert_approx_eq((-0.50_f64).acos(), castom_math::castom_acos(-0.50), castom_math::EPS_6);
    assert_approx_eq(0.000001_f64.acos(), castom_math::castom_acos(0.000001), castom_math::EPS_6);
}

#[test]
fn castom_test_atan() {
    for j in (-1000..=1000).map(|x| x as f64 * 0.1) {
        assert_approx_eq(j.atan(), castom_math::castom_atan(j), castom_math::EPS_6);
    }
    assert_approx_eq((castom_math::S21_M_PI / 4.0).atan(), castom_math::castom_atan(castom_math::S21_M_PI / 4.0), castom_math::EPS_6);
    assert_approx_eq((-castom_math::S21_M_PI / 4.0).atan(), castom_math::castom_atan(-castom_math::S21_M_PI / 4.0), castom_math::EPS_6);
    assert_approx_eq((-0.99_f64).atan(), castom_math::castom_atan(-0.99), castom_math::EPS_6);
    assert_approx_eq(0.3_f64.atan(), castom_math::castom_atan(0.3), castom_math::EPS_6);
    assert_approx_eq(0.45_f64.atan(), castom_math::castom_atan(0.45), castom_math::EPS_6);
    assert_approx_eq(0.0_f64.atan(), castom_math::castom_atan(0.0), castom_math::EPS_6);
    assert_approx_eq((-0.18_f64).atan(), castom_math::castom_atan(-0.18), castom_math::EPS_6);
    assert_approx_eq(0.360_f64.atan(), castom_math::castom_atan(0.360), castom_math::EPS_6);
    assert_approx_eq(0.255_f64.atan(), castom_math::castom_atan(0.255), castom_math::EPS_6);
    assert_approx_eq((-0.50_f64).atan(), castom_math::castom_atan(-0.50), castom_math::EPS_6);
    assert_approx_eq(0.000001_f64.atan(), castom_math::castom_atan(0.000001), castom_math::EPS_6);
    assert_approx_eq(2.1_f64.atan(), castom_math::castom_atan(2.1), castom_math::EPS_6);
    assert_approx_eq(1_f64.atan(), castom_math::castom_atan(1.0), castom_math::EPS_6);
    assert_approx_eq((-1_f64).atan(), castom_math::castom_atan(-1.0), castom_math::EPS_6);
    assert_approx_eq((-50_f64).atan(), castom_math::castom_atan(-50.0), castom_math::EPS_6);
    assert_approx_eq(0_f64.atan(), castom_math::castom_atan(0.0), castom_math::EPS_6);
}

#[test]
fn castom_test_sin() {
    for j in (-1000..=1000).map(|x| x as f64 * 0.01) {
        assert_approx_eq(j.sin(), castom_math::castom_sin(j), castom_math::EPS_6);
    }
    assert_approx_eq(0_f64.sin(), castom_math::castom_sin(0.0), castom_math::EPS_6);
    assert_approx_eq((castom_math::S21_M_PI / 6.0).sin(), castom_math::castom_sin(castom_math::S21_M_PI / 6.0), castom_math::EPS_6);
    assert_approx_eq((castom_math::S21_M_PI / 4.0).sin(), castom_math::castom_sin(castom_math::S21_M_PI / 4.0), castom_math::EPS_6);
    assert_approx_eq((castom_math::S21_M_PI / 3.0).sin(), castom_math::castom_sin(castom_math::S21_M_PI / 3.0), castom_math::EPS_6);
    assert_approx_eq((castom_math::S21_M_PI / 2.0).sin(), castom_math::castom_sin(castom_math::S21_M_PI / 2.0), castom_math::EPS_6);
    assert_approx_eq(castom_math::S21_M_PI.sin(), castom_math::castom_sin(castom_math::S21_M_PI), castom_math::EPS_6);
    assert_approx_eq((3.0 * castom_math::S21_M_PI / 2.0).sin(), castom_math::castom_sin(3.0 * castom_math::S21_M_PI / 2.0), castom_math::EPS_6);
    assert_approx_eq((2.0 * castom_math::S21_M_PI).sin(), castom_math::castom_sin(2.0 * castom_math::S21_M_PI), castom_math::EPS_6);
}

#[test]
fn castom_test_tan() {
    for j in (-100..=100).map(|x| x as f64 * 0.1) {
        assert_approx_eq(j.tan(), castom_math::castom_tan(j), castom_math::EPS_6);
    }
    assert_approx_eq(360.0_f64.tan(), castom_math::castom_tan(360.0), castom_math::EPS_6);
    assert_approx_eq(0.000001_f64.tan(), castom_math::castom_tan(0.000001), castom_math::EPS_6);
    assert_approx_eq(30.0_f64.tan(), castom_math::castom_tan(30.0), castom_math::EPS_6);
    assert_approx_eq(45.0_f64.tan(), castom_math::castom_tan(45.0), castom_math::EPS_6);
    assert_approx_eq(0.0_f64.tan(), castom_math::castom_tan(0.0), castom_math::EPS_6);
    assert_approx_eq(180.0_f64.tan(), castom_math::castom_tan(180.0), castom_math::EPS_6);
    assert_approx_eq(25.5_f64.tan(), castom_math::castom_tan(25.5), castom_math::EPS_6);
    assert_approx_eq((-5.0_f64).tan(), castom_math::castom_tan(-5.0), castom_math::EPS_6);
    assert_approx_eq(500.21_f64.tan(), castom_math::castom_tan(500.21), castom_math::EPS_6);
    assert_approx_eq(0.0001_f64.tan(), castom_math::castom_tan(0.0001), castom_math::EPS_6);
    assert_approx_eq((castom_math::S21_M_PI / 6.0).tan(), castom_math::castom_tan(castom_math::S21_M_PI / 6.0), castom_math::EPS_6);
    assert_approx_eq((castom_math::S21_M_PI / 4.0).tan(), castom_math::castom_tan(castom_math::S21_M_PI / 4.0), castom_math::EPS_6);
    assert_approx_eq((castom_math::S21_M_PI / 3.0).tan(), castom_math::castom_tan(castom_math::S21_M_PI / 3.0), castom_math::EPS_6);
    assert_approx_eq(castom_math::S21_M_PI.tan(), castom_math::castom_tan(castom_math::S21_M_PI), castom_math::EPS_6);
}

fn main() {}