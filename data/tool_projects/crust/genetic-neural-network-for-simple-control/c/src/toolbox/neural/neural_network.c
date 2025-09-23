#include "neural/neural_network.h"

#include "genetic/population.h"
#include "general/sort.h"
#include "general/matrix_math.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

static void clearNNInput(struct NNInput *input){
  free(input->neuronsSize);

  // free de_normalization matrixes
  for(int i = 0; i < 2; i++){
    free(input->normalizationMatrix[i]);
    free(input->denormalizationMatrix[i]);
  }
  free(input->normalizationMatrix);
  free(input->denormalizationMatrix);

  // free the layer type matrix
  free(input->layerType);

  // delete full strcture
  free(input);
}

void createNeuralNetwork(struct NNInput *input, struct NN *neuralNetwork) {
  // create list of neurons sizes
  neuralNetwork->layerNumber = input->layerNumber;
  neuralNetwork->neuronsSize = (int*)malloc(input->layerNumber * sizeof(int));
  memcpy(neuralNetwork->neuronsSize, input->neuronsSize, neuralNetwork->layerNumber * sizeof(int));

  // copy all layer info and neurons type
  neuralNetwork->layerType = (int*)malloc(neuralNetwork->layerNumber * sizeof(int));
  memcpy(neuralNetwork->layerType, input->layerType, neuralNetwork->layerNumber * sizeof(int));

  neuralNetwork->sdNeuronsTypes = (int**)malloc(input->sdNumber * sizeof(int*));
  neuralNetwork->SDMemory = (struct Matrix **)malloc(input->sdNumber * sizeof(struct Matrix *));

  // here the SD memory is allocated
  int globalIndexSD = 0;
  for(int i=0; i<neuralNetwork->layerNumber; i++){
    if(neuralNetwork->layerType[i] == 1){
      // first the sdNeuronsTypes is created to hold matrix of types of each neuron for future calculations
      // the logis is: 50% straight links, 25% S links and 25% D links
      neuralNetwork->sdNeuronsTypes[globalIndexSD] = (int*)malloc(neuralNetwork->neuronsSize[i] * sizeof(int));

      for(int j=0; j<neuralNetwork->neuronsSize[i]/2; j++){
        neuralNetwork->sdNeuronsTypes[globalIndexSD][j] = 0; // straight links
      }
      int midIndexSlice = neuralNetwork->neuronsSize[i]/2 + (neuralNetwork->neuronsSize[i] - neuralNetwork->neuronsSize[i]/2)/2;
      for(int j=neuralNetwork->neuronsSize[i]/2; j < midIndexSlice; j++){
        neuralNetwork->sdNeuronsTypes[globalIndexSD][j] = 1; // s links
      } for(int j=midIndexSlice; j<neuralNetwork->neuronsSize[i]; j++){
        neuralNetwork->sdNeuronsTypes[globalIndexSD][j] = 2; // d link
      }
      
      // now the memory matrix is created for future usage in caculations
      neuralNetwork->SDMemory[globalIndexSD] = (struct Matrix *)malloc(sizeof(struct Matrix));
      int *sizesSDMatrix = (int*)malloc(2*sizeof(int));

      sizesSDMatrix[0] = neuralNetwork->neuronsSize[i];
      sizesSDMatrix[1] = 1;

      createMatrix(neuralNetwork->SDMemory[globalIndexSD], sizesSDMatrix);

      // now all matrix is set to .0 to not crash first calculations
      for(int j=0; j<neuralNetwork->neuronsSize[i]; j++){
        neuralNetwork->SDMemory[globalIndexSD]->matrix[j][0] = .0;
      }
      globalIndexSD++;
    }
  }

  // create de/normalization matrixes of the system
  neuralNetwork->normalizationMatrix   = (float**)malloc(2 * sizeof(float*));
  neuralNetwork->denormalizationMatrix = (float**)malloc(2 * sizeof(float*));

  for(int i = 0; i < 2; i++){
    neuralNetwork->normalizationMatrix[i]   = (float*)malloc(neuralNetwork->neuronsSize[0] * sizeof(float));
    neuralNetwork->denormalizationMatrix[i] = (float*)malloc(neuralNetwork->neuronsSize[neuralNetwork->layerNumber - 1] * sizeof(float));
    
    memcpy(neuralNetwork->normalizationMatrix[i],   input->normalizationMatrix[i],   neuralNetwork->neuronsSize[0] * sizeof(float));
    memcpy(neuralNetwork->denormalizationMatrix[i], input->denormalizationMatrix[i], neuralNetwork->neuronsSize[neuralNetwork->layerNumber - 1] * sizeof(float));
  }

  selectActivationFunction(&neuralNetwork->func_ptr);

  // delete input structure
  clearNNInput(input);

  neuralNetwork->AW = (struct Matrix **)malloc((neuralNetwork->layerNumber - 1) * sizeof(struct Matrix *));
  neuralNetwork->BW = (struct Matrix **)malloc((neuralNetwork->layerNumber - 1) * sizeof(struct Matrix *));

  // create size for all matrixes
  int layerIndex = 0;
  neuralNetwork->countOfValues = 0; 

  for(int i=0; i<neuralNetwork->layerNumber - 1; i++){
    neuralNetwork->AW[i] = (struct Matrix*)malloc(sizeof(struct Matrix));
    neuralNetwork->BW[i] = (struct Matrix*)malloc(sizeof(struct Matrix));

    int *sizesAW = (int*)malloc(2*sizeof(int));
    int *sizesBW = (int*)malloc(2*sizeof(int));

    sizesAW[0] = neuralNetwork->neuronsSize[layerIndex + 1];
    sizesAW[1] = neuralNetwork->neuronsSize[layerIndex];

    sizesBW[0] = neuralNetwork->neuronsSize[layerIndex + 1];
    sizesBW[1] = 1;

    // add number of genes needed
    neuralNetwork->countOfValues += neuralNetwork->neuronsSize[layerIndex + 1] * neuralNetwork->neuronsSize[layerIndex] + neuralNetwork->neuronsSize[layerIndex + 1];
    createMatrix(neuralNetwork->AW[i], sizesAW);
    createMatrix(neuralNetwork->BW[i], sizesBW);

    layerIndex += 1;
  }
  neuralNetwork->countOfValues -= neuralNetwork->neuronsSize[layerIndex];

  // now last bias matrix should be set to 0. It is only created to make math part easier
  int indexBWLast = neuralNetwork->layerNumber - 2;
  for(int i=0; i<neuralNetwork->BW[indexBWLast]->sizes[0]; i++){
    for(int j=0; j<neuralNetwork->BW[indexBWLast]->sizes[0]; j++){
      neuralNetwork->BW[indexBWLast]->matrix[i][j] = 0.0;
    }
  }
}

void clearNeuralNetwork(struct NN *neuralNetwork) {
  // free matrixes
  for(int i=0; i<neuralNetwork->layerNumber - 1; i++){
     matrixDelete(neuralNetwork->AW[i]);
     matrixDelete(neuralNetwork->BW[i]);
  }
  free(neuralNetwork->AW);
  free(neuralNetwork->BW);

  // free de_normalization matrixes
  for(int i = 0; i < 2; i++){
    free(neuralNetwork->normalizationMatrix[i]);
    free(neuralNetwork->denormalizationMatrix[i]);
  }
  free(neuralNetwork->normalizationMatrix);
  free(neuralNetwork->denormalizationMatrix);

  // free last pointer
  free(neuralNetwork->neuronsSize);

  // free all sd memory
  int indexSD  = 0;
  for(int i=0; i<neuralNetwork->layerNumber; i++){
    if(neuralNetwork->layerType[i] == 1){
      free(neuralNetwork->sdNeuronsTypes[indexSD]);
      matrixDelete(neuralNetwork->SDMemory[indexSD]);
      indexSD++;
    }
  }
  free(neuralNetwork->sdNeuronsTypes);
  free(neuralNetwork->SDMemory);
  free(neuralNetwork->layerType);

  // delete full structure
  free(neuralNetwork);
}

void fillMatrixesNN(struct NN *neuralNetwork, float *population){
  int globalIndex = 0;
  
  for(int i=0; i<neuralNetwork->layerNumber - 1; i++){
    // first the AW matrix is set
    //printf("=====Matrix AW[%d]=====\n", i);
    for(int x=0; x<neuralNetwork->AW[i]->sizes[0]; x++){
      for(int y=0; y<neuralNetwork->AW[i]->sizes[1]; y++){
        neuralNetwork->AW[i]->matrix[x][y] = population[globalIndex];
        //printf("%f ", neuralNetwork->AW[i]->matrix[x][y]);
        globalIndex++;
      }
      //printf("\n");
    }

    
    // second the BW is set
    if( i < neuralNetwork->layerNumber - 2){
      //printf("=====Matrix BW[%d]=====\n", i);
      for(int x=0; x<neuralNetwork->BW[i]->sizes[0]; x++){
        for(int y=0; y<neuralNetwork->BW[i]->sizes[1]; y++){
          neuralNetwork->BW[i]->matrix[x][y] = population[globalIndex];
          //printf("%f ", neuralNetwork->BW[i]->matrix[x][y]);
          globalIndex++;
        }
        //printf("\n");
      }
    }
  }
}

void clearSDMemory(struct NN *neuralNetwork){
  int globalIndex = 0;
  // go though layers to find SD ones
  for(int i=0; i<neuralNetwork->layerNumber; i++){
    if(neuralNetwork->layerType[i] == 1){
      // if layer SD then clear matrix
      for(int j=0; j<neuralNetwork->neuronsSize[i]; j++){
        neuralNetwork->SDMemory[globalIndex]->matrix[j][0] = .0;
      }
      globalIndex++;
    }
  }
}

void deNormalizationProcess(struct NN *neuralNetwork, struct Matrix *input, int way){
  // way -> 0 - normalize
  // way -> 1 - de_normalize
  float rMin, rMax, tMin, tMax;
  for(int i=0; i< input->sizes[0]; i++){
    if(way == 0){
      rMin = neuralNetwork->normalizationMatrix[1][i];
      rMax = neuralNetwork->normalizationMatrix[0][i];
      tMin = -1.0;
      tMax =  1.0;
    } else if(way == 1){
      rMin = -1.0;
      rMax =  1.0;
      tMin = neuralNetwork->denormalizationMatrix[1][i];
      tMax = neuralNetwork->denormalizationMatrix[0][i];
    }
    
    input->matrix[i][0] = ((input->matrix[i][0] - rMin)/(rMax - rMin)) * (tMax - tMin) + tMin;
    
    if(way == 0){
      if(input->matrix[i][0] > 1.0){
        input->matrix[i][0] = 1.0;
      } else if(input->matrix[i][0] < -1.0){
        input->matrix[i][0] = -1.0;
      }
    } else if (way == 1){
      if(input->matrix[i][0] > neuralNetwork->denormalizationMatrix[0][i]){
        input->matrix[i][0] = neuralNetwork->denormalizationMatrix[0][i];
      } else if(input->matrix[i][0] < neuralNetwork->denormalizationMatrix[1][i]){
        input->matrix[i][0] = neuralNetwork->denormalizationMatrix[1][i];
      }
    }
  }
}

static void makeSDLayerAction(struct NN *neuralNetwork, struct Matrix *input, int sdIndex, int layerIndex){
  // this is action invocedonly when calculation has achived the SD layer
  // there are 3 steps:
  // 1) save the D data into the SD memory layer with the *-1 action 
  // 2) perform input + SDMemory layer action
  // 3) save the S data into SD memoty layer

  // 1 action 
  // due to delay in saving the copy of D neurons is made and after action 2 the data is saved to memory
  //printf("The D layer:\n");
  float *DSDMemory = (float*)malloc(sizeof(float));
  int dNeuronsCount = 0;
  for(int i=0; i<neuralNetwork->neuronsSize[layerIndex]; i++){
    if(neuralNetwork->sdNeuronsTypes[sdIndex][i] == 2){
      dNeuronsCount++;
      DSDMemory = (float*)realloc(DSDMemory, dNeuronsCount * sizeof(float));
      //printf("%f \n", input->matrix[i][0]);
      DSDMemory[dNeuronsCount - 1] = (-1) * input->matrix[i][0];
      // neuralNetwork->SDMemory[sdIndex]->matrix[i][0] = (-1) * input->matrix[i][0];
    }
  }

  // 2 action
  int type = 1; // for sub action
  struct Matrix *output = (struct Matrix*)malloc(sizeof(struct Matrix));
  matrixSubstAdd(input, neuralNetwork->SDMemory[sdIndex], output, type);

  // 1.2 action
  int indexD = 0;
  for(int i=0; i<neuralNetwork->neuronsSize[layerIndex]; i++){
    if(neuralNetwork->sdNeuronsTypes[sdIndex][i] == 2){
      neuralNetwork->SDMemory[sdIndex]->matrix[i][0] = DSDMemory[indexD];
    }
  }
  
  // 3 action
  //printf("The S layer:\n");
  for(int i=0; i<neuralNetwork->neuronsSize[layerIndex]; i++){
    if(neuralNetwork->sdNeuronsTypes[sdIndex][i] == 1){
      //printf("%f \n", input->matrix[i][0]);
      neuralNetwork->SDMemory[sdIndex]->matrix[i][0] = output->matrix[i][0];
    }
  }
  
  // now the copy of data performed to output the input matrix
  free(DSDMemory);
  matrixDeleteOnlyData(input);
  fullyCopyMatrix(output, input);
  matrixDelete(output);
}

void oneCalculation(struct NN *neuralNetwork, struct Matrix *input, struct Matrix *output){
  int sdIndex = 0;
  int layerIndex;

  // first the normalization should be made
  deNormalizationProcess(neuralNetwork, input, 0);

  for(int i=0; i<neuralNetwork->layerNumber - 1; i++){
    // perform W*input
    matrixMultiply(neuralNetwork->AW[i], input, output);

    // delete the matrix to create new pointer for future loop 
    matrixDeleteOnlyData(input);

    // perform  - Bias
    int type = 0; // for sub action
    matrixSubstAdd(output, neuralNetwork->BW[i], input, type);

    // clear the output
    matrixDeleteOnlyData(output);
    
    // perform activation function, created input for next action
    matrixAllValuesFormula(input, neuralNetwork->func_ptr);

    if(neuralNetwork->layerType[i + 1] == 1){
      layerIndex = i + 1;
      makeSDLayerAction(neuralNetwork, input, sdIndex, layerIndex);
      sdIndex++;
    }
  }
  fullyCopyMatrix(input, output);
  matrixDelete(input);

  // de_normalization can be made
  deNormalizationProcess(neuralNetwork, output, 1);
}