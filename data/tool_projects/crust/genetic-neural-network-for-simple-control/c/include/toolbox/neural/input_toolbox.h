#ifndef INPUT_TOOLBOX_H
#define INPUT_TOOLBOX_H

#include "neural/model_system.h"

// function to select activation function for pointer
void selectInputNNFunction(void (**func_ptr)(float*), struct SystemNN *systemInput);

#endif