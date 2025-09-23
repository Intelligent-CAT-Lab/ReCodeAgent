#ifndef POPULATION_H
#define POPULATION_H

#include <stddef.h>

// strucute used to store all input values needed to create popualtion
typedef struct InputPop {
    int cols;  // number of columns  of population
    int rows;  // number of rows of population
    float **S; // matrix of max and mins of each collumns in populations
}InputPop;

// structure used to store population 
typedef struct Pop {
    float **pop; // population matrix 
    float **S;   // matrix of max and min for each collumn 
    int cols;    // number of columns  of population
    int rows;    // number of rows of population
}Pop;

// function to create population from InputPop structure 
void createStructure(struct InputPop *input,  struct Pop *pop);

// function to clear population
void clearPopulation(struct Pop* population);

// function to create InputPop for future pop
void createInputPop(struct InputPop *inputPop, float* max, float* min, int* size);

// function to copy rows from the source to pop between two indexes in the indexes array not including second one
void placePartOfPop(struct Pop *pop, struct Pop *source, int *indexes);

// function to re-generate random population in the size matrix
void generateRandomPopulation(struct Pop *population);

#endif
