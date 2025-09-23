//
// Created by ruibin on 12/12/22.
//


#include <bits/time.h>
#include <time.h>
#include <stdio.h>
#include "matrix.h"
#include "utils.h"
#define ARRAY_TYPE_CHAR "float"
int test_one(int x1, int y1, int x2, int y2, int method, char* test_name, long *time_taken){
    ARRAY_TYPE **matrix1, **matrix2, **result;
    generate_matrix(&matrix1, x1, y1, true);
    generate_matrix(&matrix2, x2, y2, true);
    generate_matrix(&result, x1, y2, true);


    struct timespec lastSentTime, sentTime;
    clock_gettime(CLOCK_REALTIME, &lastSentTime);

    multiply(matrix1, matrix2, result, x1, y1, x2, y2, method);
    clock_gettime(CLOCK_REALTIME, &sentTime);

    long time = (long )(sentTime.tv_sec - lastSentTime.tv_sec) * 1000000000;
    time += (sentTime.tv_nsec - lastSentTime.tv_nsec);
//    time /= 1000000;
    *time_taken += time;
//    printf("method: %d consume: %ld ms\n", method, time);


    free_matrix(&matrix1, x1);
    free_matrix(&matrix2, x2);
    free_matrix(&result, x1);
    return 0;
}
int test_more(int x1, int y1, int x2, int y2, int method, char* test_name, int times){
    long time_taken = 0;
    struct timespec lastSentTime, sentTime;
    clock_gettime(CLOCK_REALTIME, &lastSentTime);
    for (int i = 0; i < times; ++i) {
        test_one(x1,  y1,  x2,  y2,  method,  test_name, &time_taken);
    }
    clock_gettime(CLOCK_REALTIME, &sentTime);
    long time = (long )(sentTime.tv_sec - lastSentTime.tv_sec) * 1000000000;
    time += (sentTime.tv_nsec - lastSentTime.tv_nsec);
    time /= 1000000;
    printf("method: %d total consume: %ld ms\n", method, time);
    printf("method: %d total time_taken only multiply: %ld ms\n================\n", method, time_taken/1000000);
    return 0;
}
int main(){
    char* test_name = "method: 1";
    int x1 = 128, x2 = 128;
    int y1 = 128, y2 = 128;
    int times = 1000;

    int method = 1;
    test_more( x1,  y1,  x2,  y2,  method, test_name, times);


    /*Note: algorithm 2 only can be used when the two matrix's share are same.*/
    method = 2;
    test_name = "method: 2";
    test_more( x1,  y1,  x2,  y2,  method, test_name, times);

    method = 3;
    test_name = "method: 3";
    test_more( x1,  y1,  x2,  y2,  method, test_name, times);
}