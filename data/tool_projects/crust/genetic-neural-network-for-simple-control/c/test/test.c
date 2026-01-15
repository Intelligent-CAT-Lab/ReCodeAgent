// neural
#include "neural/test_activation_fnc.h"
#include "neural/test_model_system.h"
#include "neural/test_neural_network.h"

// genetic
#include "genetic/test_genetic_operations.h"
#include "genetic/test_population.h"

// general
#include "general/test_matrixes.h"
#include "general/test_pid_controller.h"
#include "general/test_signal_designer.h"
#include "general/test_sort.h"
#include "general/test_system_builder.h"

// runs
#include "runs/test_full_run.h"

// libs
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"
#define ANSI_COLOR_YELLOW  "\x1b[33m"

void matrixTests(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST MATRIX STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testMatrixAddSub();
  successCount += flag;
  count++;

  flag = testMatrixMultiply();
  successCount += flag;
  count++;

  flag = testMatrixAllValuesFormula();
  successCount += flag;
  count++;

  flag = testMatrixCreateFromPointer();
  successCount += flag;
  count++;

  flag = testMatrixFullyCoppyMatrix();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST MATRIX ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void actFncTests(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST ACT FNC STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testActivationFncTanh();
  successCount += flag;
  count++;

  flag = testActivationFncSigm();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST ACT FNC ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void sortTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST SORT FULL STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testQuicksort();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST SORT FULL ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void populationTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST POPULATION STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testPopulation();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST POPULATION ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void geneticOperationsTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST GENETIC OPERATIONS STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testSelbest();
  successCount += flag;
  count++;

  flag = testCrossov();
  successCount += flag;
  count++;

  flag = testMutx();
  successCount += flag;
  count++;

  flag = testSelrand();
  successCount += flag;
  count++;

  flag = testSelturn();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST GENETIC OPERATIONS ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void neuralNetworkTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST NEURAL NETWORK STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testNeuralNetworkCreate();
  successCount += flag;
  count++;

  flag = testFillMatrixesNN();
  successCount += flag;
  count++;

  flag = testDeNormalizationProcess();
  successCount += flag;
  count++;

  flag = testOneCalculation();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST NEURAL NETWORK ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void systemBuilderTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST SYSTEM BUILDER STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testSelectSystem();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST SYSTEM BUILDER ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void signalDesignerTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST SYSTEM BUILDER STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testSignalCreate();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST SYSTEM BUILDER ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void pidControllerTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST SYSTEM BUILDER STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testPIDCreate();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST SYSTEM BUILDER ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void fullRunTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST FULL RUN STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testNNRun();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST FULL RUN ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}

void modelSystemTest(){
  int successCount = 0;
  int count = 0;
  int flag;
  printf(ANSI_BOLD "=======TEST MODEL SYSTEM STARTED=======" ANSI_COLOR_RESET "\n");

  flag = testSystemCreate();
  successCount += flag;
  count++;

  if(successCount != count){
    printf(ANSI_BOLD ANSI_COLOR_RED "Tests resulted in %d errors" ANSI_COLOR_RESET "\n", count - successCount);
  } 
  printf(ANSI_BOLD ANSI_COLOR_YELLOW "=======TEST MODEL SYSTEM ENDED [%d/%d]=======" ANSI_COLOR_RESET "\n", successCount, count);
}



int main(){
  srand(time(0));
  int userChoice;

  while (1) {
    printf("1.  matrixes\n");
    printf("2.  act fnc\n");
    printf("3.  sort\n");
    printf("4.  population\n");
    printf("5.  genetic operations\n");
    printf("6.  neural network\n");
    printf("7.  system builder\n");
    printf("8.  signal designer\n");
    printf("9.  pid\n");
    printf("10. full run\n");
    printf("11. system model\n");
    printf("99. Exit\n");

    printf("Enter your choice: ");
    scanf("%d", &userChoice);

    switch (userChoice) {
      case 1:
        matrixTests();
        break;
      case 2:
        actFncTests();
        break;
      case 3:
        sortTest();
        break;
      case 4:
        populationTest();
        break;
      case 5:
        geneticOperationsTest();
        break;
      case 6:
        neuralNetworkTest();
        break;
      case 7:
        systemBuilderTest();
        break;
      case 8:
        signalDesignerTest();
        break;
      case 9:
        pidControllerTest();
        break;
      case 10:
        fullRunTest();
        break;
      case 11:
        modelSystemTest();
        break;
      case 99:
        printf("Exiting the tests\n");
        return 0;
      default:
        printf("Invalid option. Please try again.\n");
    }
    }
  return 0;
}

