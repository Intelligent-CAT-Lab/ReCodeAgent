//
// Created by ruibin on 12/12/22.
//

#include <stdio.h>
#include <string.h>



int print_float_array(float ** matrix, int x, int y){
    for (int i = 0; i < x; ++i) {
        for (int j = 0; j < y; ++j) {
            printf("%10.2f ", matrix[i][j]);
        }
        printf("\n");
    }
    return 0;
}


int print_array(float **matrix, int x, int y, char* name, char* type){
    printf("-------------%s--------------\n", name);
    if(!strcmp(type, "float")){
        print_float_array(matrix, x, y);
    }else{
        printf("unsupported %s", type);
    }
    return 0;
}
