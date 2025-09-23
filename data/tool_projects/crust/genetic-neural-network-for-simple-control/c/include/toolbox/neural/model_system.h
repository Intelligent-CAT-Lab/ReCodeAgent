#ifndef MODEL_SYSTEM_H
#define MODEL_SYSTEM_H

#include "neural/neural_network.h"
#include "general/matrix_math.h"
#include "general/signal_designer.h"
#include "general/systems_builder.h"

#include <stdio.h>

// this struct has all information needed for the NN run of the system
typedef struct SystemNN {

  struct NN *neuralNetwork; // NN used in the system

  // input values
  struct Signal *signal;    // structure which contains the siggnal used in the system 
  float (*func_system)(float*); // function pointer to the system simulated

  // memory of output
  struct Signal *output; // the array of output values
  float *dataSystem;     // memory used by the system
  int sizeDataSystem;    // size of data_system saving point

  float (*input_sys)(float*); // function pointer to the input system
  float  *inputData;     // memory for the input system
  int    *inputDataSize; // the vector containing all nececary information about data [start, end, full]
  int    *inputTypes;  // the array containing the types of the generic normalization toolbox config
  // 0  - e,  1  - u,   2  - y
  // 3  - de, 4  - dde, 5  - ie
  // 6  - du, 7  - ddu, 8  - iu
  // 9  - dy, 10 - ddy, 11 - iy

  // limist of the system output
  float maxSys;
  float minSys;

  float fit; // fit value of the run

  // checks
  int steadyRiseCheck; // same as in pid
  int maxCounter; // same as in pid
}SystemNN;

// functions to create and remove NN system
void createNNSystem(struct SystemNN *systemNN, struct NNInput *input);
void clearNNSystem(struct SystemNN *systemNN);

// function to simulate one close loop run of the system
void makeSimulationOfSignalNN(struct SystemNN *systemNN, FILE *csvFile, int csv);

#endif