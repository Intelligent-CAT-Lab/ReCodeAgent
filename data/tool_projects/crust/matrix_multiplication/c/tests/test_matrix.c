//
// Created by ruibin on 12/12/22.
//

#include <stddef.h>
#include <stdio.h>
#include "matrix.h"
#include "utils.h"

#define ARRAY_TYPE_CHAR "float"

void test_generate_matrix(ARRAY_TYPE** matrix, int x, int y, char* name, char* type){
    generate_matrix(&matrix, x, y, false);
    print_array(matrix, x, y, name, type);
    free_matrix(&matrix, x);
}

void tests_generate_matrix(){
    ARRAY_TYPE** matrix = NULL;
    int x = 4;
    int y = 4;
    test_generate_matrix(matrix, x, y, "matrix", ARRAY_TYPE_CHAR);
    x = 3; y = 4;
    test_generate_matrix(matrix, x, y, "matrix 2", ARRAY_TYPE_CHAR);

    x = 4; y = 3;
    test_generate_matrix(matrix, x, y, "matrix 3", ARRAY_TYPE_CHAR);


    x = 0; y = 0;
    test_generate_matrix(matrix, x, y, "matrix 4", ARRAY_TYPE_CHAR);

    x = 2; y = 2;
    test_generate_matrix(matrix, x, y, NULL, ARRAY_TYPE_CHAR);
}

void test_multiply(int x1, int y1,
                    int x2, int y2, char* test_name, int method){
    ARRAY_TYPE** matrix1 = NULL;
    ARRAY_TYPE** matrix2 = NULL;
    generate_matrix(&matrix1, x1, y1, false);
    generate_matrix(&matrix2, x2, y2, false);
    ARRAY_TYPE** r;
    generate_matrix(&r, x1, y2, true);
    int flag = multiply(matrix1, matrix2, r, x1, y1, x2, y2, method);
    if(flag!=0){
        printf("x1:%d, y1:%d, x2:%d, y2:%d, method: %d: error\n", x1, y1, x2, y2, 1);
    }else{
        printf("=====%s======\n", test_name);
        print_array(matrix1, x1, y1, "result", ARRAY_TYPE_CHAR);
        print_array(matrix2, x2, y2, "result", ARRAY_TYPE_CHAR);
        print_array(r, x1, y2, "result", ARRAY_TYPE_CHAR);
        printf("=============\n");
    }

    free_matrix(&matrix1, x1);
    free_matrix(&matrix2, x2);
    free_matrix(&r, x1);

}

void tests_multiply(){
    int x1=4, y1=4, x2=4, y2 = 4;
    test_multiply(x1, y1, x2, y2, "multiply 1-1", 1);
    test_multiply(x1, y1, x2, y2, "multiply 1-3", 3);

    x1=8; y1=8; x2=8; y2 = 8;
    test_multiply(x1, y1, x2, y2, "multiply 1-1", 1);
    test_multiply(x1, y1, x2, y2, "multiply 1-3", 3);
    test_multiply(x1, y1, x2, y2, "multiply 1-2", 2);
    x1=4; y1=4; x2=1; y2 = 4;
    test_multiply(x1, y1, x2, y2, "multiply 2-1", 1);
    test_multiply(x1, y1, x2, y2, "multiply 2-3", 3);

    x1=1; y1=1; x2=1; y2 = 1;
    test_multiply(x1, y1, x2, y2, "multiply 3-1", 1);
    test_multiply(x1, y1, x2, y2, "multiply 3-3", 3);

    x1=1; y1=2; x2=2; y2 = 1;
    test_multiply(x1, y1, x2, y2, "multiply 4-1", 1);
    test_multiply(x1, y1, x2, y2, "multiply 4-3", 3);

    x1=1; y1=2; x2=2; y2 = 1;
    test_multiply(x1, y1, x2, y2, "multiply 5-0", 0);


}

int main(){
//    tests_generate_matrix();
    tests_multiply();
}
