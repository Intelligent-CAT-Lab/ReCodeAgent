use avalanche::avalanche::*;

/// Tests the matrix “macros” by allocating a 2×2 matrix,
/// setting each element to 1/(r+c+1), and then verifying the values.
fn test_macros() {
    let mut m = Matrix::matrix_alloc(2, 2);
    assert!(!m.vals.is_empty());

    for r in 0..2 {
        for c in 0..2 {
            m.matrix_set(r, c, 1.0 / ((r + c + 1) as f64));
        }
    }

    for r in 0..2 {
        for c in 0..2 {
            let expected = 1.0 / ((r + c + 1) as f64);
            assert_eq!(expected, m.matrix_get(r, c));
        }
    }

    println!("test_macros PASS");
}

/// Prints a 3×3 Hilbert matrix by allocating the matrix,
/// filling it with 1/(r+c+1) and printing it.
fn print_hilbert() {
    let mut m = Matrix::matrix_alloc(3, 3);
    assert!(!m.vals.is_empty());

    for r in 0..3 {
        for c in 0..3 {
            m.matrix_set(r, c, 1.0 / ((r + c + 1) as f64));
        }
    }

    m.matrix_fprintf(&mut std::io::stdout(), "%8.4f");
}

#[test]
fn test() {
    test_macros();
    print_hilbert();
}
fn main() {}
