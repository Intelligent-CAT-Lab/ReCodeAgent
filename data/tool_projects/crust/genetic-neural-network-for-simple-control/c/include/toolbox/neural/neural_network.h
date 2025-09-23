#ifndef NN_H
#define NN_H

#include "general/matrix_math.h"

// struct used to store and use NN
typedef struct NN{
    struct Matrix **AW; // all layers vweights connecting them (matrix of matrixes)
    struct Matrix **BW; // all values of biases of each level

    int *neuronsSize;  // matrix of number of neurons per layer incl. input and output
    int  layerNumber;  // number representing the number of layers incl. input and output
    // using layer_number we can get all values of neurons_size, from which we can get respective matrixes sizes

    int  countOfValues; // number of values needed to fill all matrixes, is used to generate population

    float **normalizationMatrix;   // matrix of all normalization values max and min. Max is on 0 row, min on 1
    float **denormalizationMatrix; // matrix of all denormalization values max and mins 
    // number of normalization max/min pairs should be same as the number of inpputs. Same for denormalization and outputs

    float (*func_ptr)(float); // activation function pointer

    int    *layerType; // array showing the type of each HL 0 - FF, 1 - SD

    int   **sdNeuronsTypes;   // matrix containing type for each neuron in the SD layer 0 - straight, 1 - S, 2 - D
    struct Matrix **SDMemory; // array of Matrixes for each SD layer
}NN;

// struct used to deffine needed values for NN to be created
typedef struct NNInput{
    int *neuronsSize; // matrix of number of neurons per layer incl. input and output
    int  layerNumber; // number representing the number of layers incl. input and output

    float **normalizationMatrix;  // matrix of all normalization values max and min. Max is on 0 row, min on 1
    float **denormalizationMatrix; // matrix of all denormalization values max and mins 

    int    *layerType; // array showing the type of each HL 0 - FF, 1 - SD

    int  sdNumber;           // the number of sd layers
}NNInput;

// function to create new neural network out of input structure and deletion of the input structure at the end
void createNeuralNetwork(struct NNInput *input, struct NN *neuralNetwork);

// function to delete neural network 
void clearNeuralNetwork(struct NN *neuralNetwork);

// function to calculate the output of network based on the input
void oneCalculation(struct NN *neuralNetwork, struct Matrix *input, struct Matrix *output);

// function to set all matrixes based on one row of population
void fillMatrixesNN(struct NN *neuralNetwork, float *population);

void deNormalizationProcess(struct NN *neuralNetwork, struct Matrix *input, int way);

// function to clear SD
void clearSDMemory(struct NN *neuralNetwork);

#endif