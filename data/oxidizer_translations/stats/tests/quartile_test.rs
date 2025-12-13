use wspace::data::Float64Data;
use wspace::quartile::{inter_quartile_range, midhinge, quartile, trimean};

#[test]
fn TestQuartile() {
    let s1 = vec![6.0, 7.0, 15.0, 36.0, 39.0, 40.0, 41.0, 42.0, 43.0, 47.0, 49.0];
    let s2 = vec![7.0, 15.0, 36.0, 39.0, 40.0, 41.0];

    let test_cases = vec![
        (&s1, 15.0, 40.0, 43.0),
        (&s2, 15.0, 37.5, 40.0),
    ];

    for (input, expected_q1, expected_q2, expected_q3) in test_cases {
        let data = Float64Data::from(input.clone());
        let quartiles = quartile(data);

        assert!(quartiles.is_ok(), "Should not have returned an error");
        let quartiles = quartiles.unwrap();

        assert_eq!(quartiles.q1, expected_q1, "Q1 {} != {}", quartiles.q1, expected_q1);
        assert_eq!(quartiles.q2, expected_q2, "Q2 {} != {}", quartiles.q2, expected_q2);
        assert_eq!(quartiles.q3, expected_q3, "Q3 {} != {}", quartiles.q3, expected_q3);
    }

    let empty = Float64Data::from(vec![]);
    let err = quartile(empty);
    assert!(err.is_err(), "Empty slice should have returned an error");
}

#[test]
fn TestInterQuartileRange() {
    let s1 = vec![102.0, 104.0, 105.0, 107.0, 108.0, 109.0, 110.0, 112.0, 115.0, 116.0, 118.0];
    let data = Float64Data::from(s1.clone());
    let iqr = inter_quartile_range(data).unwrap();

    assert_eq!(iqr, 10.0, "IQR {} != 10", iqr);

    let empty = Float64Data::from(vec![]);
    let err = inter_quartile_range(empty);
    assert!(err.is_err(), "Empty slice should have returned an error");
}

#[test]
fn TestMidhinge() {
    let s1 = vec![1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0, 9.0, 10.0, 11.0, 12.0, 13.0];
    let data = Float64Data::from(s1.clone());
    let mh = midhinge(data).unwrap();

    assert_eq!(mh, 7.5, "Midhinge {} != 7.5", mh);

    let empty = Float64Data::from(vec![]);
    let err = midhinge(empty);
    assert!(err.is_err(), "Empty slice should have returned an error");
}

#[test]
fn TestTrimean() {
    let s1 = vec![1.0, 3.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 8.0, 8.0, 9.0, 9.0, 10.0, 11.0, 12.0, 13.0];
    let data = Float64Data::from(s1.clone());
    let tr = trimean(data).unwrap();

    assert_eq!(tr, 7.25, "Trimean {} != 7.25", tr);

    let empty = Float64Data::from(vec![]);
    let err = trimean(empty);
    assert!(err.is_err(), "Empty slice should have returned an error");
}
