# Matrix Multiplication

## how to write the project?

1. Create `CMakeLists.txt`
2. create the main file
3. generate matrix and write test
4. write different algorithms

## algorithms

### Traditional

The simplest solution anyone can think of.

```c

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
```

### memory friendly
We did a bit of research  and found out that if we change the 
loop order it keeps the correctness while the speed increase because 
cache hit increases.

```c
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
```

### SIMD

The difference between the algorithm written below and the algorithm in matrix is that we use the loop unrolling method in the latter to reduce the number of loops.

```c
int algorithm2( ARRAY_TYPE **a,  ARRAY_TYPE **bb, ARRAY_TYPE **c, int size){
    ARRAY_TYPE** b = malloc(sizeof(ARRAY_TYPE*) * size);
    for (int i = 0; i < size; ++i) {
        b[i] = malloc(sizeof(ARRAY_TYPE) * size);
        for (int j = 0; j < size; j++) {
            b[i][j] = bb[j][i];
            c[i][j] = 0;
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
            }

        }

    }
}
```

## Result

```
method: 1 total consume: 3338 ms
method: 1 total time_taken only multiply: 2450 ms
================
method: 2 total consume: 1718 ms
method: 2 total time_taken only multiply: 897 ms
================
method: 3 total consume: 2261 ms
method: 3 total time_taken only multiply: 1426 ms
================
```

what is the difference between "total consume" and "total time taken only multiply"?
The last one only take time taken by the process of multiplication. In order to perform multiple tests, we need to do the same preparation each time, and these tasks are time-consuming, but these times should not be counted in the time required for multiplication. Ideally, one preparation will be performed multiple times. For example, instead of creating a matrix for each operation, use a pre-generated matrix.

## Tests

I wrote simply test functions to test them. These functions can be found in tests directory.

## Implement
The currently implemented SIMD algorithm can only operate on square matrices whose side length is a multiple of 8. In the future, it will be possible to perform multiplication operations on arbitrary matrices.

## Issues

In order to show the excellence of the SIMD algorithm, it is necessary to specify `-O1` or above in Cmake. Is `-O0` the default in cmake?

