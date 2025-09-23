use c_aces::common::{gcd, xgcd, Xgcd};

#[test]
fn gcd_test() {
    assert_eq!(gcd(1, 1), 1);
    assert_eq!(gcd(2, 1), 1);
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(2, 2), 2);
    assert_eq!(gcd(6, 15), 3);
    assert_eq!(gcd(15, 6), 3);
    assert_eq!(gcd(7, 15), 1);
    assert_eq!(gcd(15, 7), 1);
    assert_eq!(gcd(7, 15), 1);
    assert_eq!(gcd(11112, 44445), 3);
}

#[test]
fn xgcd_test() {
    let result: Xgcd = xgcd(7, 7);
    assert_eq!(result.gcd, 7);
    assert_eq!(result.a, 0);
    assert_eq!(result.b, 1);

    let result = xgcd(2, 2);
    assert_eq!(result.gcd, 2);
    assert_eq!(result.a, 0);
    assert_eq!(result.b, 1);

    let result = xgcd(1, 1);
    assert_eq!(result.gcd, 1);
    assert_eq!(result.a, 0);
    assert_eq!(result.b, 1);

    let result = xgcd(1, 2);
    assert_eq!(result.gcd, 1);
    assert_eq!(result.a, 1);
    assert_eq!(result.b, 0);

    let result = xgcd(5, 6);
    assert_eq!(result.gcd, 1);
    assert_eq!(result.a, -1);
    assert_eq!(result.b, 1);

    let result = xgcd(19, 13);
    assert_eq!(result.gcd, 1);
    assert_eq!(result.a, -2);
    assert_eq!(result.b, 3);
}
fn main(){}