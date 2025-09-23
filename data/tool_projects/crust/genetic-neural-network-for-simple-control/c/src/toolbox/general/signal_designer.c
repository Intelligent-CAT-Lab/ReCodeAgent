#include "general/signal_designer.h"

#include <stdio.h>
#include <stdlib.h>

// the time step constant
#define DT001 0.001; 
#define DT01  0.01; 

static void selectStepSignal(struct Signal *signal){
    signal->length = 1000;
    signal->dt     = DT01;

    signal->signal = (float*)malloc(signal->length * sizeof(float));   
    int globalIndex = 0;
    for(float i = 0.0; i < 0.3; i += signal->dt){
        signal->signal[globalIndex] = 0.0;
        globalIndex++;
    } for(float i = 0.3 + signal->dt; i < 10.0; i += signal->dt){
        signal->signal[globalIndex] = 1.0;
        globalIndex++;
    }
    signal->length = globalIndex;
}

static void selectCustomASignal(struct Signal *signal){
    signal->length = 1002;
    signal->dt     = DT01;

    signal->signal = (float*)malloc(signal->length * sizeof(float));   
    int globalIndex = 0;
    for(float i = 0.0; i < 1.0; i += signal->dt){
        signal->signal[globalIndex] = 0.0;
        globalIndex++;
    } for(float i = 1.0; i < 3.0; i += signal->dt){
        signal->signal[globalIndex] = 30.0;
        globalIndex++;
    } for(float i = 3.0; i < 5.0; i += signal->dt){
        signal->signal[globalIndex] = 15.0;
        globalIndex++;
    } for(float i = 5.0; i < 8.0; i += signal->dt){
        signal->signal[globalIndex] = 5.0;
        globalIndex++;
    } for(float i = 8.0; i < 9.0; i += signal->dt){
        signal->signal[globalIndex] = 25.0;
        globalIndex++;
    } for(float i = 9.0; i < 10.0; i += signal->dt){
        signal->signal[globalIndex] = 0.0;
        globalIndex++;
    }   
    signal->length = globalIndex;
}


void deleteSignal(struct Signal *signal){
    free(signal->signal);
    free(signal);
}

void cliSignalSelector(struct Signal *signal){
    printf("Please select the Signal:\n");
    printf("1 - step\n");
    printf("2 - customa\n");
    printf("Select: ");
    int userChoice;
    scanf("%d", &userChoice);

    if (userChoice == 1) {
        selectStepSignal(signal);
    } else if(userChoice == 2){
        selectCustomASignal(signal);
    }
}