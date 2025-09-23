#include "general/test_sort.h"
#include "general/sort.h"

#include <stdlib.h>
#include <stdio.h>

// used to print testing outputs
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_BOLD         "\x1b[1m"
#define ANSI_COLOR_RESET   "\x1b[0m"

int testQuicksort(){
  printf(ANSI_BOLD "=======TEST QUICK SORT STARTED=======" ANSI_COLOR_RESET "\n");

  int length = 5;
  float fit[] = {9.6, 3.4, 10, 4.6, 0.4};
  int *result = (int*)malloc(length*sizeof(int));
  for(int i=0; i<length; i++){
    result[i] = i;
  }

  int resultCorrect[] = {4, 1, 3, 0, 2};

  quickSort(fit, result, length);

  int flag = 1;
  for(int i=0; i<length; i++){
    printf("%d - %f\n", result[i], fit[result[i]]);
    if(resultCorrect[i] != result[i]){
      flag = 0;
    }
  }
  free(result);

  if(flag == 0){
    printf(ANSI_BOLD ANSI_COLOR_RED "=======TEST QUICK SORT FAILED=======" ANSI_COLOR_RESET "\n");
    return 0;
  }
  printf(ANSI_BOLD ANSI_COLOR_GREEN "=======TEST QUICK SORT SUCCESSFUL=======" ANSI_COLOR_RESET "\n");
  return 1;
}