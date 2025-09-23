#include "genetic/fit_functions.h"

#include "genetic/population.h"
#include "general/pid_controller.h"
#include "general/signal_designer.h"
#include "neural/model_system.h"
#include "neural/neural_network.h"

#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void pidFitFunction(struct Pop *population, float *fit, struct PID *pid){
  int csv = 0;
  FILE *trash;

  for(int i=0; i<population->rows; i++){
    // clear system memory before running
    for(int j=0; j<pid->sizeDataSystem; j++){
      pid->dataSystem[j] = 0.0;
    }

    // first set the coeficients
    pid->Kp   = population->pop[i][0];
    pid->Ki   = population->pop[i][1];
    pid->Kd   = population->pop[i][2];
    pid->tauD = population->pop[i][3];

    makeSimulationOfSignal(pid, trash, csv);

    fit[i] = pid->fit;
    
  }
}

void nnFitFunction(struct Pop *population, float *fit, struct SystemNN *systemNN){
  FILE *trash;

  for(int i=0; i<population->rows; i++){
    // clear system memory before running
    for(int j=0; j<systemNN->sizeDataSystem; j++){
      systemNN->dataSystem[j] = 0.0;
    }

    // first set the NN wages
    fillMatrixesNN(systemNN->neuralNetwork, population->pop[i]);

    // now model is simulated
    makeSimulationOfSignalNN(systemNN, trash, 0);

    fit[i] = systemNN->fit;
    
  }
}

