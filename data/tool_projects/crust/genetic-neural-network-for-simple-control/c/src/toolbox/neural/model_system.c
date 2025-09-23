#include "neural/model_system.h"

#include "neural/neural_network.h"
#include "general/signal_designer.h"
#include "general/matrix_math.h"


#include <float.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <unistd.h>

void createNNSystem(struct SystemNN *systemNN, struct NNInput *input){

  // input system 
  selectInputNNFunction(&systemNN->input_sys, systemNN);

  // make input size be the same as the input system needs
  input->neuronsSize[0] = systemNN->inputDataSize[2] - 1;

  // now empty de_norm are created
  input->normalizationMatrix   = (float**)malloc(2 * sizeof(float*));
  input->denormalizationMatrix = (float**)malloc(2 * sizeof(float*));

  input->normalizationMatrix[0]   = (float*)malloc(input->neuronsSize[0] * sizeof(float));
  input->normalizationMatrix[1]   = (float*)malloc(input->neuronsSize[0] * sizeof(float));

  input->denormalizationMatrix[0] = (float*)malloc(sizeof(float));
  input->denormalizationMatrix[1] = (float*)malloc(sizeof(float));

  // first NN is created
  systemNN->neuralNetwork = (struct NN*)malloc(sizeof(struct NN));
  createNeuralNetwork(input, systemNN->neuralNetwork);

  // select signal and system
  systemNN->signal = (struct Signal *)malloc(sizeof(struct Signal));
  
  cliSignalSelector(systemNN->signal);

  int size = selectSystem(&systemNN->func_system);
    
  systemNN->dataSystem = (float*)malloc(size * sizeof(float));
  for(int i=0; i<size; i++){
      systemNN->dataSystem[i] = 0.0;
  }
  systemNN->sizeDataSystem = size;

  // set the output signal
  systemNN->output = (struct Signal*)malloc(sizeof(struct Signal));
  systemNN->output->length = systemNN->signal->length;
  systemNN->output->dt = systemNN->signal->dt;
  systemNN->output->signal = (float*)malloc(systemNN->signal->length * sizeof(float));

  
}

void clearNNSystem(struct SystemNN *systemNN){
  clearNeuralNetwork(systemNN->neuralNetwork);

  deleteSignal(systemNN->signal);
  deleteSignal(systemNN->output);

  free(systemNN->dataSystem);
  free(systemNN->inputData);
  free(systemNN->inputDataSize);

  free(systemNN);
}

static void cleanNNSystem(struct SystemNN *systemNN){
  // clear output and system data
  for(int i=0; i < systemNN->signal->length; i++){
      systemNN->output->signal[i] = 0.0;
  }
  for(int i=0; i<systemNN->sizeDataSystem; i++){
      systemNN->dataSystem[i] = 0.0;
  }
  systemNN->dataSystem[1] = systemNN->signal->dt;

  // clear input data
  for(int i=0; i<systemNN->inputDataSize[0]; i++){
      systemNN->inputData[i] = 0.0;
  }
  systemNN->inputData[0] = systemNN->signal->dt;

  systemNN->steadyRiseCheck = 0;
  systemNN->maxCounter = 0;

  systemNN->fit = 0.0;
}

static void getMaxMinSignalValues(struct Signal *signal, float *max, float *min){
  *max = 0.0; // max
  *min = 0.0; // min of system

  for(int i=0; i<signal->length; i++){
    if(signal->signal[i] > *max){
      *max = signal->signal[i];
    } else if (signal->signal[i] < *min){
      *min = signal->signal[i];
    }
  }
}

static void findUOneRound(struct SystemNN *systemNN, float *forValues, float *Data){
  float analyzedValue, maxY, maxDDY, maxE, maxDY; 
  float finValue, sysOutput, dy, ddy, iy, preY, preDY;
  float du, ddu, iu, preU, preDU, min, penalty;
  float de, dde, ie, preE, preDE, maxDE, maxDDE, sum;
  int count;

  float maxSig, minSig;

  min = FLT_MAX;
  getMaxMinSignalValues(systemNN->signal, &maxSig, &minSig);

  for(float curValue = forValues[0]; curValue < forValues[1]; curValue += forValues[2]){
    // get all max values set
    maxY   = 0;
    maxDY  = 0;
    maxDDY = 0;

    maxE   = 0;
    maxDE  = 0;
    maxDDE = 0;

    // all aditional 
    count = 0;
    preY  = 0;
    preDY = 0;
    iy    = 0;

    preU  = 0;
    preDU = 0;
    iu    = 0;

    preE  = 0;
    preDE = 0;
    ie    = 0;

    penalty = 0;
    sum = 0;

    // first the data of the system is reseted
    for(int j=0; j<systemNN->sizeDataSystem; j++){
        systemNN->dataSystem[j] = 0.0;
    }
    systemNN->dataSystem[1] = systemNN->signal->dt;

    // now the idea is to simulate 0.5s with the u set to curValue, finnal value is then analyzed
    for(float j=0.0; j< systemNN->signal->length; j += systemNN->signal->dt){
      if(j < 1.0){
        systemNN->dataSystem[0] = 0;
      } else {
        systemNN->dataSystem[0] = curValue;
      }
      sysOutput = systemNN->func_system(systemNN->dataSystem);

      if(sysOutput > systemNN->maxSys){
        penalty = 50000;
      } else if(sysOutput < systemNN->minSys){
        penalty = 50000;
      }

      dy  = (sysOutput - preY ) / systemNN->output->dt;
      ddy = (dy        - preDY) / systemNN->output->dt;

      de  = (maxSig - sysOutput  - preE ) / systemNN->output->dt;
      dde = (de                  - preDE) / systemNN->output->dt;   

      preY  = sysOutput;
      preDY = dy;

      preE  = maxSig - sysOutput;
      preDE = de;

      if(sysOutput > maxY  ) { maxY   = sysOutput; } 
      if(preE      > maxE  ) { maxE   = preE;      } 
      if(dy        > maxDY ) { maxDY  = dy;        } 
      if(ddy       > maxDDY) { maxDDY = ddy;       }
      if(de        > maxDE ) { maxDE  = de;        }
      if(dde       > maxDDE) { maxDDE = dde;       }
  
      iy += sysOutput;
      iu += curValue;
      ie += preE;

      if(j>systemNN->signal->length - 1.0){
        sum += sysOutput;
        count++;
      }
      
    }
    analyzedValue = maxSig - (sum / count) + penalty;
    if(analyzedValue < 0){
      analyzedValue *= (-1);
    }

    if(iy < 0){
      iy *= (-1);
    } if(iu < 0){
      iu *= (-1);
    } if(ie < 0){
      ie *= (-1);
    }

    if(analyzedValue < min){
      min = analyzedValue;

      Data[0] = maxE;
      Data[1] = curValue;
      Data[2] = maxY;

      Data[3] = maxDE;
      Data[4] = maxDDE;
      Data[5] = ie;

      Data[6] = curValue / systemNN->output->dt;
      Data[7] = Data[7]  / systemNN->output->dt;
      Data[8] = iu;

      Data[9]  = maxDY;
      Data[10] = maxDDY;
      Data[11] = iy;
    }
  }
}

static void findUForSystemAndSignal(struct SystemNN *systemNN, float *Data){

  float forValueOne[]  = {0.0, 10000.0, 1000.0};
  findUOneRound(systemNN, &forValueOne, Data);

  float forValueTwo[]  = {Data[1] - 100.0, Data[1] + 100.0, 10.0};
  findUOneRound(systemNN, &forValueTwo, Data);

  float forValueThree[] = {Data[1] - 10.0, Data[1] + 10.0, 1.0};
  findUOneRound(systemNN, &forValueThree, Data);
}

static float makeDerivation(float *x, float *x_t, float *dt){
  return (*x - *x_t) / *dt;
}

void createDeNormalization(struct SystemNN *systemNN){
  // first all generic normalizations are created into array and then the NN normalization is filled
  // the filling is controlled by the inputTypes

  float **normalization = (float**)malloc(2 * sizeof(float*));

  normalization[0] = (float*)malloc(12 * sizeof(float)); // create max
  normalization[1] = (float*)malloc(12 * sizeof(float)); // create min

  // get min and max of function
  float *Data = (float*)malloc(12 * sizeof(float));

  findUForSystemAndSignal(systemNN, Data);

  // e 
  normalization[0][0]  = Data[0];
  normalization[1][0]  = Data[0] * (-1);
  printf("E - %f\n", Data[0]);

  // u
  normalization[0][1]  = Data[1];
  normalization[1][1]  = Data[1] * (-1);
  printf("U - %f\n", Data[1]);

  // y
  normalization[0][2]  = Data[2];
  normalization[1][2]  = Data[2] * (-1);
  printf("Y - %f\n", Data[2]);

  // de 
  normalization[0][3]  = Data[3];        // bigest possible result
  normalization[1][3]  = Data[3] * (-1); // smallest possible result
  printf("dE - %f\n", Data[3]);

  // dde 
  normalization[0][4]  = Data[4];        // bigest possible result
  normalization[1][4]  = Data[4] * (-1); // smallest possible result
  printf("dE2 - %f\n", Data[4]);

  // ie
  normalization[0][5]  = Data[5];        
  normalization[1][5]  = Data[5] * (-1); 
  printf("iE - %f\n", Data[5]);

  // du 
  normalization[0][6]  = Data[6];        // bigest possible result
  normalization[1][6]  = Data[6] * (-1); // smallest possible result
  printf("dU - %f\n", Data[6]);

  // ddu 
  normalization[0][7]  = Data[7];        // bigest possible result
  normalization[1][7]  = Data[7] * (-1); // smallest possible result
  printf("dU2 - %f\n", Data[7]);

  // iu
  normalization[0][8]  = Data[8];        
  normalization[1][8]  = Data[8] * (-1); 
  printf("iU - %f\n", Data[8]);

  // dy
  normalization[0][9]  = Data[9];        // bigest possible result
  normalization[1][9]  = Data[9] * (-1); // smallest possible result
  printf("dY - %f\n", Data[9]);

  // ddy 
  normalization[0][10] = Data[10];        // bigest possible result
  normalization[1][10] = Data[10] * (-1); // smallest possible result
  printf("dY2 - %f\n", Data[10]);

  // iy
  normalization[0][11] = Data[11];        
  normalization[1][11] = Data[11] * (-1); 
  printf("iY - %f\n", Data[11]);

  // 10 * e
  // for(int i=0; i<12; i++){
  //   normalization[0][i] *= 2.0;
  //   if(normalization[1][i] > 0){
  //     normalization[1][i] /= 2.0;
  //   } else{
  //     normalization[1][i] *= 2.0; 
  //   }
  // }
  normalization[0][1] *= 10.0;
  normalization[1][1] *= 10.0; 
  

  // now the normalization values are added to the de_normalization matrix of NN
  for(int i = 0; i < systemNN->inputDataSize[2] - 1; i++){
    systemNN->neuralNetwork->normalizationMatrix[0][i] = normalization[0][systemNN->inputTypes[i]];
    systemNN->neuralNetwork->normalizationMatrix[1][i] = normalization[1][systemNN->inputTypes[i]];
    printf("%d - MAX: %f MIN: %f\n", i, systemNN->neuralNetwork->normalizationMatrix[0][i], systemNN->neuralNetwork->normalizationMatrix[1][i]);
  }

  systemNN->neuralNetwork->denormalizationMatrix[0][0] = normalization[0][1];
  systemNN->neuralNetwork->denormalizationMatrix[1][0] = normalization[1][1];

  free(normalization[0]);
  free(normalization[1]);
  free(normalization);
  free(Data);

  // the inputTypes will not be needed, so the array can be free
  free(systemNN->inputTypes);
}

void makeSimulationOfSignalNN(struct SystemNN *systemNN, FILE *csvFile, int csv){
  cleanNNSystem(systemNN);
  clearSDMemory(systemNN->neuralNetwork);
  
  float error, diff;
  float neuralOutput = 0.0;
  float systemOutput = 0.0;

  int matrixIndex;

  float max = 0.0;

  for(int i=1; i<systemNN->signal->length; i++){

    error = systemNN->signal->signal[i] - systemNN->output->signal[i-1];

    // now the input for the NN is generated
    systemNN->inputData[1] = error;        // e
    systemNN->inputData[2] = neuralOutput; // u
    systemNN->inputData[3] = systemOutput; // y

    systemNN->input_sys(systemNN->inputData);

    // now the input matrix is created and filled
    struct Matrix *inputMatrix  = (struct Matrix*)malloc(sizeof(struct Matrix));
    struct Matrix *outputMatrix = (struct Matrix*)malloc(sizeof(struct Matrix));

    int *inputSize = (int *)malloc(2 * sizeof(int));
    inputSize[0] = systemNN->neuralNetwork->neuronsSize[0];
    inputSize[1] = 1;

    createMatrix(inputMatrix, inputSize);

    matrixIndex = 0;
    for(int j=systemNN->inputDataSize[1]; j<systemNN->inputDataSize[2]; j++){
      inputMatrix->matrix[matrixIndex][0] = systemNN->inputData[j];
      matrixIndex++;
    }
    // now the matrix calculation can be made
    oneCalculation(systemNN->neuralNetwork, inputMatrix, outputMatrix);

    if(outputMatrix->matrix[0][0] > max){
      max = outputMatrix->matrix[0][0];
    }

    systemNN->dataSystem[0] = outputMatrix->matrix[0][0]; // the values of the calculation
    neuralOutput = systemNN->dataSystem[0]; // the next round u is set for input

    // set data and pass to system
    systemNN->output->signal[i] = systemNN->func_system(systemNN->dataSystem);

    if(systemNN->output->signal[i] > systemNN->maxSys){
        systemNN->output->signal[i] = systemNN->maxSys;
        systemNN->maxCounter++;
    } else if(systemNN->output->signal[i] < systemNN->minSys){
        systemNN->output->signal[i] = systemNN->minSys;
        systemNN->maxCounter++;
    }
    systemOutput = systemNN->output->signal[i]; // y is set for next round input

    if(csv == 1){
      fprintf(csvFile, "%f,%f,%f,%f\n", systemNN->output->signal[i], systemNN->signal->signal[i], outputMatrix->matrix[0][0], systemNN->output->signal[i]);
    } 

    diff = fabs(systemNN->signal->signal[i] - systemNN->output->signal[i]);
    systemNN->fit += diff;

    if(systemNN->output->signal[i-1] > systemNN->output->signal[i]){
        systemNN->steadyRiseCheck++;
    }

    matrixDelete(outputMatrix);
  }
  if(csv == 1){
    printf("%f\n", max);
  }

}