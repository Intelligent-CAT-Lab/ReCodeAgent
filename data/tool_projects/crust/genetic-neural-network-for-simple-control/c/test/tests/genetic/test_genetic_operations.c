#include "genetic/test_genetic_operations.h"
#include "genetic/genetic_operations.h"
#include "genetic/population.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"

static int arraysEqual(int *arr1, int *arr2, int size) {
    for (int i = 0; i < size; ++i) {
        if (arr1[i] != arr2[i]) {
            return 0; 
        }
    }
    return 1; 
}

static void createPopulation(struct Pop *population, int* size){
  struct InputPop *input = (struct InputPop*)malloc(sizeof(struct InputPop));
  float max[]  = {10.0, 100.0, -10.0};
  float min[]  = {.0, 50.0, -20.0};

  createInputPop(input, max, min, size);
  createStructure(input, population);
}

int testSelbest(){
  printf(ANSI_BOLD "=======TEST SELBEST STARTED=======" ANSI_COLOR_RESET "\n");

  struct Pop *newPopulationHigher = (struct Pop*)malloc(sizeof(struct Pop));
  struct Pop *newPopulationLower  = (struct Pop*)malloc(sizeof(struct Pop));
  struct Pop *population = (struct Pop*)malloc(sizeof(struct Pop));
  

  float fit[]       = {-1, 15, -3};
  int selects[]     = {2, 2};
  int size[]        = {3, 3};
  int fitLength     = 3;
  int selectsLength = 2;
  int wayhigher     = 0;
  int waylower      = 1;

  createPopulation(population, size);
  selbest(fit, fitLength, population, newPopulationHigher, selects, selectsLength, wayhigher);
  selbest(fit, fitLength, population, newPopulationLower,  selects, selectsLength, waylower);

  printf("Initial population\n");
  for(int i=0; i<population->rows; i++){
    for(int j=0; j<population->cols; j++){
        printf("%f ", population->pop[i][j]);
    }
    printf("\n");
  }

  printf("New population higher\n");
  for(int i=0; i<newPopulationHigher->rows; i++){
    for(int j=0; j<newPopulationHigher->cols; j++){
        printf("%f ", newPopulationHigher->pop[i][j]);
    }
    printf("\n");
  }

  printf("New population lower\n");
  for(int i=0; i<newPopulationLower->rows; i++){
    for(int j=0; j<newPopulationLower->cols; j++){
        printf("%f ", newPopulationLower->pop[i][j]);
    }
    printf("\n");
  }

  int flagHigher1 = arraysEqual(population->pop[1], newPopulationHigher->pop[0], population->cols);
  int flagHigher2 = arraysEqual(population->pop[1], newPopulationHigher->pop[1], population->cols);
  int flagHigher3 = arraysEqual(population->pop[0], newPopulationHigher->pop[2], population->cols);
  int flagHigher4 = arraysEqual(population->pop[0], newPopulationHigher->pop[3], population->cols);

  int flagLower1 = arraysEqual(population->pop[2], newPopulationLower->pop[0], population->cols);
  int flagLower2 = arraysEqual(population->pop[2], newPopulationLower->pop[1], population->cols);
  int flagLower3 = arraysEqual(population->pop[0], newPopulationLower->pop[2], population->cols);
  int flagLower4 = arraysEqual(population->pop[0], newPopulationLower->pop[3], population->cols);

  int flag = 1;
  if (flagHigher1 == 0 || flagHigher2 == 0 || flagHigher3 == 0 || flagHigher4 == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "ERROR: Higher population selbest error" ANSI_COLOR_RESET "\n");
    flag = 0;
  } else if (flagLower1 == 0 || flagLower2 == 0 || flagLower3 == 0 || flagLower4 == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "ERROR: Lower population selbest error" ANSI_COLOR_RESET "\n");
    flag = 0;
  }

  clearPopulation(population);
  clearPopulation(newPopulationHigher);
  clearPopulation(newPopulationLower);
  
  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST SELBEST FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST SELBEST SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testCrossov(){
  printf(ANSI_BOLD "=======TEST CROSSOV STARTED=======" ANSI_COLOR_RESET "\n");

  struct Pop *population = (struct Pop*)malloc(sizeof(struct Pop));

  int size[]        = {2, 9};
  float row1[]      = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0};
  float row2[]      = {10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0};
  float row1Check[] = {1.0, 2.0, 30.0, 40.0, 50.0, 6.0, 7.0, 80.0, 90.0};
  float row2Check[] = {10.0, 20.0, 3.0, 4.0, 5.0, 60.0, 70.0, 8.0, 9.0};
  int selectsLength = 3;
  int *selects      = (int*)malloc(selectsLength * sizeof(int));
  selects[0] = 2;
  selects[1] = 5;
  selects[2] = 7;

  // created one row population row is made to new row in order to proceed
  createPopulation(population, size);
  memcpy(population->pop[0], row1, population->cols * sizeof(float));
  memcpy(population->pop[1], row2, population->cols * sizeof(float));

  crosov(population, selects, selectsLength);   

  printf("The result population:\n");
  for(int i=0; i<population->rows; i++){
    for(int j=0; j<population->cols; j++){
        printf("%f ", population->pop[i][j]);
    }
    printf("\n");
  }

  int flag1 = arraysEqual(population->pop[0], row1Check, population->cols);
  int flag2 = arraysEqual(population->pop[1], row2Check, population->cols);

  clearPopulation(population);

  if(flag1 == 0 || flag2 == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST CROSSOV FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST CROSSOVSUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testMutx(){
  printf(ANSI_BOLD "=======TEST MUTX STARTED=======" ANSI_COLOR_RESET "\n");
  
  struct Pop *population = (struct Pop*)malloc(sizeof(struct Pop));
  int size[] = {2, 9};
  int chance = 0.3;

  createPopulation(population, size);
  mutx(population, chance);
  clearPopulation(population);
  
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST MUTX SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testSelrand(){
  printf(ANSI_BOLD "=======TEST SELRAND STARTED=======" ANSI_COLOR_RESET "\n");
  
  struct Pop *population    = (struct Pop*)malloc(sizeof(struct Pop));
  struct Pop *newPopulation = (struct Pop*)malloc(sizeof(struct Pop));
  int size[] = {5, 3};
  int rows = 10;

  createPopulation(population, size);
  selrand(population, newPopulation, rows);
  
  printf("The result population:\n");
  for(int i=0; i<newPopulation->rows; i++){
    for(int j=0; j<newPopulation->cols; j++){
        printf("%f ", newPopulation->pop[i][j]);
    }
    printf("\n");
  }

  clearPopulation(population);
  clearPopulation(newPopulation);
  
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST SELRAND SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testSelturn(){
  printf(ANSI_BOLD "=======TEST SELTURN STARTED=======" ANSI_COLOR_RESET "\n");
  
  struct Pop *population    = (struct Pop*)malloc(sizeof(struct Pop));
  struct Pop *newPopulation = (struct Pop*)malloc(sizeof(struct Pop));
  int size[] = {3, 3};
  int rows = 10;

  float fit[] = {0.9, 0.3, 0.5};

  createPopulation(population, size);
  selturn(population, fit, newPopulation, rows);
  
  printf("The result population:\n");
  for(int i=0; i<newPopulation->rows; i++){
    for(int j=0; j<newPopulation->cols; j++){
        printf("%f ", newPopulation->pop[i][j]);
    }
    printf("\n");
  }

  clearPopulation(population);
  clearPopulation(newPopulation);
  
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST SELTURN SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}