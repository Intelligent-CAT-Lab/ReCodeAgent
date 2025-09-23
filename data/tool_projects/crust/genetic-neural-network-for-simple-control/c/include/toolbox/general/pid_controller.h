#ifndef PID_CONTROLLER_H
#define PID_CONTROLLER_H

#include "general/signal_designer.h"

#include <stdio.h>

// the structure which will hold all information needed for PID work
typedef struct PID {
  // constants not changed in calculations
  float Kp; // constant of P
  float Ki; // constant of I
  float Kd; // constant of D
  float tauI; // time constant, can be used in the PID to make up for responce time in Integral part 
  float tauD; // same as tauI but for the Derivative part 

  // values used in process of the calculations
  float prevError;      // the previous error used in D 

  float proportiError;  // the values of the P argument of the PID controller
  float integralError;  // the values of the I argument of the PID controller
  float differenError;  // the values of the D argument of the PID controller

  // input values
  struct Signal *signal;    // structure which contains the siggnal used in the system 
  float (*func_system)(float*); // function pointer to the system simulated

  // memory of output
  struct Signal *output; // the array of output values
  float *dataSystem;     // memory used by the system

  int sizeDataSystem; // size of data_system saving point

  // the limits
  float limMaxInt;
  float limMinInt;

  float limMax;
  float limMin;

  // the fit value of pid run
  float fit;

  // the check for steady rise pid to prevent best PID fi value beeing the steady rise line
  int steadyRiseCheck;

  // the maxCounter is used to determine sygnals that are too osciliating and as such have too much over the limit value (>1%)
  int maxCounter;
}PID;

void createNewPidController(struct PID *pid);

void deletePid(struct PID *pid);

void makeSimulationOfSignal(struct PID *pid, FILE *csvFile, int csv);

#endif