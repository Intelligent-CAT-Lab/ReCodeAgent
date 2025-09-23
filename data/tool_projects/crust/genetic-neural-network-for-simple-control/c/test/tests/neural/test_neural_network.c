#include "neural/test_neural_network.h"

#include "neural/neural_network.h"
#include "genetic/population.h"
#include "general/matrix_math.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <math.h>

// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"

static int arraysEqualNN(int *arr1, int *arr2, int size) {
    for (int i = 0; i < size; ++i) {
        if (arr1[i] != arr2[i]) {
            return 0; 
        }
    }
    return 1; 
}

static int checkMatrixesNN(struct NN *neuralNetwork, float *population){
  int globalIndex = 0;
  int flag = 1;
  
  for(int i=0; i<neuralNetwork->layerNumber - 1; i++){
    // first the AW matrix is set
    for(int x=0; x<neuralNetwork->AW[i]->sizes[0]; x++){
      for(int y=0; y<neuralNetwork->AW[i]->sizes[1]; y++){
        if(neuralNetwork->AW[i]->matrix[x][y] != population[globalIndex]){
          flag = 0;
        }
        globalIndex++;
      }
    }

    
    // second the BW is set
    if( i < neuralNetwork->layerNumber - 2){
      for(int x=0; x<neuralNetwork->BW[i]->sizes[0]; x++){
        for(int y=0; y<neuralNetwork->BW[i]->sizes[1]; y++){
          if(neuralNetwork->BW[i]->matrix[x][y] != population[globalIndex]){
            flag = 0;
          }
          globalIndex++;
        }
      }
    }
  }
  return flag;
}

static void createSimpleNeuralNetwork(struct NN *neuralNetwork, int check){
  struct NNInput *input = (struct NNInput*)malloc(sizeof(struct NNInput));

  input->neuronsSize = (int*)malloc(4 * sizeof(int));
  input->neuronsSize[0] = 1;
  if(check == 1){
    input->neuronsSize[1] = 3;
    input->neuronsSize[2] = 2;
  } else{
    input->neuronsSize[1] = 5;
    input->neuronsSize[2] = 5;
  }
  input->neuronsSize[3] = 1;

  input->layerNumber = 4;

  input->normalizationMatrix   = (float**)malloc(2 * sizeof(float*));
  input->denormalizationMatrix = (float**)malloc(2 * sizeof(float*));

  input->normalizationMatrix[0]   = (float*)malloc(sizeof(float));
  input->normalizationMatrix[1]   = (float*)malloc(sizeof(float));

  input->denormalizationMatrix[0] = (float*)malloc(sizeof(float));
  input->denormalizationMatrix[1] = (float*)malloc(sizeof(float));

  input->normalizationMatrix[0][0]   = 100;
  input->normalizationMatrix[1][0]   = .0;

  input->denormalizationMatrix[0][0] = 100;
  input->denormalizationMatrix[1][0] = 0.0;

  input->layerType = (int*)malloc(input->layerNumber * sizeof(int));
  input->layerType[0] = 0;
  input->layerType[1] = 1;
  input->layerType[2] = 0;
  input->layerType[3] = 0;

  input->sdNumber = 1;

  createNeuralNetwork(input, neuralNetwork);
}

int testNeuralNetworkCreate(){
  printf(ANSI_BOLD "=======TEST NEURAL NETWORK CREATE STARTED=======" ANSI_COLOR_RESET "\n");

  struct NN *neuralNetwork = (struct NN*)malloc(sizeof(struct NN));
  int check = 0;
  createSimpleNeuralNetwork(neuralNetwork, check);

  int neuronsSize[] = {1, 5, 5, 1};
  int layerType[]   = {0, 1, 0, 0};

  float normalizationMax[] = {100};
  float normalizationMin[] = {0.0};

  float deNormalizationMax[] = {100};
  float deDormalizationMin[] = {0.0};

  int sdTypesOne[] = {0, 0, 1, 2, 2};
  
  int rowSD[] = {5, 1};

  int flag1 = arraysEqualNN(neuronsSize, neuralNetwork->neuronsSize, neuralNetwork->layerNumber);
  int flag2 = arraysEqualNN(layerType,   neuralNetwork->layerType,   neuralNetwork->layerNumber);
  int flag3 = arraysEqualNN(layerType,   neuralNetwork->layerType,   neuralNetwork->layerNumber);

  int flag4 = arraysEqualNN(normalizationMax,   neuralNetwork->normalizationMatrix[0],   1);
  int flag5 = arraysEqualNN(normalizationMin,   neuralNetwork->normalizationMatrix[1],   1);

  int flag6 = arraysEqualNN(deNormalizationMax,   neuralNetwork->denormalizationMatrix[0],   1);
  int flag7 = arraysEqualNN(deDormalizationMin,   neuralNetwork->denormalizationMatrix[1],   1);

  int flag8 = arraysEqualNN(sdTypesOne,   neuralNetwork->sdNeuronsTypes[0],   5);

  int flag9 = arraysEqualNN(rowSD,   neuralNetwork->SDMemory[0]->sizes,   2);

  clearNeuralNetwork(neuralNetwork);
  
  if(flag1 == 0 || flag2 == 0 || flag3 == 0 || flag4 == 0 || flag5 == 0 || flag6 == 0 || flag7 == 0 || flag8 == 0 || flag9 == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST NEURAL NETWORK CREATE FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST NEURAL NETWORK CREATE SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testFillMatrixesNN(){
  printf(ANSI_BOLD "=======TEST FILL MATRIXES NN STARTED=======" ANSI_COLOR_RESET "\n");

  struct NN *neuralNetwork = (struct NN*)malloc(sizeof(struct NN));
  int check = 0;
  createSimpleNeuralNetwork(neuralNetwork, check);

  struct InputPop *input = (struct InputPop*)malloc(sizeof(struct InputPop));
  struct Pop *pop = (struct Pop*)malloc(sizeof(struct Pop));
  int size[] = {1, neuralNetwork->countOfValues};
  float *max = (float*)malloc(neuralNetwork->countOfValues * sizeof(float));
  float *min = (float*)malloc(neuralNetwork->countOfValues * sizeof(float));
  for(int i=0; i<neuralNetwork->countOfValues; i++){
    max[i] = 10.0;
    min[i] =  0.0;
  }

  createInputPop(input, max, min, size);
  createStructure(input, pop);

  fillMatrixesNN(neuralNetwork, pop->pop[0]);

  
  int flag = checkMatrixesNN(neuralNetwork, pop->pop[0]);

  clearNeuralNetwork(neuralNetwork);
  clearPopulation(pop);
  free(min);
  free(max);

  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST FILL MATRIXES NN FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST FILL MATRIXES NN SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testDeNormalizationProcess(){
  printf(ANSI_BOLD "=======TEST DE_NORMALIZATION PROCESS STARTED=======" ANSI_COLOR_RESET "\n");

  struct NN *neuralNetwork = (struct NN*)malloc(sizeof(struct NN));
  struct Matrix *A  = (struct Matrix*)malloc(sizeof(struct Matrix));

  int *sizes1 = (int *)malloc(2 * sizeof(int));
  sizes1[0] = 1;
  sizes1[1] = 1;
  int way = 0;
  int flag = 1;
  int check = 0;
  createSimpleNeuralNetwork(neuralNetwork, check);

  createMatrix(A, sizes1);

  A->matrix[0][0] = 50;
  deNormalizationProcess(neuralNetwork, A, way);
  if(A->matrix[0][0] != 0.0){
    flag = 0;
  }

  A->matrix[0][0] = 0.40;
  way = 1;
  deNormalizationProcess(neuralNetwork, A, way);

  if(A->matrix[0][0] != 70.0){
    flag = 0;
  }


  matrixDelete(A);
  clearNeuralNetwork(neuralNetwork);

  
  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST DE_NORMALIZATION PROCESS FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST DE_NORMALIZATION PROCESS SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testOneCalculation(){
  printf(ANSI_BOLD "=======TEST ONE CALCULATION STARTED=======" ANSI_COLOR_RESET "\n");

  struct NN *neuralNetwork = (struct NN*)malloc(sizeof(struct NN));
  int check = 1;
  createSimpleNeuralNetwork(neuralNetwork, check);
  
  struct Matrix *input = (struct Matrix*)malloc(sizeof(struct Matrix));
  struct Matrix *output = (struct Matrix*)malloc(sizeof(struct Matrix));

  int *size = (int *)malloc(2 * sizeof(int));
  size[0] = 1;
  size[1] = 1;

  createMatrix(input, size);
  input->matrix[0][0] = 70.0;


  struct Pop *pop = (struct Pop*)malloc(sizeof(struct Pop));
  struct InputPop *inputPop = (struct InputPop*)malloc(sizeof(struct InputPop));
  int sizePop[] = {1, neuralNetwork->countOfValues};
  float *max = (float*)malloc(neuralNetwork->countOfValues * sizeof(float));
  float *min = (float*)malloc(neuralNetwork->countOfValues * sizeof(float));
  for(int i=0; i<neuralNetwork->countOfValues; i++){
    max[i] =  1.0;
    min[i] = -1.0;
  }


  createInputPop(inputPop, max, min, sizePop);
  createStructure(inputPop, pop);

  float popExample[] = {0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.1, 0.2, 0.1, 0.2};
  

  fillMatrixesNN(neuralNetwork, popExample);
  oneCalculation(neuralNetwork, input, output);

  printf("Result output:\n");
  for(int i=0; i<output->sizes[0];i++){
    for(int j=0;j<output->sizes[1];j++){
      float multiplier = pow(10, 4);
      float roundedNumber = round(output->matrix[i][j] * multiplier) / multiplier;
      printf("%f ", roundedNumber);
    }
    printf("\n");
  }
  printf("Result SD memory:\n");
  for(int i=0; i<neuralNetwork->SDMemory[0]->sizes[0];i++){
    for(int j=0;j<neuralNetwork->SDMemory[0]->sizes[1];j++){
      printf("%f ", neuralNetwork->SDMemory[0]->matrix[i][j]);
    }
    printf("\n");
  }

  clearNeuralNetwork(neuralNetwork);
  clearPopulation(pop);
  matrixDelete(output);
  free(max);
  free(min);

  int flag = 1;
  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST ONE CALCULATION FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST ONE CALCULATION SUCCESSFU =======" ANSI_COLOR_RESET "\n");
  return 1;
}