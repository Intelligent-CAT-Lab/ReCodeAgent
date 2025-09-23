#include "neural/test_activation_fnc.h"
#include "neural/activation_fnc.h"

#include <stdlib.h>
#include <stdio.h>
#include <math.h>

// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"

static int compareResults(float a, float b){
  if(a == b){
    return 1;
  } 
  return 0;
}

static float testSigmoid(float x) {
    return 1.0 / (1.0 + exp(-x));
}

int testActivationFncTanh(){
  printf(ANSI_BOLD "=======TEST ACTIVAION FNC TANH STARTED=======" ANSI_COLOR_RESET "\n");

  float dataCheck1 = 1.0;
  float dataCheck2 = 23.50;
  float dataCheck3 = 3.333;

  float (*func_ptr_tanh)(float);
  selectTangActivationFunction(&func_ptr_tanh);

  int flag1 = compareResults(tanh(5*dataCheck1), func_ptr_tanh(dataCheck1));
  int flag2 = compareResults(tanh(5*dataCheck2), func_ptr_tanh(dataCheck2));
  int flag3 = compareResults(tanh(5*dataCheck3), func_ptr_tanh(dataCheck3));

  float (*func_ptr_sigm)(float);
  selectSigmActivationFunction(&func_ptr_sigm);

  if(flag1 == 0 || flag2 == 0 || flag3 == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST ACTIVAION FNC TANH FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST ACTIVAION FNC TANH SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}

int testActivationFncSigm(){
  printf(ANSI_BOLD "=======TEST ACTIVAION FNC SIGM STARTED=======" ANSI_COLOR_RESET "\n");

  float dataCheck1 = 1.0;
  float dataCheck2 = 23.50;
  float dataCheck3 = 3.333;

  float (*func_ptr_sigm)(float);
  selectSigmActivationFunction(&func_ptr_sigm);

  int flag1 = compareResults(testSigmoid(dataCheck1), func_ptr_sigm(dataCheck1));
  int flag2 = compareResults(testSigmoid(dataCheck2), func_ptr_sigm(dataCheck2));
  int flag3 = compareResults(testSigmoid(dataCheck3), func_ptr_sigm(dataCheck3));

  if(flag1 == 0 || flag2 == 0 || flag3 == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST ACTIVAION FNC SIGM FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST ACTIVAION FNC SIGM SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}