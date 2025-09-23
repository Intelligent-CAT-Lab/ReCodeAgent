use Linear_Algebra_C::linear_algebra::*;
use Linear_Algebra_C::utils::*;

#[test]
fn test_create() {
    println!("\nTest create matrix and vector from array...");

    let data = [1.0, 2.0, 3.0, 4.0];
    let m = new_matrix(&data, 2, 2);
    assert!(
        get_matrix_element(&m, 0, 0) == 1.0
            && get_matrix_element(&m, 0, 1) == 2.0
            && get_matrix_element(&m, 1, 0) == 3.0
            && get_matrix_element(&m, 1, 1) == 4.0
            && matrix_size(&m) == 4
            && matrix_size_bytes(&m) == 32
    );
    delete_matrix(m);

    let data2 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let v = new_vector(&data2, 8);
    assert!(vector_size_bytes(&v) == 64);
    delete_vector(v);
}

#[test]
fn test_copy() {
    println!("\nTest matrix and vector copying...");
    let mut m = zero_matrix(2, 2);
    set_matrix_element(&mut m, 0, 1, 5.0);
    let n = copy_matrix(&m);
    assert!(
        get_matrix_element(&m, 0, 0) == get_matrix_element(&n, 0, 0)
            && get_matrix_element(&m, 0, 1) == get_matrix_element(&n, 0, 1)
            && get_matrix_element(&m, 1, 0) == get_matrix_element(&n, 1, 0)
            && get_matrix_element(&m, 1, 1) == get_matrix_element(&n, 1, 1)
    );
    delete_matrix(n);
    delete_matrix(m);

    let mut v = zero_vector(4);
    set_vector_element(&mut v, 2, 2.0);
    let v2 = copy_vector(&v);
    assert!(get_vector_element(&v2, 2) == 2.0);
    delete_vector(v2);
    delete_vector(v);
}

#[test]
fn test_flatten() {
    println!("\nTest flatten...");
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let m = new_matrix(&data, 2, 3);
    let flat = flatten_matrix(&m);
    assert!(get_vector_element(&flat, 0) == 1.0 && get_vector_element(&flat, 5) == 6.0);
    delete_vector(flat);
    delete_matrix(m);
}

#[test]
fn test_elem() {
    println!("\nTest element accessor and mutator...");
    let mut m = zero_matrix(2, 2);
    set_matrix_element(&mut m, 0, 0, 4.0);
    assert!(
        get_matrix_element(&m, 0, 0) == 4.0
            && get_matrix_element(&m, 0, 1) == 0.0
            && get_matrix_element(&m, 1, 0) == 0.0
            && get_matrix_element(&m, 1, 1) == 0.0
    );
    delete_matrix(m);

    let mut v = zero_vector(2);
    set_vector_element(&mut v, 1, 10.0);
    assert!(get_vector_element(&v, 1) == 10.0);
    delete_vector(v);
}

#[test]
fn test_row_vector() {
    println!("\nTest accessor/mutator for row vectors...");
    let mut m = zero_matrix(3, 2);
    let mut row = get_row_vector(&m, 1);
    set_vector_element(&mut row, 0, 123.0);
    assert!(get_vector_element(&row, 0) == 123.0 && get_vector_element(&row, 1) == 0.0);
    let data = [1.0, 4.0];
    let v = new_vector(&data, 2);
    set_row_vector(&mut m, 1, &v);
    assert!(
        get_matrix_element(&m, 0, 0) == 0.0
            && get_matrix_element(&m, 0, 1) == 0.0
            && get_matrix_element(&m, 1, 0) == 1.0
            && get_matrix_element(&m, 1, 1) == 4.0
    );
    delete_vector(v);
    delete_vector(row);
    delete_matrix(m);
}

#[test]
fn test_col_vector() {
    println!("\nTest accessor/mutator for column vectors...");
    let mut m = zero_matrix(3, 2);
    let mut col = get_col_vector(&m, 0);
    set_vector_element(&mut col, 0, 5.0);
    set_vector_element(&mut col, 1, 10.0);
    set_vector_element(&mut col, 2, 15.0);
    assert!(get_vector_element(&col, 0) == 5.0 && get_vector_element(&col, 1) == 10.0);
    let data = [10.0, 3.0];
    let v = new_vector(&data, 2);
    set_col_vector(&mut m, 0, &v);
    delete_vector(v);
    delete_vector(col);
    delete_matrix(m);
}

#[test]
fn test_main_diagonal() {
    println!("\nTest accessor/mutator for main diagonal...");
    let mut m = zero_matrix(2, 2);
    set_matrix_element(&mut m, 0, 0, 34.0);
    set_matrix_element(&mut m, 1, 1, 56.0);
    let v = get_main_diagonal(&m);
    assert!(get_vector_element(&v, 0) == 34.0 && get_vector_element(&v, 1) == 56.0);
    delete_vector(v);
    delete_matrix(m);

    let mut n = zero_matrix(2, 2);
    let mut w = zero_vector(2);
    set_vector_element(&mut w, 0, 100.0);
    set_vector_element(&mut w, 1, 4.0);
    set_main_diagonal(&mut n, &w);
    assert!(get_matrix_element(&n, 0, 0) == 100.0 && get_matrix_element(&n, 1, 1) == 4.0);
    delete_matrix(n);
    delete_vector(w);
}

#[test]
fn test_anti_diagonal() {
    println!("\nTest accessor/mutator for anti diagonal...");
    let mut m = zero_matrix(2, 2);
    set_matrix_element(&mut m, 0, 1, 100.0);
    set_matrix_element(&mut m, 1, 0, 250.0);
    let d = get_anti_diagonal(&m);
    assert!(get_vector_element(&d, 0) == 250.0 && get_vector_element(&d, 1) == 100.0);
    delete_vector(d);
    delete_matrix(m);

    let mut n = zero_matrix(2, 2);
    let mut e = zero_vector(2);
    set_vector_element(&mut e, 0, 9.0);
    set_vector_element(&mut e, 1, 8.0);
    set_anti_diagonal(&mut n, &e);
    assert!(get_matrix_element(&n, 1, 0) == 9.0 && get_matrix_element(&n, 0, 1) == 8.0);
    delete_matrix(n);
    delete_vector(e);
}

#[test]
fn test_is_equal() {
    println!("\nTest vector and matrix equal...");
    let m = zero_matrix(2, 2);
    let n = zero_matrix(2, 2);
    let o = zero_matrix(3, 3);
    assert!(!is_matrix_equal(&m, &o));
    assert!(is_matrix_equal(&m, &n));
    let m2 = m.clone();
    set_matrix_element(&mut m2.clone(), 0, 0, 1.0);
    assert!(!is_matrix_equal(&m2, &n));
    delete_matrix(o);
    delete_matrix(n);
    delete_matrix(m);

    let v = zero_vector(3);
    let w = zero_vector(3);
    assert!(is_vector_equal(&v, &w));
    delete_vector(w);
    delete_vector(v);
}

#[test]
fn test_is_zero_matrix() {
    println!("\nTest isZeroMatrix...");
    let m = zero_matrix(1, 3);
    let mut n = zero_matrix(2, 2);
    set_matrix_element(&mut n, 0, 0, 1.0);
    assert!(is_zero_matrix(&m));
    assert!(!is_zero_matrix(&n));
    delete_matrix(n);
    delete_matrix(m);
}

#[test]
fn test_is_identity_matrix() {
    println!("\nTest isIdentityMatrix...");
    let mut m = zero_matrix(2, 2);
    set_matrix_element(&mut m, 0, 0, 1.0);
    set_matrix_element(&mut m, 1, 1, 1.0);
    assert!(is_identity_matrix(&m));
    delete_matrix(m);

    let n = zero_matrix(1, 3);
    assert!(!is_identity_matrix(&n));
    delete_matrix(n);

    let mut o = zero_matrix(3, 3);
    set_matrix_element(&mut o, 0, 0, 1.0);
    set_matrix_element(&mut o, 1, 1, 1.0);
    set_matrix_element(&mut o, 2, 2, 1.0);
    set_matrix_element(&mut o, 1, 2, -4.0);
    assert!(!is_identity_matrix(&o));
    delete_matrix(o);

    let p = identity_matrix(2);
    assert!(is_identity_matrix(&p));
    delete_matrix(p);
}

#[test]
fn test_is_square_matrix() {
    println!("\nTest isSquareMatrix...");
    let m = zero_matrix(2, 2);
    assert!(is_square_matrix(&m));
    delete_matrix(m);

    let n = zero_matrix(1, 3);
    assert!(!is_square_matrix(&n));
    delete_matrix(n);
}

#[test]
fn test_is_diagonal() {
    println!("\nTest isDiagonal...");
    let data1 = [1.0, 2.0, 3.0, 0.0, 5.0, 6.0, 0.0, 0.0, 9.0];
    let m1 = new_matrix(&data1, 3, 3);
    assert!(!is_diagonal_matrix(&m1));
    delete_matrix(m1);

    let data2 = [1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0];
    let m2 = new_matrix(&data2, 3, 3);
    assert!(is_diagonal_matrix(&m2));
    delete_matrix(m2);
}

#[test]
fn test_is_triangular() {
    println!("\nTest isTriangular, isLoTriMatrix, isUpTriMatrix...");
    let data1 = [1.0, 2.0, 3.0, 0.0, 5.0, 6.0, 0.0, 0.0, 9.0];
    let m1 = new_matrix(&data1, 3, 3);
    assert!(is_up_tri_matrix(&m1));
    delete_matrix(m1);

    let data2 = [1.0, 0.0, 0.0, 4.0, 5.0, 0.0, 7.0, 8.0, 9.0];
    let m2 = new_matrix(&data2, 3, 3);
    assert!(is_lo_tri_matrix(&m2));
    delete_matrix(m2);

    let data3 = [1.0, 0.0, 0.0, 4.0, 5.0, 0.0, 7.0, 8.0, 9.0];
    let m3 = new_matrix(&data3, 3, 3);
    assert!(is_triangular_matrix(&m3));
    delete_matrix(m3);
}

#[test]
fn test_is_symmetric() {
    println!("\nTest isSymmetric...");
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let m = new_matrix(&data, 2, 3);
    assert!(!is_matrix_symmetric(&m));
    delete_matrix(m);
    let n = zero_matrix(2, 2);
    assert!(is_matrix_symmetric(&n));
    delete_matrix(n);
}

#[test]
fn test_transpose() {
    println!("\nTest transposeMatrix...");
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let m = new_matrix(&data, 2, 3);
    let t = transpose_matrix(&m);
    assert!(
        get_matrix_element(&t, 0, 0) == 1.0
            && get_matrix_element(&t, 1, 0) == 2.0
            && get_matrix_element(&t, 2, 0) == 3.0
            && get_matrix_element(&t, 0, 1) == 4.0
            && get_matrix_element(&t, 1, 1) == 5.0
            && get_matrix_element(&t, 2, 1) == 6.0
    );
    delete_matrix(t);
    delete_matrix(m);
}

#[test]
fn test_trace() {
    println!("\nTest trace...");
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let m = new_matrix(&data, 3, 3);
    assert!(trace_matrix(&m) == 15.0);
    delete_matrix(m);
}

#[test]
fn test_add_matrices() {
    println!("\nTest adding matrices...");
    let data1 = [4.0, 6.0, 3.0, 8.0];
    let data2 = [6.0, 4.0, 7.0, 2.0];
    let m1 = new_matrix(&data1, 2, 2);
    let m2 = new_matrix(&data2, 2, 2);
    let sum = add_matrices(&m1, &m2);
    assert!(
        get_matrix_element(&sum, 0, 0) == 10.0
            && get_matrix_element(&sum, 0, 1) == 10.0
            && get_matrix_element(&sum, 1, 0) == 10.0
            && get_matrix_element(&sum, 1, 1) == 10.0
    );
    delete_matrix(sum);
    delete_matrix(m2);
    delete_matrix(m1);
}

#[test]
fn test_multiply_matrices() {
    println!("\nTest multiplying matrices...");
    let data1 = [1.0, 2.0, 3.0, 4.0];
    let data2 = [1.0, 1.0, 1.0, 1.0];
    let m1 = new_matrix(&data1, 2, 2);
    let m2 = new_matrix(&data2, 2, 2);
    let prod = multiply_matrices(&m1, &m2);
    assert!(
        get_matrix_element(&prod, 0, 0) == 3.0
            && get_matrix_element(&prod, 0, 1) == 3.0
            && get_matrix_element(&prod, 1, 0) == 7.0
            && get_matrix_element(&prod, 1, 1) == 7.0
    );
    delete_matrix(prod);
    delete_matrix(m2);
    delete_matrix(m1);
}

#[test]
fn test_scale_matrix() {
    println!("\nTest scaling matrices...");
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let m = new_matrix(&data, 3, 2);
    let scaled = scale_matrix(&m, 10.0);
    assert!(
        get_matrix_element(&scaled, 0, 0) == 10.0
            && get_matrix_element(&scaled, 0, 1) == 20.0
            && get_matrix_element(&scaled, 1, 0) == 30.0
            && get_matrix_element(&scaled, 1, 1) == 40.0
            && get_matrix_element(&scaled, 2, 0) == 50.0
            && get_matrix_element(&scaled, 2, 1) == 60.0
    );
    delete_matrix(scaled);
    delete_matrix(m);
}

#[test]
fn test_sub_matrix() {
    println!("\nTest sub matrix...");
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let m = new_matrix(&data, 3, 3);
    let sub = sub_matrix(&m, 2, 2);
    assert!(
        get_matrix_element(&sub, 0, 0) == 1.0
            && get_matrix_element(&sub, 0, 1) == 2.0
            && get_matrix_element(&sub, 1, 0) == 4.0
            && get_matrix_element(&sub, 1, 1) == 5.0
    );
    delete_matrix(sub);
    delete_matrix(m);
}

#[test]
fn test_determinant() {
    println!("\nTest determinant...");
    let data1 = [10.0];
    let m1 = new_matrix(&data1, 1, 1);
    assert!(determinant(&m1) == 10.0);
    delete_matrix(m1);

    let data2 = [4.0, 6.0, 3.0, 8.0];
    let m2 = new_matrix(&data2, 2, 2);
    assert!(determinant(&m2) == 14.0);
    delete_matrix(m2);

    let data3 = [6.0, 1.0, 1.0, 4.0, -2.0, 5.0, 2.0, 8.0, 7.0];
    let m3 = new_matrix(&data3, 3, 3);
    assert!(determinant(&m3) == -306.0);
    delete_matrix(m3);

    let data4 = [
        11.0, 9.0, 24.0, 2.0, 1.0, 5.0, 2.0, 6.0, 3.0, 17.0, 18.0, 1.0, 2.0, 5.0, 7.0, 1.0,
    ];
    let m4 = new_matrix(&data4, 4, 4);
    let det = determinant(&m4);
    // For comparison, round the determinant to one decimal place.
    assert!(roundn(det, 1) == 284.0);
    delete_matrix(m4);
}

#[test]
fn test_inverse() {
    println!("\nTest inverse matrix...");
    let data = [3.0, 0.0, 2.0, 2.0, 0.0, -2.0, 0.0, 1.0, 1.0];
    let m = new_matrix(&data, 3, 3);
    let inv = inverse_matrix(&m);
    println!("\nInv(m):");
    print_matrix(&inv, false);
    delete_matrix(inv);
    delete_matrix(m);
}

#[test]
fn test_vector_operations() {
    println!("\nTest vector operations...");
    let data1 = [2.0, 4.0, 3.0];
    let data2 = [5.0, 10.0, 15.0];
    let data3 = [1.0, 0.0, 0.0];
    let data4 = [0.0, 1.0, 0.0];
    let v = new_vector(&data1, 3);
    let w = new_vector(&data2, 3);
    let u1 = new_vector(&data3, 3);
    let u2 = new_vector(&data4, 3);

    assert!(roundn(vector_magnitude(&v), 3) == 5.385);
    assert!(!is_unit_vector(&v) && is_unit_vector(&u1));
    assert!(dot_product(&v, &w) == 95.0);
    assert!(is_vector_orthogonal(&u1, &u2));
    assert!(roundn(vector_distance(&v, &w), 3) == 13.748);

    let cross = cross_product(&v, &w);
    assert!(
        get_vector_element(&cross, 0) == 30.0
            && get_vector_element(&cross, 1) == 15.0
            && get_vector_element(&cross, 2) == 0.0
    );
    delete_vector(cross);
    delete_vector(u2);
    delete_vector(u1);
    delete_vector(w);
    delete_vector(v);
}
pub fn main(){}