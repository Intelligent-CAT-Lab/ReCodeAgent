#ifndef FIT_FUNCTIONS_H
#define FIT_FUNCTIONS_H

#include "genetic/population.h"
#include "general/pid_controller.h"
#include "general/signal_designer.h"
#include "neural/model_system.h"


#include <stdio.h>

// function to get fit value of the pid designed controller
void pidFitFunction(struct Pop *population, float *fit, struct PID *pid);

// function to get fir values of the nn system population
void nnFitFunction(struct Pop *population, float *fit, struct SystemNN *systemNN);

#endif