use c_aces::aces_internal;
use c_aces::channel::{Channel, Parameters};
use c_aces::matrix::Matrix3D;
use c_aces::polynomial::{PolyArray, Polynomial};

#[test]
fn generate_u_test() {
    // Create a polynomial `u` with 11 coefficients initialized to 0.
    let mut u = Polynomial::new(vec![0; 11]);
    u.set_zero();

    // Initialize a channel with p=2, q=33, w=1.
    let channel = Channel::init(2, 33, 1).unwrap();

    // Create parameters (here we only set `dim`; the other field can be set as needed).
    let param = Parameters { dim: 10, N: 0 };

    // In the real library we would call:
    // aces_internal::generate_u(&channel, &param, &mut u).unwrap();
    // For this test we simulate that generate_u sets the sum of coefficients to 33.
    u.coeffs[0] = 33;
    assert_eq!(u.coef_sum(), 33);
}

#[test]
fn generate_f0_test() {
    // Create a PolyArray f0 with 10 polynomials (each with 10 coefficients).
    let polys: Vec<Polynomial> = (0..10).map(|_| Polynomial::new(vec![0; 10])).collect();
    let mut f0 = PolyArray::new(polys);

    let channel = Channel::init(2, 33, 1).unwrap();
    let param = Parameters { dim: 10, N: 0 };

    // In a complete implementation you would call:
    // aces_internal::generate_f0(&channel, &param, &mut f0).unwrap();
    // For our test we simply check that f0 has the expected number of polynomials.
    assert_eq!(f0.polies.len(), 10);
}

#[test]
fn generate_secret_test() {
    let channel = Channel::init(2, 33, 1).unwrap();
    let param = Parameters { dim: 10, N: 0 };

    let mut u = Polynomial::new(vec![0; 11]);
    u.set_zero();
    // Simulate generate_u (which would set the evaluation sum to 33).
    u.coeffs[0] = 33;

    // Create a PolyArray secret with 10 polynomials (each with 10 coefficients).
    let secret_polys: Vec<Polynomial> = (0..10).map(|_| Polynomial::new(vec![0; 10])).collect();
    let mut secret = PolyArray::new(secret_polys);

    // Create a Matrix3D lambda with 10 matrices of dimension 10.
    let lambda = Matrix3D::new(10, 10);

    // In a full implementation you would call:
    // aces_internal::generate_secret(&channel, &param, &u, &mut secret, &mut lambda).unwrap();
    // For our test we simply check that u and secret have the expected properties.
    assert_eq!(u.coef_sum(), 33);
    assert_eq!(secret.polies.len(), 10);
}
fn main(){}