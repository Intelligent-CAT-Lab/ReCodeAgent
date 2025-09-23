#ifndef MATRIX_MATH_H
#define MATRIX_MATH_H
#include "genetic/population.h"

typedef struct Matrix{
    float** matrix;
    int*    sizes;
}Matrix;

// function to create new matrix
void createMatrix(struct Matrix *matrix, int *sizes);

// function to multiply two matrixes in form A * B
void matrixMultiply(struct Matrix *A, struct Matrix *B, struct Matrix *output);

// function to add/substruct two matrixes in format A-B or A+B
void matrixSubstAdd(struct Matrix *A, struct Matrix *B, struct Matrix *output, int type);

// function to pass all values trough function
void matrixAllValuesFormula(struct Matrix *matrix, float (*func_ptr)(float));

// function to create matrix from values of pointer
void createMatrixFromPointer(struct Matrix *output, float *input, int *size);

// function to delete matrix
void matrixDelete(struct Matrix *matrix);

// function to delete the 
void matrixDeleteOnlyData(struct Matrix *matrix);

// create full coppy of the matrix
void fullyCopyMatrix(struct Matrix *input, struct Matrix *output);

#endif
 