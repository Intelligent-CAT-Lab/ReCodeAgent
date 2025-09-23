use Genetic_neural_network_for_simple_control::matrix_math::{Matrix, matrixSubstAdd, matrixMultiply, matrixAllValuesFormula};
use Genetic_neural_network_for_simple_control::activation_fnc::*;

pub fn from_vec(matrix: Vec<Vec<f32>>) -> Matrix {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    Matrix {
        matrix,
        sizes: vec![rows, cols],
    }
}


fn compare_matrices(a: &Matrix, b: &Matrix) -> bool {
    if a.matrix.len() != b.matrix.len() || a.matrix[0].len() != b.matrix[0].len() {
        return false;
    }
    for x in 0..a.matrix.len() {
        for y in 0..a.matrix[x].len() {
            if (a.matrix[x][y] - b.matrix[x][y]).abs() > f32::EPSILON {
                return false;
            }
        }
    }
    true
}
#[test]
pub fn test_matrix_add_sub()  {
    println!("\x1b[1m=======TEST MATRIX ADD SUB STARTED=======\x1b[0m");

    let a = from_vec(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    let b = from_vec(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);

    let desired_add = from_vec(vec![vec![2.0, 2.0], vec![2.0, 2.0]]);
    let desired_sub = from_vec(vec![vec![0.0, 0.0], vec![0.0, 0.0]]);

    let add = matrixSubstAdd(&a, &b, 1);
    let sub = matrixSubstAdd(&a, &b, 0);

    // println!("Result of the add:");
    // add.print();
    // println!("Result of the sub:");
    // sub.print();

    let ok_add = compare_matrices(&add, &desired_add);
    let ok_sub = compare_matrices(&sub, &desired_sub);

    assert!(ok_add);
    assert!(ok_sub);

    println!("\x1b[1m\x1b[32m=======TEST MATRIX ADD SUB SUCCESSFUL=======\x1b[0m");
    
}
#[test]
pub fn test_matrix_multiply() {
    println!("\x1b[1m=======TEST MATRIX MULTIPLY STARTED=======\x1b[0m");

    let a = from_vec(vec![vec![1.0, 1.0, 1.0], vec![2.0, 2.0, 2.0]]);
    let b = from_vec(vec![vec![1.0, 2.0], vec![1.0, 2.0], vec![1.0, 2.0]]);
    let desired = from_vec(vec![vec![3.0, 6.0], vec![6.0, 12.0]]);

    let result = matrixMultiply(&a, &b);
    assert!(compare_matrices(&result, &desired));

    println!("\x1b[1m\x1b[32m=======TEST MATRIX MULTIPLY SUCCESSFUL=======\x1b[0m");
}
#[test]
pub fn test_matrix_all_values_formula()  {
    println!("\x1b[1m=======TEST MATRIX ALL VALUES FORMULA STARTED=======\x1b[0m");

    let mut data_tanh = from_vec(vec![vec![1.0, 1.0], vec![2.0, 2.0]]);
    let mut data_sigm = from_vec(vec![vec![1.0, 1.0], vec![2.0, 2.0]]);

    let func_ptr_tanh = tangenth;
    let func_ptr_sigm = sigmoid;

    let desired_tanh = from_vec(vec![
        vec![func_ptr_tanh(1.0), func_ptr_tanh(1.0)],
        vec![func_ptr_tanh(2.0), func_ptr_tanh(2.0)],
    ]);
    let desired_sigm = from_vec(vec![
        vec![func_ptr_sigm(1.0), func_ptr_sigm(1.0)],
        vec![func_ptr_sigm(2.0), func_ptr_sigm(2.0)],
    ]);

    matrixAllValuesFormula(&mut data_tanh, func_ptr_tanh);
    matrixAllValuesFormula(&mut data_sigm, func_ptr_sigm);

    let ok_tanh = compare_matrices(&data_tanh, &desired_tanh);
    let ok_sigm = compare_matrices(&data_sigm, &desired_sigm);

    assert!(ok_tanh);
    assert!(ok_sigm);

    println!("\x1b[1m\x1b[32m=======TEST MATRIX ALL VALUES FORMULA SUCCESSFUL=======\x1b[0m");
}

pub fn test_matrix_from_pointer() {
    println!("\x1b[1m=======TEST MATRIX FROM POINTER STARTED=======\x1b[0m");

    let desired = from_vec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
        vec![5.0, 6.0],
    ]);

    let data = Matrix::createMatrixFromPointer(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![3, 2]);

    // println!("Result of the matrix from pointer:");
    // data.print();

    assert!(compare_matrices(&data, &desired));

    println!("\x1b[1m\x1b[32m=======TEST MATRIX FROM POINTER SUCCESSFUL=======\x1b[0m");
    
}
#[test]
pub fn test_matrix_fully_copy_matrix() {
    println!("\x1b[1m=======TEST MATRIX FULLY COPY STARTED=======\x1b[0m");

    let original = from_vec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
        vec![5.0, 6.0],
    ]);
    let copied = original.clone();

    assert!(compare_matrices(&original, &copied));
    println!("\x1b[1m\x1b[32m=======TEST MATRIX FULLY COPY SUCCESSFUL=======\x1b[0m");
    
}
pub fn main(){}