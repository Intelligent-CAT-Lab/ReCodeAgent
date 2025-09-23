#include "neural/activation_fnc.h"

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

float tangenth(float x){
    return tanh(5*x);
}

float sigmoid(float x) {
    return 1.0 / (1.0 + exp(-x));
}

void selectTangActivationFunction(float (**func_ptr)(float)){
    *func_ptr = tangenth;
}

void selectSigmActivationFunction(float (**func_ptr)(float)){
    *func_ptr = sigmoid;
}

void selectActivationFunction(float (**func_ptr)(float)){
    printf("Please select the AF:\n1 - tanh\n2 - sigmoid\nSelect: ");
    int userChoice;
    scanf("%d", &userChoice);

    if (userChoice == 1) {
        *func_ptr = tangenth;
    } else if (userChoice == 2) {
        *func_ptr = sigmoid;
    }
}