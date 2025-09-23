#include "genetic/genetic_operations.h"

#include "genetic/population.h"
#include "general/sort.h"
#include "general/general_math.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <float.h>


static void create_input_pop_custom(struct InputPop *inputPop, int rows, struct Pop *population){
  inputPop->cols = population->cols;
  inputPop->rows = rows;

  inputPop->S    = (float**)malloc(2 * sizeof(float*));
  inputPop->S[0] = (float *)malloc(inputPop->cols * sizeof(float));
  inputPop->S[1] = (float *)malloc(inputPop->cols * sizeof(float));

  // correct the copy of the array
  memcpy(inputPop->S[0], population->S[0], inputPop->cols * sizeof(float));
  memcpy(inputPop->S[1], population->S[1], inputPop->cols * sizeof(float));
}

void selbest(float *fit, int fitLength, struct Pop *population, struct Pop *newPopulation, int *selects, int selectsLength, int way){
  // fit - fit of the functions
  // fitLength - length of the fit array
  // population - population used to create new
  // newPopulation - pointer to new population location
  // selects - array of rows of each best solution in new population
  // selectsLength - the length of the selects array
  // if way == 1 -> from lowest to highest
  // if way == 0 -> from highest to lowest
  // this function is selecting best values out of the pop

  // first sort the fit function and create "result" - the index array of sorting
  int rows = 0;
  for(int i=0; i<selectsLength; i++){
    rows += selects[i];
  }

  int* result = (int*)malloc(fitLength * sizeof(int));
  for(int i=0; i<fitLength; i++){
    result[i] = i;
  }

  quickSort(fit, result, fitLength);

  // now depending on the way the starting index is selected
  int resultIndex = 0;
  if(way == 0){
    resultIndex = fitLength -1;
  } else if(way == 1){
    resultIndex = 0;
  } else{
    fprintf(stderr, "Error: way should be 0 or 1\n");
    exit(EXIT_FAILURE);
  }

  // now the pop structure is created based on pointer
  struct InputPop *inputPop = (struct InputPop*)malloc(sizeof(struct InputPop));
  create_input_pop_custom(inputPop, rows, population);

  createStructure(inputPop, newPopulation);

  // the main purpose of this loop is iterate though the selects and add new rows to new population based on the sorted fit and old population
  int globalIndex = 0;
  for(int i=0; i<selectsLength; i++){
    int num = selects[i];
    for(int y=0; y<num; y++){
      // the desired row is copied to the population
      memcpy(newPopulation->pop[globalIndex], population->pop[result[resultIndex]], newPopulation->cols * sizeof(float));
      globalIndex++;
    }
    if(way == 0){
      resultIndex--;
    } else{
      resultIndex++;
    }
  }

  
  free(result);
}

void selrand(struct Pop *population, struct Pop *newPopulation, int rows){

  // now the pop structure is created based on pointer
  struct InputPop *inputPop = (struct InputPop*)malloc(sizeof(struct InputPop));
  create_input_pop_custom(inputPop, rows, population);

  createStructure(inputPop, newPopulation);
  int index;
  for(int i=0; i<rows; i++){
    index  = rand() % population->rows;
    memcpy(newPopulation->pop[i], population->pop[index], population->cols * sizeof(float));
  }
}

void selturn(struct Pop *population, float* fit, struct Pop *newPopulation, int rows){

  // this code is a copy of the code created by prof. Ivan Sekaj STU 2002 using Matlab 

  struct InputPop *inputPop = (struct InputPop*)malloc(sizeof(struct InputPop));
  create_input_pop_custom(inputPop, rows, population);
  createStructure(inputPop, newPopulation);
  
  for(int i=0; i<rows; i++){
    int j = rand() % population->rows;
    int k = rand() % population->rows;

    if(j == k){
      memcpy(newPopulation->pop[i], population->pop[j], population->cols * sizeof(float));
    } else if (fit[j] <= fit[k]){
      memcpy(newPopulation->pop[i], population->pop[j], population->cols * sizeof(float));
    } else{
      memcpy(newPopulation->pop[i], population->pop[k], population->cols * sizeof(float));
    }
  }
}

void crosov(struct Pop *population, int *selects, int selectsLength){

  if(selectsLength % 2 != 0){
    selects = (int *)realloc(selects, (selectsLength + 1) * sizeof(int));
    selects[selectsLength] = population->cols;
    selectsLength++;
  }

  int start, end, globalIndex;
  float *additionaSaving = (float*)malloc(sizeof(float));
  for(int index = 0; index < population->rows - 1; index += 2){
    
    for(int i = 0; i<selectsLength; i+=2){
      start = selects[i];
      end   = selects[i + 1];

      additionaSaving = (float *)realloc(additionaSaving, (end - start) * sizeof(float));
      globalIndex = 0;
      for(int x = start; x<end; x++){
        additionaSaving[globalIndex]  = population->pop[index][x];
        globalIndex++;
      }
      globalIndex = 0;

      for(int x = start; x<end; x++){
        population->pop[index][x]   = population->pop[index+1][x];
        population->pop[index+1][x] = additionaSaving[globalIndex];
        globalIndex++;
      }
      
    }
  }
  free(additionaSaving);
  free(selects);
}

void mutx(struct Pop *population, float chance){
  float randomFloat;

  for(int x=0; x<population->rows; x++){
    for(int y=0; y<population->cols; y++){
      randomFloat = (float)rand() / RAND_MAX;
      if(randomFloat < chance){
        randomFloat = createRandomFloat(population->S[1][y], population->S[0][y]);
        population->pop[x][y] = randomFloat;
      }
    }
  }

}