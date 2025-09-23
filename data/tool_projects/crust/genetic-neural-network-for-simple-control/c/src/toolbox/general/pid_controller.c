#include "general/pid_controller.h"

#include "general/signal_designer.h"
#include "general/systems_builder.h"

#include <stdio.h>
#include <float.h>
#include <stdlib.h>

// I thank https://github.com/pms67/PID for the detailed info on how to program PID in C properly


void createNewPidController(struct PID *pid){
    // first the coeficients are set to 0 for start
    pid->Kp = 0.0;
    pid->Ki = 0.0;
    pid->Kd = 0.0;

    // the tau is set to 1.0 to not affect code in case it is not needed
    pid->tauD = 1.0;
    pid->tauI = 1.0;

    // select signal and system
    pid->signal = (struct Signal*)malloc(sizeof(struct Signal));
    cliSignalSelector(pid->signal);

    
    int size = selectSystem(&pid->func_system);
    
    pid->dataSystem = (float*)malloc(size * sizeof(float));
    for(int i=0; i<size; i++){
        pid->dataSystem[i] = 0.0;
    }
    pid->sizeDataSystem = size;

    // set the output signal
    pid->output = (struct Signal*)malloc(sizeof(struct Signal));
    pid->output->length = pid->signal->length;
    pid->output->dt = pid->signal->dt;
    pid->output->signal = (float*)malloc(pid->signal->length * sizeof(float));

    // now the memory part is set to 0.0
    pid->prevError     = 0.0;
    pid->integralError = 0.0;
}

void deletePid(struct PID *pid){
    deleteSignal(pid->signal);
    deleteSignal(pid->output);

    free(pid->dataSystem);

    free(pid);
}

static void resetOutputMemoryPid(struct PID *pid){
    for(int i=0; i < pid->signal->length; i++){
        pid->output->signal[i] = 0.0;
    }
    for(int i=0; i<pid->sizeDataSystem; i++){
        pid->dataSystem[i] = 0.0;
    }
    pid->dataSystem[1] = pid->signal->dt;

    pid->proportiError = 0.0;
    pid->integralError = 0.0;
    pid->differenError = 0.0; 

    pid->prevError     = 0.0;

    pid->steadyRiseCheck = 1;
    pid->maxCounter = 0;
}

void makeSimulationOfSignal(struct PID *pid, FILE *csvFile, int csv){
    resetOutputMemoryPid(pid);
    pid->fit = 0.0;

    float error;

    if(csv == 1){
        fprintf(csvFile, "%f,%f,%f,%f,%f\n", pid->proportiError, pid->integralError, pid->differenError, pid->output->signal[0], pid->signal->signal[0]);
    }

    // we start from the index 2 due to D parameter which requires the -2 time delay of y[t]
    for(int i=2; i<pid->signal->length; i++){
        // first action desired signal minus current value

        // P = e[t] * Kp
        // I = I + Ki * (e[t] + e[t-1]) * dt
        // D = -(Kd * (y[t-1] - y[t-2]) + (tau - dt) * D)/(tau + dt)
        // minus because it is on y[t] and not e[t]

        // e[t] = w[t] - y[t-1]
        error = pid->signal->signal[i] - pid->output->signal[i-1];
        
        // P = e[t] * Kp
        pid->proportiError = pid->Kp * error;

        // I = I + 0.5Ki * (e[t] + e[t-1]) * dt
        pid->integralError = pid->integralError + 0.5f * pid->Ki * (error - pid->prevError) * pid->signal->dt;

        // D = -(Kd * 2(y[t-1] - y[t-2]) + (2tau - dt) * D) / (2tau + dt)
        pid->differenError = -(2.0f * pid->Kd * (pid->output->signal[i-1] - pid->output->signal[i-2]) + (2.0f * pid->tauD - pid->signal->dt) * pid->differenError) 
                                / (2.0f * pid->tauD + pid->signal->dt);

        if(pid->integralError > pid->limMaxInt){
            pid->integralError =pid->limMaxInt;
        } else if(pid->integralError < pid->limMinInt){
            pid->integralError = pid->limMinInt;
        }

        // set data and pass to system
        pid->dataSystem[0] = pid->proportiError + pid->integralError + pid->differenError;
        
        pid->output->signal[i] = pid->func_system(pid->dataSystem);

        if(pid->output->signal[i] > pid->limMax){
            pid->output->signal[i] = pid->limMax;
            pid->maxCounter++;
        } else if(pid->output->signal[i] < pid->limMin){
            pid->output->signal[i] = pid->limMin;
            pid->maxCounter++;
        }

        if(csv == 1){
            fprintf(csvFile, "%f,%f,%f,%f,%f\n", pid->proportiError, pid->integralError, pid->differenError, pid->output->signal[i], pid->signal->signal[i]);
        }

        // set prevError
        pid->prevError = error;   

        float diff = pid->signal->signal[i] - pid->output->signal[i];
        pid->fit += diff > 0 ? diff : diff * (-1);

        if(pid->output->signal[i-1] > pid->output->signal[i]){
            pid->steadyRiseCheck = 0;
        }
    }

    
    if(pid->steadyRiseCheck == 1){
        pid->fit =  FLT_MAX; // make fit max value to not consider the steady rise solutions
    } else if(pid->maxCounter > pid->signal->length * 1 / 100){
        pid->fit =  FLT_MAX; // same as the steady rise solutions
    }
}