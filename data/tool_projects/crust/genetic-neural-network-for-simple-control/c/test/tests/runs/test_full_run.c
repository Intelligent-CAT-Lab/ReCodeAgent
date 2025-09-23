#include "general/test_sort.h"
#include "general/sort.h"

#include "genetic/fit_functions.h"
#include "genetic/genetic_operations.h"

#include <stdlib.h>
#include <stdio.h>
#include <time.h>


// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"

static int getMinFit(float *fit, int length){
  int min = 0;

  for(int i=0; i<length; i++){
    if( fit[min] > fit[i]){
      min = i;
    }
  }
  return min;
}

static void closeRandomSizeGeneration(struct Pop *population, float* sizes){
  for(int i=0; i<population->cols; i++){
    population->S[0][i] = sizes[i] + 0.7;
    population->S[1][i] = sizes[i] - 0.7;
  }
}

static void createSystemNeuralNetworkInputTEST(struct NNInput *input){

  input->layerNumber = 5;

  input->neuronsSize = (int*)malloc(input->layerNumber * sizeof(int));
  input->neuronsSize[0] = 1;
  input->neuronsSize[1] = 5;
  input->neuronsSize[2] = 5;
  input->neuronsSize[3] = 5;
  input->neuronsSize[4] = 5;
  input->neuronsSize[5] = 1;

  input->layerType = (int*)malloc(input->layerNumber * sizeof(int));
  input->layerType[0] = 0;
  input->layerType[1] = 0;
  input->layerType[2] = 0;
  input->layerType[3] = 0;
  input->layerType[4] = 0;
  input->layerType[5] = 0;

  input->sdNumber = 0;
}
// ##################################################################################################################

int testPIDRun(){
  printf(ANSI_BOLD "=======TEST PID RUN STARTED=======" ANSI_COLOR_RESET "\n");

  FILE *csvFile = fopen("TOOLBOX/PYTHON/input/data_pid_run.csv", "w");
  fprintf(csvFile, "best\n");

  float chance = 0.1;
  int generations = 5000;
  int bestIndex[]    = {0,   10};
  int randOneIndex[] = {10,  195};
  int randTwoIndex[] = {195, 380};
  int randFulIndex[] = {380, 500};

  int sizeOneIndex = 185;
  int sizeTwoIndex = 185;

  int bestNums[] = {4, 2, 2, 2};
  int *restCros  = (int*)malloc(2 * sizeof(int));
  restCros[0] = 1;
  restCros[1] = 2;

  int bestFit;

  // create first the population
  struct InputPop *input = (struct InputPop*)malloc(sizeof(struct InputPop));
  struct Pop *pop = (struct Pop*)malloc(sizeof(struct Pop));
  //              |    P   |   I    |    D    | tau |
  float max[]  = {   100.0,   100.0,    100.0, 1.0};
  float min[]  = {     0.0,     0.0,      0.0, 0.0};

  int size[]    = {500, 4};
  int sizeFul[] = {120, 4};

  createInputPop(input, max, min, size);
  createStructure(input, pop);

  float *fit = (float*)malloc(pop->rows * sizeof(float));

  // then PID is created
  struct PID *pid = (struct PID*)malloc(sizeof(struct PID));
  createNewPidController(pid);  // then PID is created

  // set deffault min_max values

  // set deffault min_max values
  pid->limMax =  60.0;
  pid->limMin = -60.0;

  pid->limMaxInt = pid->limMax / 2.0;
  pid->limMinInt = pid->limMin / 2.0;

  for(int i=0; i<generations; i++){
    
    
    pidFitFunction(pop, fit, pid);
    bestFit = getMinFit(fit, pop->rows);
    
    
    fprintf(csvFile, "%f\n", fit[bestFit]);
    if(i % 100 == 0 || i>9990){
      printf("GENERATION[%05d] - fit: %f P: %f I: %f D: %f Tau: %f - %d\n", i, fit[bestFit], pop->pop[bestFit][0], pop->pop[bestFit][1], pop->pop[bestFit][2], pop->pop[bestFit][3], bestFit);
    }
    

    struct Pop *bestPop    = (struct Pop*)malloc(sizeof(struct Pop));
    struct Pop *randPopOne = (struct Pop*)malloc(sizeof(struct Pop));
    struct Pop *randPopTwo = (struct Pop*)malloc(sizeof(struct Pop));

    // now the genetic operations are runned
   
    // the best pop with 50 records is created
    selbest(fit, pop->rows, pop, bestPop, bestNums, 4, 1);

    // 2 seturn arrays are created
    selturn(pop, fit, randPopOne, sizeOneIndex);
    selturn(pop, fit, randPopTwo, sizeTwoIndex);

    // now crosov is run
    int *restCrosOne  = (int*)malloc(2 * sizeof(int));
    restCrosOne[0] = 1;
    restCrosOne[1] = 2;
    crosov(randPopOne, restCrosOne, 2);

    int *restCrosTwo  = (int*)malloc(2 * sizeof(int));
    restCrosTwo[0] = 1;
    restCrosTwo[1] = 2;
    crosov(randPopTwo, restCrosTwo, 2);

    // and finnaly the mutx
    mutx(randPopOne, chance);
    mutx(randPopTwo, chance);

    // add the random 200 at the end of each generation
    struct InputPop *input = (struct InputPop*)malloc(sizeof(struct InputPop));
    struct Pop *popRandom  = (struct Pop*)malloc(sizeof(struct Pop));
    createInputPop(input, max, min, sizeFul);
    createStructure(input, popRandom);

    // now copy pops to main pop
    placePartOfPop(pop, bestPop,    bestIndex);
    placePartOfPop(pop, randPopOne, randOneIndex);
    placePartOfPop(pop, randPopTwo, randTwoIndex);
    placePartOfPop(pop, popRandom,  randFulIndex);

    if(i == generations - 1 || i % 100 == 0){
      FILE *csvFile2 = fopen("TOOLBOX/PYTHON/input/data_pid_.csv", "w");
      fprintf(csvFile2, "P,I,D,CV,RV\n");

      // plot best values
      pid->Kp   = bestPop->pop[0][0];
      pid->Ki   = bestPop->pop[0][1];
      pid->Kd   = bestPop->pop[0][2];
      pid->tauD = bestPop->pop[0][3];
      makeSimulationOfSignal(pid, csvFile2, 1);

      printf("BEST PID: P: %f I: %f D: %f Tau: %f - FIT: %f Count: %d\n", pid->Kp, pid->Ki, pid->Kd, pid->tauD, pid->fit, pid->maxCounter);

      fclose(csvFile2);
    }

    clearPopulation(bestPop);
    clearPopulation(randPopOne);
    clearPopulation(randPopTwo);
    clearPopulation(popRandom);
  }  

  clearPopulation(pop);

  deletePid(pid);

  fclose(csvFile);

  free(restCros);
  free(fit);


  plotGraph();

  int flag = 1;
  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST PID RUN FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST PID RUN SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}
// ##################################################################################################################

int testNNRun(){
  printf(ANSI_BOLD "=======TEST NN RUN STARTED=======" ANSI_COLOR_RESET "\n");

  FILE *csvFile = fopen("data_nn_fit.csv", "w");
  fprintf(csvFile, "best\n");

  // the system is created and denormilized
  struct NNInput *inputNN = (struct NNInput*)malloc(sizeof(struct NNInput));
  struct SystemNN *systemNN = (struct SystemNN*)malloc(sizeof(struct SystemNN));

  createSystemNeuralNetworkInputTEST(inputNN);

  createNNSystem(systemNN, inputNN);

  // 37

  float chance = 0.1;
  int generations = 50000;

  // 500
  // int bestIndex[]      = {0,   10};
  // int randOneIndex[]   = {10,  180};
  // int randTwoIndex[]   = {180, 350};
  // int randRandIndex[]  = {350, 475};
  // int randRandCIndex[] = {475, 500};

  // int sizeOneIndex = 170;
  // int sizeTwoIndex = 170;

  // int size[]            = {500, systemNN->neuralNetwork->countOfValues};
  // int sizeRandom[]      = {125, systemNN->neuralNetwork->countOfValues};
  // int sizeCloseRandom[] = {25,  systemNN->neuralNetwork->countOfValues};

  // int bestNums[] = {4, 2, 2, 2};
  // int bestCount = 4;

  // systemNN->maxSys =  7.0;
  // systemNN->minSys = -4.0;

  // 100
  int bestIndex[]      = {0,  5};
  int randOneIndex[]   = {5,  43};
  int randTwoIndex[]   = {43, 70};
  int randRandIndex[]  = {70, 95};
  int randRandCIndex[] = {95, 100};

  int sizeOneIndex = 38;
  int sizeTwoIndex = 27;

  int size[]            = {100, systemNN->neuralNetwork->countOfValues};
  int sizeRandom[]      = {25,  systemNN->neuralNetwork->countOfValues};
  int sizeCloseRandom[] = {5,   systemNN->neuralNetwork->countOfValues};

  int bestNums[] = {2, 2, 1};
  int bestCount = 3;

  systemNN->maxSys =  40.0;
  systemNN->minSys =  -5.0;


  int *restCros  = (int*)malloc(2 * sizeof(int));
  restCros[0] = 1;
  restCros[1] = 2;

  int bestFit;

  createDeNormalization(systemNN);

  // create first the population
  struct InputPop *inputPop = (struct InputPop*)malloc(sizeof(struct InputPop));
  struct Pop *pop = (struct Pop*)malloc(sizeof(struct Pop));

  float *max = (float*)malloc(systemNN->neuralNetwork->countOfValues * sizeof(float));
  float *min = (float*)malloc(systemNN->neuralNetwork->countOfValues * sizeof(float));
  for(int i=0; i < systemNN->neuralNetwork->countOfValues; i++){
    max[i] =  1.0;
    min[i] = -1.0;
  } 

  createInputPop(inputPop, max, min, &size);
  createStructure(inputPop, pop);

  float *fit = (float*)malloc(pop->rows * sizeof(float));

  // add the random 200 at the end of each generation
  struct InputPop *inputRandom = (struct InputPop*)malloc(sizeof(struct InputPop));
  struct Pop *popRandom        = (struct Pop*)malloc(sizeof(struct Pop));
  createInputPop(inputRandom, max, min, sizeRandom);
  createStructure(inputRandom, popRandom);

  struct InputPop *inputCRandom = (struct InputPop*)malloc(sizeof(struct InputPop));
  struct Pop *popCloseRandom    = (struct Pop*)malloc(sizeof(struct Pop));
  createInputPop(inputCRandom, max, min, sizeCloseRandom);
  createStructure(inputCRandom, popCloseRandom);

  for(int i=0; i<generations; i++){
    
    
    nnFitFunction(pop, fit, systemNN);
    bestFit = getMinFit(fit, pop->rows);
    
    
    fprintf(csvFile, "%f\n", fit[bestFit]);
    if(i % 10 == 0 || i>9990){
      printf("GENERATION[%05d] - fit: %f  - %d\n", i, fit[bestFit], bestFit);
    }
    

    struct Pop *bestPop    = (struct Pop*)malloc(sizeof(struct Pop));
    struct Pop *randPopOne = (struct Pop*)malloc(sizeof(struct Pop));
    struct Pop *randPopTwo = (struct Pop*)malloc(sizeof(struct Pop));

    // now the genetic operations are runned
   
    // the best pop with 50 records is created
    selbest(fit, pop->rows, pop, bestPop, bestNums, bestCount, 1);

    // 2 seturn arrays are created
    selturn(pop, fit, randPopOne, sizeOneIndex);
    selturn(pop, fit, randPopTwo, sizeTwoIndex);

    // now crosov is run
    int *restCrosOne  = (int*)malloc(5 * sizeof(int));
    restCrosOne[0] = 5;
    restCrosOne[1] = 10;
    restCrosOne[2] = 15;
    restCrosOne[3] = 20;
    restCrosOne[4] = 25;
    crosov(randPopOne, restCrosOne, 2);

    int *restCrosTwo  = (int*)malloc(4 * sizeof(int));
    restCrosTwo[0] = 10;
    restCrosTwo[1] = 15;
    restCrosTwo[2] = 27;
    restCrosTwo[3] = 35;
    crosov(randPopTwo, restCrosTwo, 4);

    // and finnaly the mutx
    mutx(randPopOne, chance);
    mutx(randPopTwo, chance);

    // create new pop random and close random pop
    closeRandomSizeGeneration(popCloseRandom, pop->pop[bestFit]);

    generateRandomPopulation(popRandom);
    generateRandomPopulation(popCloseRandom);

    if((i % 100 == 0 || i == generations - 1) && i != 0){
      FILE *csvFile2 = fopen("TOOLBOX/PYTHON/input/data_nn.csv", "w");
      fprintf(csvFile2, "CV,RV, System Output, Corrected Output\n");

      FILE *csvFile3 = fopen("TOOLBOX/PYTHON/input/best_nn.txt", "a");
      fprintf(csvFile3, "[");
      for(int j=0; j<pop->cols; j++){
        fprintf(csvFile3, "%f, ", pop->pop[0][j]);
      } fprintf(csvFile3, "]");
      fclose(csvFile3);

      // first set the NN wages
      fillMatrixesNN(systemNN->neuralNetwork, pop->pop[bestFit]);

      // now model is simulated
      makeSimulationOfSignalNN(systemNN, csvFile2, 1);

      printf("BEST NN: FIT: %f Count: %d\n", systemNN->fit, systemNN->maxCounter);

      fclose(csvFile2);
      fclose(csvFile);


      plotGraphNN();

      FILE *csvFile = fopen("TOOLBOX/PYTHON/input/data_nn_fit.csv", "a");
    }

    // now copy pops to main pop
    placePartOfPop(pop, bestPop,         bestIndex);
    placePartOfPop(pop, randPopOne,      randOneIndex);
    placePartOfPop(pop, randPopTwo,      randTwoIndex);
    placePartOfPop(pop, popRandom,       randRandIndex);
    placePartOfPop(pop, popCloseRandom,  randRandCIndex);


    clearPopulation(bestPop);
    clearPopulation(randPopOne);
    clearPopulation(randPopTwo);
  }  

  clearPopulation(pop);
  clearPopulation(popRandom);

  clearNNSystem(systemNN);

  free(restCros);
  free(fit);


  plotGraphNN();

  int flag = 1;
  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST NN RUN FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST NN RUN SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}