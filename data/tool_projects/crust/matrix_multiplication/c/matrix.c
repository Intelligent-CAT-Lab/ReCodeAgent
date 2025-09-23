//
// Created by ruibin on 12/12/22.
//
#include <malloc.h>
#include <stdlib.h>
#include <immintrin.h>
#include "matrix.h"



int generate_matrix(ARRAY_TYPE*** matrix, int x, int y, bool random){
    *matrix = malloc(sizeof(ARRAY_TYPE*) * x);
    if(!random){
        for (int i = 0; i < x; ++i) {
            (*matrix)[i] = malloc(sizeof(ARRAY_TYPE) * x);
            for (int j = 0; j < y; ++j) {
                (*matrix)[i][j] = (ARRAY_TYPE)(j + i * x);
            }
        }
    }else{
        for (int i = 0; i < x; ++i) {
            (*matrix)[i] = malloc(sizeof(ARRAY_TYPE) * y);
            for (int j = 0; j < y; ++j) {
                (*matrix)[i][j] = (ARRAY_TYPE)rand();
            }
        }
    }
    return 0;
}

int free_matrix(ARRAY_TYPE*** matrix, int x){
    for (int i = 0; i < x; ++i) {
        free((*matrix)[i]);
    }
    free(*matrix);
    *matrix = NULL;
    return 0;
}
int multiply(ARRAY_TYPE** m1, ARRAY_TYPE** m2, ARRAY_TYPE** r, int x1, int y1, int x2, int y2, int method){
    if(y1 - x2){
        printf("The number of columns in the first matrix must be equal to the number of rows in the second matrix. \n");
        return -1;
    }
    switch (method) {
        case 1:
            algorithm1(m1, m2, r, x1, y1, x2, y2);
            break;
        case 2:
            algorithm2(m1, m2, r, x1);
            break;
        case 3:
            algorithm3(m1, m2, r, x1, y1, x2, y2);
            break;
        default:
            printf("Choose the correct method!\n");
            return -1;
    }
    return 0;
}
int algorithm1(ARRAY_TYPE** m1, ARRAY_TYPE** m2, ARRAY_TYPE** r, int x1, int y1, int x2, int y2){
    for (int i = 0; i < x1; ++i) {
        for (int j = 0; j < y2; ++j) {
            r[i][j] = 0;
            for (int k = 0; k < y1; ++k) {
                r[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }
    return 0;
}

int algorithm2( ARRAY_TYPE **a,  ARRAY_TYPE **bb, ARRAY_TYPE **c, int size){
    ARRAY_TYPE** b = malloc(sizeof(ARRAY_TYPE*) * size);
    for (int i = 0; i < size; ++i) {
        b[i] = malloc(sizeof(ARRAY_TYPE) * size);
        for (int j = 0; j < size;) {
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
            b[i][j] = bb[j][i];
            c[i][j] = 0;
            j++;
        }
    }

    for (int i = 0; i < size; ++i) {
        for (int j = 0; j < size; j+=8) {
            __m256 m_a =  _mm256_loadu_ps(a[i] + j);
            for (int k = 0; k < size; k++) {
                __m256 m_b =  _mm256_loadu_ps(b[k] + j);
                __m256 m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];
            }

            j+=8; if(j >= size) break;
            m_a =  _mm256_loadu_ps(a[i] + j);
            for (int k = 0; k < size; k++) {
                __m256 m_b =  _mm256_loadu_ps(b[k] + j);
                __m256 m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];
            }

            j+=8; if(j >= size) break;
            m_a =  _mm256_loadu_ps(a[i] + j);
            for (int k = 0; k < size; k++) {
                __m256 m_b =  _mm256_loadu_ps(b[k] + j);
                __m256 m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];
            }

            j+=8; if(j >= size) break;
            m_a =  _mm256_loadu_ps(a[i] + j);
            for (int k = 0; k < size; k++) {
                __m256 m_b =  _mm256_loadu_ps(b[k] + j);
                __m256 m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];

                k++;
                m_b =  _mm256_loadu_ps(b[k] + j);
                m_c = _mm256_mul_ps(m_a, m_b);
                c[i][k] += m_c[0];
                c[i][k] += m_c[1];
                c[i][k] += m_c[2];
                c[i][k] += m_c[3];
                c[i][k] += m_c[4];
                c[i][k] += m_c[5];
                c[i][k] += m_c[6];
                c[i][k] += m_c[7];
            }

        }

    }
}

int algorithm3(ARRAY_TYPE** m1, ARRAY_TYPE** m2, ARRAY_TYPE** r, int x1, int y1, int x2, int y2){
    for (int i = 0; i < x1; ++i) {
        for (int j = 0; j < y2; ++j) {
            r[i][j] = 0;
        }
    }

    for (int i = 0; i < x1; ++i) {
        for (int k = 0; k < y1; ++k){
            for (int j = 0; j < y2; ++j) {
                r[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }


    return 0;
}