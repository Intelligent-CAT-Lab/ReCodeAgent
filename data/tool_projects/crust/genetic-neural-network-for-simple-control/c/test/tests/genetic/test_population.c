#include "genetic/test_population.h"

#include "genetic/population.h"

#include <stdlib.h>
#include <stdio.h>

// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"



int testPopulation(){
  printf(ANSI_BOLD "=======TEST POPULATION STARTED=======" ANSI_COLOR_RESET "\n");
  
  struct InputPop *input = (struct InputPop*)malloc(sizeof(struct InputPop));
  struct Pop *pop = (struct Pop*)malloc(sizeof(struct Pop));
  float max[]  = {10.0, 100.0, -10.0};
  float min[]  = {.0, 50.0, -20.0};
  int size[] = {3, 3};

  createInputPop(input, max, min, size);
  createStructure(input, pop);

  int flag = 1;
  
  for(int i=0; i>pop->cols; i++){
    printf("%f - %f\n", pop->S[0][i], pop->S[1][i]); 
    if(pop->S[0][i] != max[i] || pop->S[1][i] != min[i]){
      flag = 0;
      printf(ANSI_BOLD ANSI_COLOR_RED  "ERROR: max/min is incorrect" ANSI_COLOR_RESET "\n");
    }
  }

  printf("Following population created:\n");
  for(int i=0; i<pop->rows; i++){
    for(int j=0; j<pop->cols; j++){
      printf("%f ", pop->pop[i][j]);
      if(pop->pop[i][j] > pop->S[0][j] || pop->pop[i][j] < pop->S[1][j]){
        flag = 0;
      }
    }
    printf("\n");
  }

  clearPopulation(pop);

  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST POPULATION FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST POPULATION SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}