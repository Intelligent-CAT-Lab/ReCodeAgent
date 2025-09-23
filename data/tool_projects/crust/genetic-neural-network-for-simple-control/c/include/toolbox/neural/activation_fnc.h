#ifndef ACTIVATION_FNC_H
#define ACTIVATION_FNC_H

// function to select activation function for pointer
void selectActivationFunction(float (**func_ptr)(float));

// for tests
void selectTangActivationFunction(float (**func_ptr)(float));
void selectSigmActivationFunction(float (**func_ptr)(float));
#endif