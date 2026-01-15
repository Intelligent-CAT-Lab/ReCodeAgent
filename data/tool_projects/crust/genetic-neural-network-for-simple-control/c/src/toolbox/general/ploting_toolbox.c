#include "general/ploting_toolbox.h"

#include "general/signal_designer.h"

#include <stdio.h>
#include <stdlib.h>



void plotGraph() {

    const char *command = "python3 data/python/ploting_code.py";

    int status = system(command);
}


void plotGraphNN() {

    const char *command = "python3 data/python/ploting_code_NN.py";

    int status = system(command);
}