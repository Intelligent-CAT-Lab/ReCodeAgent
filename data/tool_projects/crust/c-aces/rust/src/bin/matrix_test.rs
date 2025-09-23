use c_aces::common::randrange;
use c_aces::matrix::{
    fill_random_invertible_pairs, matrix2d_eye, matrix2d_multiply, Matrix2D,
};

#[test]
fn matrix2d_multiply_test() {
    let dim = 3;
    let mut m1 = Matrix2D::new(dim);
    let mut m2 = Matrix2D::new(dim);

    // Fill m1 with random numbers in the range [0, 96].
    for i in 0..dim {
        for j in 0..dim {
            let val = randrange(0, 96);
            m1.set(i, j, val).unwrap();
        }
    }
    // Set m2 to the identity matrix.
    matrix2d_eye(&mut m2).unwrap();

    let result = matrix2d_multiply(&m1, &m2, 97).unwrap();
    // Multiplying by the identity should reproduce m1.
    assert_eq!(m1.data, result.data);
}

#[test]
fn fill_random_invertible_pairs_test() {
    let dim = 3;
    let mut m1 = Matrix2D::new(dim);
    let mut m2 = Matrix2D::new(dim);

    fill_random_invertible_pairs(&mut m1, &mut m2, 97, 600).unwrap();
    let result = matrix2d_multiply(&m1, &m2, 97).unwrap();

    let mut exp_result = Matrix2D::new(dim);
    matrix2d_eye(&mut exp_result).unwrap();

    // The product m1 * m2 should be the identity.
    assert_eq!(result.data, exp_result.data);
}
fn main(){}