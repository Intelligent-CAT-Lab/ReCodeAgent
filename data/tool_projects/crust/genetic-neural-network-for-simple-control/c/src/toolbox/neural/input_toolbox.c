#include "neural/input_toolbox.h"

#include "neural/model_system.h"

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

void typeOne(float *data) {

  // data[0] - dt

  // data[1] - e[t]
  // data[2] - u[t]
  // data[3] - y[t]

  // data[4] -  de[t]
  // data[5] -  ie[t]
  // data[6] -  dy[t]
  // data[7] - ddy[t]
  // data[8] -  du[t]

  // data[9]  -  e[t-1]
  // data[10] -  u[t-1]
  // data[11] -  y[t-1]
  // data[12] - dy[t-1]

  // e
  data[4]  = (data[1] - data[9])/data[0];
  data[5] +=  data[1];
  data[9]  =  data[1];

  // u
  data[8]  = (data[2] - data[10])/data[0];
  data[10] =  data[2];

  // y
  data[6]  = (data[3] - data[11])/data[0];
  data[7]  = (data[6] - data[12])/data[0];
  data[11] =  data[3];
  data[12] =  data[6];
}

void typeTwo(float *data) {

  // data[0] - dt

  // data[1] - e[t]
  // data[2] - u[t]
  // data[3] - y[t]

}

static void makeInputDataSystem(struct SystemNN *systemNN, int size, int full){
  systemNN->inputData = (float*)malloc(full * sizeof(float));
  for(int i=0; i<full; i++){
      systemNN->inputData[i] = 0.0;
  }

  systemNN->inputDataSize = (int*)malloc(3 * sizeof(int));
  systemNN->inputDataSize[0] = full;
  systemNN->inputDataSize[1] = 1;
  systemNN->inputDataSize[2] = size + 1;

  systemNN->inputTypes    = (int*)malloc(size * sizeof(int));
  systemNN->inputTypes[0] = 0;  // e
  systemNN->inputTypes[1] = 1;  // u
  systemNN->inputTypes[2] = 2;  // y
  systemNN->inputTypes[3] = 4;  // de
  systemNN->inputTypes[4] = 5;  // ie
  systemNN->inputTypes[5] = 9;  // dy
  systemNN->inputTypes[6] = 10; // ddy
  systemNN->inputTypes[7] = 6;  // du
}

static void makeInputDataSystemTwo(struct SystemNN *systemNN, int size, int full){
  systemNN->inputData = (float*)malloc(full * sizeof(float));
  for(int i=0; i<full; i++){
      systemNN->inputData[i] = 0.0;
  }

  systemNN->inputDataSize = (int*)malloc(3 * sizeof(int));
  systemNN->inputDataSize[0] = full;
  systemNN->inputDataSize[1] = 1;
  systemNN->inputDataSize[2] = size + 1;

  systemNN->inputTypes    = (int*)malloc(size * sizeof(int));
  systemNN->inputTypes[0] = 0;  // e
  systemNN->inputTypes[1] = 1;  // u
  systemNN->inputTypes[2] = 2;  // y
}

void selectInputNNFunction(void (**func_ptr)(float*), struct SystemNN *systemNN){
    printf("Please select the AF:\n1 - typeOne\n");
    printf("2 - typeTwo (SD)\n");
    printf("Select: ");
    int userChoice;
    scanf("%d", &userChoice);

    if (userChoice == 1) {
        *func_ptr = typeOne;
        makeInputDataSystem(systemNN, 8, 13);
    } else if(userChoice == 2){
      *func_ptr = typeTwo;
      makeInputDataSystemTwo(systemNN, 3, 4);
    }
}