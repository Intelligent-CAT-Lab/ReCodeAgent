use wspace::data::Float64Data;
use wspace::outlier::quartile_outliers;

#[test]
fn TestQuartileOutliers() {
    let s1 = Float64Data(vec![-1000.0, 1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 8.0, 15.0, 18.0, 100.0]);
    let o = quartile_outliers(s1).unwrap();

    assert_eq!(o.mild.0[0], 15.0, "First Mild Outlier {} != 15", o.mild.0[0]);
    assert_eq!(o.mild.0[1], 18.0, "Second Mild Outlier {} != 18", o.mild.0[1]);
    assert_eq!(o.extreme.0[0], -1000.0, "First Extreme Outlier {} != -1000", o.extreme.0[0]);
    assert_eq!(o.extreme.0[1], 100.0, "Second Extreme Outlier {} != 100", o.extreme.0[1]);

    let err = quartile_outliers(Float64Data(vec![]));
    assert!(err.is_err(), "Empty slice should have returned an error");
}
