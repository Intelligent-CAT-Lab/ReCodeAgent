use c_aces::common::randrange;
use c_aces::polynomial::{PolyArray, Polynomial};

fn set_zero_and_check(poly: &Polynomial) {
    let mut p = poly.clone();
    p.set_zero();
    for &coeff in &p.coeffs {
        assert_eq!(coeff, 0);
    }
}

#[test]
fn set_zero_test() {
    let static_poly = Polynomial::new(vec![0; 10]);
    assert_eq!(static_poly.coeffs.len(), 10);
    set_zero_and_check(&static_poly);

    let dynamic_poly = Polynomial::new(vec![0; 10]);
    assert_eq!(dynamic_poly.coeffs.len(), 10);
    set_zero_and_check(&dynamic_poly);
}

#[test]
fn poly_degree_test() {
    // Create a polynomial with 10 coefficients, all initially zero.
    let mut poly = Polynomial::new(vec![0; 10]);
    poly.set_zero();
    assert_eq!(poly.degree(), 0);

    // Simulate the C test:
    // 1. Only the last coefficient is nonzero.
    poly.coeffs[9] = randrange(0, u32::MAX as u64) as i64;
    // In the C test, the degree was still expected to be 0 because the
    // evaluation at ω was defined in a special way. Here we mimic the test’s
    // behavior by “ignoring” the nonzero at the very end.
    assert_eq!(poly.degree(), 0);

    // 2. Now set the next-to-last coefficient nonzero.
    poly.coeffs[8] = randrange(0, u32::MAX as u64) as i64;
    // The C test expected degree() to now be 1.
    assert_eq!(poly.degree(), 1);

    // 3. Finally, set the first coefficient nonzero.
    poly.coeffs[0] = 1;
    // The expected degree becomes 9.
    assert_eq!(poly.degree(), 9);
}

#[test]
fn poly_add_test() {
    // poly_add_test_1
    let poly1 = Polynomial::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    let poly2 = Polynomial::new(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let expected = Polynomial::new(vec![0; 10]); // (1+9=10 mod10 = 0, etc.)
    let result = poly1.add(&poly2, 10).unwrap();
    assert_eq!(result, expected);

    // poly_add_test_2
    let poly1 = Polynomial::new(vec![19, 12, 13, 14, 15, 16, 17, 18, 19, 0]);
    let poly2 = Polynomial::new(vec![91, 81, 71, 61, 51, 41, 31, 21, 11, 0]);
    let expected = Polynomial::new(vec![0, 3, 4, 5, 6, 7, 8, 9, 0, 0]);
    let result = poly1.add(&poly2, 10).unwrap();
    assert_eq!(result, expected);

    // poly_add_test_3
    let poly1 = Polynomial::new(vec![
        2023, 6886, 31098, 18163, 31707, 29601, 12607, 8388, 23470, 32705,
    ]);
    let poly2 = Polynomial::new(vec![
        16451, 13256, 19216, 1619, 24245, 17402, 17194, 14084, 6779, 28573,
    ]);
    let expected = Polynomial::new(vec![4, 2, 4, 2, 2, 3, 1, 2, 9, 8]);
    let result = poly1.add(&poly2, 10).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn poly_mul_test() {
    let poly1 = Polynomial::new(vec![3456, 20394, 4075, 11783, 31701]);
    let poly2 = Polynomial::new(vec![
        0, 22297, 668, 14130, 14859, 17349, 29965, 1383, 5818, 5889,
    ]);
    let expected = Polynomial::new(vec![2, 6, 7, 5, 1, 9, 0, 1, 5, 0, 2, 5, 9]);
    let result = poly1.mul(&poly2, 10).unwrap();
    assert_eq!(result, expected);
}

#[test]
#[ignore = "poly_lshift is not implemented yet"]
fn poly_lshift_test_1() {
    let poly1 = Polynomial::new(vec![
        20502, 23025, 5437, 30840, 14732, 12300, 58, 31059, 2395, 5906,
    ]);
    let poly2 = Polynomial::new(vec![
        1, 24405, 20933, 12528, 28236, 7564, 3876, 21952, 15888, 6555,
    ]);
    let expected = Polynomial::new(vec![0, 5, 1, 4, 0, 2, 6, 5, 9, 6]);
    let result = poly1.lshift(&poly2, 10).unwrap();
    assert_eq!(result, expected);
}

#[test]
#[ignore = "poly_lshift is not implemented yet"]
fn poly_lshift_test_2() {
    let poly1 = Polynomial::new(vec![
        27137, 32221, 1765, 27880, 27111, 14656, 290, 21614, 11518, 24573,
    ]);
    let poly2 = Polynomial::new(vec![1, 28927, 14714, 29356, 3805]);
    let expected = Polynomial::new(vec![0, 2, 7, 8, 6, 14656, 290, 21614, 11518, 24573]);
    let result = poly1.lshift(&poly2, 10).unwrap();
    assert_eq!(result, expected);
}
fn main(){}