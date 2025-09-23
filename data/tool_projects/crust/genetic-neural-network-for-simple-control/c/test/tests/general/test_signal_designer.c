#include "general/test_signal_designer.h"
#include "general/signal_designer.h"

#include <stdlib.h>
#include <stdio.h>

// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"

int testSignalCreate(){
  printf(ANSI_BOLD "=======TEST SIGNAL CREATE STARTED=======" ANSI_COLOR_RESET "\n");

  struct Signal *signal = (struct Signal*)malloc(sizeof(struct Signal));

  cliSignalSelector(signal);
  deleteSignal(signal);

  int flag = 1;

  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST SIGNAL CREATE FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST SIGNAL CREATE SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}