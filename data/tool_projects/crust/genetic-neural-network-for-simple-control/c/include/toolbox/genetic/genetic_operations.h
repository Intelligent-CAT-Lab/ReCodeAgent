#ifndef GENETIC_OPERATIONS_H
#define GENETIC_OPERATIONS_H

#include "genetic/population.h"

// is used to select best gens of population based on fit function output and create new population
void selbest(float *fit, int fitLength, struct Pop *population, struct Pop *newPopulation, int *selects, int selectsLength, int way);

// is used to select random genes to result in new population 
void selrand(struct Pop *population, struct Pop *newPopulation, int rows);

// is used to select by turn genes for new population
void selturn(struct Pop *population, float* fit, struct Pop *newPopulation, int rows);

// is used to do crossover of the gens of population and return population
void crosov(struct Pop *population, int *selects, int selectsLength);

// is used to mutate with chance the values of gen
void mutx(struct Pop *population, float chance);

#endif
