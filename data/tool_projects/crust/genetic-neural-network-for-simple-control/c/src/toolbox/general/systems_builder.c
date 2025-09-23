#include "general/systems_builder.h"
#include "general/pid_controller.h"

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

float linear(float *data){
    // in casse of linear data is returned as it is
    return data[0];
}

float complexYDot(float *data){
    // in this case
    // 0 - u
    // 1 - dt
    // 2 - y
    // 3 - dot_y
    
    data[2] = data[2] + data[0] * data[1];

    return data[2];
}


float complexYDddot(float *data){
    // in this case data is:
    // 0 - u
    // 5 - dddot_y
    // 2 - ddot_y
    // 3 - dot_y
    // 4 - y
    // 1 - dt
    // the u and dt should have this indexes, as PID do not know the system

    data[5] = data[0] - 2 * data[2] - 3 * data[3] - 4 * data[4]; // y_dddot = (u(i) - a*y_ddot(i-1) - b*y_dot(i-1) - c*y(i-1))

    data[2] = data[2] + data[5] * data[1]; // y_ddot(i) = y_ddot(i-1) + y_dddot   * (t(i) - t(i-1));
    data[3] = data[3] + data[2] * data[1]; // y_dot(i)  = y_dot(i-1)  + y_ddot(i) * (t(i) - t(i-1));
    data[4] = data[4] + data[3] * data[1]; // y(i)      = y(i-1)      + y_dot(i)  * (t(i) - t(i-1));
    return data[4];
}

int selectSystem(float (**func_ptr)(float*)){
    printf("Please select the system:\n");
    printf("1 - linear\n");
    printf("2 - complexYDddot\n");
    printf("3 - complexYDot\n");
    printf("Select: ");
    int userChoice;
    scanf("%d", &userChoice);

    if (userChoice == 1) {
        *func_ptr = linear;
        return 2;
    } else if (userChoice == 2){
        *func_ptr = complexYDddot;
        return 6;
    } else if (userChoice == 3){
        *func_ptr = complexYDot;
        return 4;
    } else{
        exit(0);
    }
}