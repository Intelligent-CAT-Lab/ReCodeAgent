//
// Created by ruibin on 12/12/22.
//



#ifndef MATRIX_MULTIPLICATION_MATRIX_GENERATOR_H
#define MATRIX_MULTIPLICATION_MATRIX_GENERATOR_H
#include <stdbool.h>
#define ARRAY_TYPE float
int generate_matrix(ARRAY_TYPE*** matrix, int x, int y, bool random);
int free_matrix(ARRAY_TYPE*** matrix, int x);
int algorithm1(ARRAY_TYPE** m1, ARRAY_TYPE** m2, ARRAY_TYPE** r, int x1, int y1, int x2, int y2);
int algorithm3(ARRAY_TYPE** m1, ARRAY_TYPE** m2, ARRAY_TYPE** r, int x1, int y1, int x2, int y2);
int algorithm2( ARRAY_TYPE **a,  ARRAY_TYPE **bb, ARRAY_TYPE **c, int size);
int multiply(ARRAY_TYPE** m1, ARRAY_TYPE** m2, ARRAY_TYPE** r, int x1, int y1, int x2, int y2, int method);

#endif //MATRIX_MULTIPLICATION_MATRIX_GENERATOR_H
