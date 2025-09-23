use matrix_multiplication::matrix;
use matrix_multiplication::utils;

const ARRAY_TYPE_CHAR: &str = "float";

/// Equivalent to:
/// void test_generate_matrix(ARRAY_TYPE** matrix, int x, int y, char* name, char* type)
///
/// In Rust, we:
/// 1. Generate a matrix and store in a local Vec.
/// 2. Print its contents.
/// 3. "Free" it (clear it) to mimic your original logic.
fn test_generate_matrix(x: usize, y: usize, name: Option<&str>, typ: &str) {
    // 1. Create the matrix
    let mut matrix_data = matrix::generate_matrix(x, y, false);

    // 2. Print (mimics print_array(...))
    //    If name is None, we supply a default fallback name.
    let display_name = name.unwrap_or("No name");
    utils::print_array(&matrix_data, display_name, typ);

    // 3. Free / clear the matrix for demonstration
    //    (Rust would free automatically, but we call free_matrix to mirror C code)
    let _ = matrix::free_matrix(&mut matrix_data);
}

/// A set of tests for generating matrices with different dimensions.
#[test]
fn tests_generate_matrix() {
    // 4x4
    test_generate_matrix(4, 4, Some("matrix"), ARRAY_TYPE_CHAR);

    // 3x4
    test_generate_matrix(3, 4, Some("matrix 2"), ARRAY_TYPE_CHAR);

    // 4x3
    test_generate_matrix(4, 3, Some("matrix 3"), ARRAY_TYPE_CHAR);

    // 0x0
    test_generate_matrix(0, 0, Some("matrix 4"), ARRAY_TYPE_CHAR);

    // 2x2 but no name provided (NULL in C)
    test_generate_matrix(2, 2, None, ARRAY_TYPE_CHAR);
}

/// Equivalent to:
/// void test_multiply(int x1, int y1, int x2, int y2, char* test_name, int method)
///
/// 1. Generate m1 (x1×y1), m2 (x2×y2), r (x1×y2).
/// 2. Multiply them using the chosen `method`.
/// 3. Print results if success, else print error.
/// 4. Free them all.
fn test_multiply(x1: usize, y1: usize, x2: usize, y2: usize, test_name: &str, method: i32)-> i32 {
    // 1. Generate the matrices
    let mut matrix1 = matrix::generate_matrix(x1, y1, false);
    let mut matrix2 = matrix::generate_matrix(x2, y2, false);
    let mut r = matrix::generate_matrix(x1, y2, true);

    // 2. Multiply
    let flag = matrix::multiply(&matrix1, &matrix2, &mut r, method);
    // 3. Print or error
    if flag != 0 {
        println!(
            "x1:{}, y1:{}, x2:{}, y2:{}, method:{} => error",
            x1, y1, x2, y2, method
        );
    } else {
        
        println!("====={}======", test_name);
        utils::print_array(&matrix1, "result", ARRAY_TYPE_CHAR);
        utils::print_array(&matrix2, "result", ARRAY_TYPE_CHAR);
        utils::print_array(&r, "result", ARRAY_TYPE_CHAR);
        println!("=============");
        
    }

    // 4. Free / clear the matrices
    let _ = matrix::free_matrix(&mut matrix1);
    let _ = matrix::free_matrix(&mut matrix2);
    let _ = matrix::free_matrix(&mut r);
    flag
}

/// A set of tests for multiplying different dimensioned matrices
/// with different algorithms (method=1,2,3,...).
#[test]
fn tests_multiply() {
    let (mut x1, mut y1, mut x2, mut y2) = (4, 4, 4, 4);

    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 1-1", 1), 0);
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 1-3", 3),0);

    x1 = 8;
    y1 = 8;
    x2 = 8;
    y2 = 8;
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 1-1", 1), 0);
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 1-3", 3),0);
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 1-2", 2),0);
    

    x1 = 4;
    y1 = 4;
    x2 = 1;
    y2 = 4;
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 2-1", 1), 0);
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 2-3", 3),0);
    

    x1 = 1;
    y1 = 1;
    x2 = 1;
    y2 = 1;
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 3-1", 1), 0);
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 3-3", 3),0);

    x1 = 1;
    y1 = 2;
    x2 = 2;
    y2 = 1;
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 4-1", 1), 0);
    assert_eq!(test_multiply(x1, y1, x2, y2, "multiply 4-3", 3),0);
    

    // Testing an invalid method = 0
    assert_ne!(test_multiply(x1, y1, x2, y2, "multiply 5-0", 0),0);
}
fn main(){}
