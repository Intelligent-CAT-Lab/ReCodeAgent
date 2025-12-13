use wspace::{Ncr, NormBoxMullerRvs, NormCdf, NormEntropy, NormFit, NormInterval, NormIsf, NormLogCdf, NormLogPdf, NormLogSf, NormMean, NormMedian, NormMoment, NormPdf, NormPpf, NormPpfRvs, NormSf, NormStats, NormStd, NormVar};

// Approximate float comparisons
// Taken from the standard library's math/all_test.go
fn tolerance(a: f64, b: f64, e: f64) -> bool {
    // Multiplying by e here can underflow denormal values to zero.
    // Check a==b so that at least if a and b are small and identical
    // we say they match.
    if a == b {
        return true;
    }
    let mut d = a - b;
    if d < 0.0 {
        d = -d;
    }

    // note: b is correct (expected) value, a is actual value.
    // make error tolerance a fraction of b, not a.
    let mut e = e;
    if b != 0.0 {
        e = e * b;
        if e < 0.0 {
            e = -e;
        }
    }
    d < e
}

fn close(a: f64, b: f64) -> bool {
    tolerance(a, b, 1e-14)
}

fn veryclose(a: f64, b: f64) -> bool {
    tolerance(a, b, 4e-16)
}

#[test]
fn test_norm_ppf() {
    assert_eq!(NormPpf(0.5, 0.0, 1.0), 0.0);

    assert!(veryclose(NormPpf(0.1, 0.0, 1.0), -1.2815515655446004));

    assert_eq!(NormPpf(0.002423, 0.0, 1.0), -2.817096255323953);

    assert!(close(NormPpf(1.0 - 0.002423, 0.0, 1.0), 2.817096255323956));

    assert!(NormPpf(1.1, 0.0, 1.0).is_nan());

    assert!(NormPpf(-1.1, 0.0, 1.0).is_nan());

    assert_eq!(NormPpf(0.0, 0.0, 1.0), f64::NEG_INFINITY);

    assert_eq!(NormPpf(1.0, 0.0, 1.0), f64::INFINITY);
}

#[test]
fn test_norm_cdf() {
    assert_eq!(NormCdf(0.0, 0.0, 1.0), 0.5);

    assert_eq!(NormCdf(0.5, 0.0, 1.0), 0.6914624612740131);

    assert_eq!(NormCdf(-0.5, 0.0, 1.0), 0.3085375387259869);
}

#[test]
fn test_norm_pdf() {
    assert!(close(NormPdf(0.5, 0.0, 1.0), 0.35206532676429947));

    assert_eq!(NormPdf(0.0, 0.0, 1.0), 0.3989422804014327);

    assert!(close(NormPdf(-0.5, 0.0, 1.0), 0.35206532676429947));
}

#[test]
fn test_norm_log_pdf() {
    assert_eq!(NormLogPdf(0.0, 0.0, 1.0), -0.9189385332046727);

    assert_eq!(NormPdf(0.0, 0.0, 1.0), 0.3989422804014327);

    assert!(close(NormPdf(-0.5, 0.0, 1.0), 0.35206532676429947));
}

#[test]
fn test_norm_log_cdf() {
    assert_eq!(NormLogCdf(0.5, 0.0, 1.0), -0.36894641528865635);
}

#[test]
fn test_norm_isf() {
    assert_eq!(NormIsf(0.5, 0.0, 1.0), 0.0);

    assert!(veryclose(NormIsf(0.1, 0.0, 1.0), 1.2815515655446004));
}

#[test]
fn test_norm_sf() {
    assert_eq!(NormSf(0.5, 0.0, 1.0), 0.3085375387259869);
}

#[test]
fn test_norm_log_sf() {
    assert!(close(NormLogSf(0.5, 0.0, 1.0), -1.1759117615936185));
}

#[test]
fn test_norm_moment() {
    assert_eq!(NormMoment(4, 0.0, 1.0), 3.0);

    assert_eq!(NormMoment(4, 0.0, 1.0), 3.0);
}

#[test]
fn test_norm_stats() {
    assert_eq!(NormStats(0.0, 1.0, "m"), vec![0.0]);

    assert_eq!(NormStats(0.0, 1.0, "v"), vec![1.0]);

    assert_eq!(NormStats(0.0, 1.0, "s"), vec![0.0]);

    assert_eq!(NormStats(0.0, 1.0, "k"), vec![0.0]);
}

#[test]
fn test_norm_entropy() {
    assert_eq!(NormEntropy(0.0, 1.0), 1.4189385332046727);
}

#[test]
fn test_norm_fit() {
    assert_eq!(NormFit(&[0.0, 2.0, 3.0, 4.0]), [2.25, 1.479019945774904]);
}

#[test]
fn test_norm_interval() {
    assert_eq!(NormInterval(0.5, 0.0, 1.0), [-0.6744897501960818, 0.674489750196082]);
}

#[test]
fn test_norm_mean() {
    assert_eq!(NormMean(0.0, 1.0), 0.0);
}

#[test]
fn test_norm_median() {
    assert_eq!(NormMedian(0.0, 1.0), 0.0);
}

#[test]
fn test_norm_var() {
    assert_eq!(NormVar(0.0, 1.0), 1.0);
}

#[test]
fn test_norm_std() {
    assert_eq!(NormStd(0.0, 1.0), 1.0);
}

#[test]
fn test_norm_ppf_rvs() {
    assert_eq!(NormPpfRvs(0.0, 1.0, 101).len(), 101);
}

#[test]
fn test_norm_box_muller_rvs() {
    assert_eq!(NormBoxMullerRvs(0.0, 1.0, 101).len(), 101);
}

#[test]
fn test_ncr() {
    assert_eq!(Ncr(4, 1), 4);

    assert_eq!(Ncr(4, 3), 4);
}
