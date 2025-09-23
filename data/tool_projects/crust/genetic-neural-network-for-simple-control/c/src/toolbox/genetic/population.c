#include "genetic/population.h"

#include "general/general_math.h"

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <stddef.h>
#include <string.h>

static void clearInputPop(struct InputPop* input){
  if(input){
    free(input->S[0]);
    free(input->S[1]);
    free(input->S);
    free(input);
  }
}


void createInputPop(struct InputPop *inputPop, float* max, float* min, int* size){
  
  inputPop->cols = size[1];
  inputPop->rows = size[0];

  inputPop->S    = (float**)malloc(2 * sizeof(float*));
  inputPop->S[0] = (float* )malloc(inputPop->cols * sizeof(float));
  inputPop->S[1] = (float* )malloc(inputPop->cols * sizeof(float));

  memcpy(inputPop->S[0], max, inputPop->cols * sizeof(float));
  memcpy(inputPop->S[1], min, inputPop->cols * sizeof(float));
}

static void alocatePopulation(struct InputPop* input, struct Pop* population){
  population->cols = input->cols;
  population->rows = input->rows;

  population->pop = (float **)malloc(population->rows * sizeof(float *));
  for (int i = 0; i < population->rows; i++) {
    population->pop[i] = (float *)malloc(population->cols * sizeof(float));
  }

  population->S    = (float**)malloc(2 * sizeof(float*));
  population->S[0] = (float* )malloc(population->cols * sizeof(float));
  population->S[1] = (float* )malloc(population->cols * sizeof(float));

  memcpy(population->S[0], input->S[0], population->cols * sizeof(float));
  memcpy(population->S[1], input->S[1], population->cols * sizeof(float));

  // clear the input pop
  clearInputPop(input);
}

void clearPopulation(struct Pop* population) {
  if (population) {
    // free the pop part
    for (int i = 0; i < population->rows; i++) {
      free(population->pop[i]);
    }
    free(population->pop);

    // free the S part
    free(population->S[0]);
    free(population->S[1]);
    free(population->S);

    // free population struct
    free(population);
  }
}

void createStructure(struct InputPop *input, struct Pop *population) {

  alocatePopulation(input, population);

  generateRandomPopulation(population);
}

void generateRandomPopulation(struct Pop *population){
  float newValue;

  for(int y = 0; y < population->rows; y++){
    for(int i=0; i < population->cols; i++ ){
      
      newValue = createRandomFloat(population->S[1][i], population->S[0][i]);
      population->pop[y][i] = newValue;
    }
  }
}

void placePartOfPop(struct Pop *pop, struct Pop *source, int *indexes){
  // the rows of the source are placed into pop between two indexes in the indexes, not including the second index
  int globalIndex = 0;
  for(int i=indexes[0]; i<indexes[1]; i++){
    memcpy(pop->pop[i], source->pop[globalIndex], source->cols * sizeof(float));
    globalIndex++;
  }
}
