#include "neural/test_model_system.h"

#include "neural/neural_network.h"
#include "neural/model_system.h"

#include <stdlib.h>
#include <stdio.h>

// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"

static void createSystemNeuralNetworkInput(struct NNInput *input, int check){

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

  input->layerType = (int*)malloc(input->layerNumber * sizeof(int));
  input->layerType[0] = 0;
  input->layerType[1] = 0;
  input->layerType[2] = 0;
  input->layerType[3] = 0;

  input->sdNumber = 0;
}

int testSystemCreate(){
  printf(ANSI_BOLD "=======TEST SYSTEM NN CREATE STARTED=======" ANSI_COLOR_RESET "\n");

  struct NNInput *input = (struct NNInput*)malloc(sizeof(struct NNInput));
  struct SystemNN *systemNN = (struct SystemNN*)malloc(sizeof(struct SystemNN));

  createSystemNeuralNetworkInput(input, 1);

  createNNSystem(systemNN, input);

  systemNN->maxSys =  50.0;
  systemNN->minSys = -50.0;

  createDeNormalization(systemNN);

  struct InputPop *inputPop = (struct InputPop*)malloc(sizeof(struct InputPop));
  struct Pop *pop = (struct Pop*)malloc(sizeof(struct Pop));
  int size[] = {1, systemNN->neuralNetwork->countOfValues};
  float *max = (float*)malloc(systemNN->neuralNetwork->countOfValues * sizeof(float));
  float *min = (float*)malloc(systemNN->neuralNetwork->countOfValues * sizeof(float));
  for(int i=0; i < systemNN->neuralNetwork->countOfValues; i++){
    max[i] = 1.0;
    min[i] =  0.0;
  }

  FILE *csvFile = fopen("TOOLBOX/PYTHON/input/data_nn.csv", "w");
  fprintf(csvFile, "CV,RV\n");

  createInputPop(inputPop, max, min, size);
  createStructure(inputPop, pop);

  fillMatrixesNN(systemNN->neuralNetwork, pop->pop[0]);
  makeSimulationOfSignalNN(systemNN, csvFile, 1);

  FILE *csvFile2 = fopen("TOOLBOX/PYTHON/input/data_nn_fit.csv", "w");
  fprintf(csvFile2, "best\n");
  fprintf(csvFile, "%f\n", systemNN->fit);

  fclose(csvFile);
  fclose(csvFile2);

  plotGraphNN();

  clearNNSystem(systemNN);
  clearPopulation(pop);
  free(max);
  free(min);

  
 
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST SYSTEM NN CREATE SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}