use crate::population;
#[derive(Debug, Clone, Default)]
pub struct Matrix {
    pub matrix: Vec<Vec<f32>>,
    pub sizes: Vec<usize>,
}
impl Matrix{
    pub fn createMatrix(matrix: Matrix, sizes: Vec<usize>) -> Matrix{
        unimplemented!()
    }
    pub fn createMatrixFromPointer(input: &[f32], sizes: Vec<usize>) -> Matrix{
        unimplemented!()
    }
    pub fn matrixDelete(&mut self){
        unimplemented!()
    }
    pub fn matrixDeleteOnlyData(&mut self){
        unimplemented!()
    }
}
pub fn matrixMultiply(a: &Matrix, b: &Matrix) -> Matrix{
    unimplemented!()
}
pub fn fullCopyMatrix(a: &Matrix) -> Matrix{
    unimplemented!()
}
pub fn matrixSubstAdd(a: &Matrix, b: &Matrix, operation: i32) -> Matrix{
    unimplemented!()
}
pub fn matrixAllValuesFormula(a: &mut Matrix, formula: fn(f32)->f32) -> Matrix{
    unimplemented!()
}