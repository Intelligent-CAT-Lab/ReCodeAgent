#include "general/sort.h"

#include <stdio.h> 
#include <stdlib.h> 
#include <time.h> 
#include <stdbool.h>

void quickSort(float *fit, int *input, int length){
  // in this sort the fit is only used as a datasource for the result
  // the length is the data for the result, not fit
  // the isea is that the list of values will be recursed so that in result the
  // input - the array of indexes which should be sorted based on fit

  // first the indexes as well as arrays for left and right are created
  int index = length / 2;

  int indexMore = 0;
  int indexLess = 0;

  int *arrayMore = (int*)malloc(length* sizeof(int));
  int *arrayLess = (int*)malloc(length* sizeof(int));

  // now the check is made and arrays are filled with values
  for(int i=0; i<length; i++){
    if(i != index){
      // logic is, indexes with more equal are going right and less going left
      if(fit[input[i]] >= fit[input[index]]){
        arrayMore[indexMore] = input[i];
        indexMore++;
      } else if(fit[input[i]] < fit[input[index]]){
        arrayLess[indexLess] = input[i];
        indexLess++;
      }
    }
  }
  // now we have two lists with indexes less and more. Except of the midle one

  arrayMore = (int*)realloc(arrayMore, indexMore*sizeof(int));
  arrayLess = (int*)realloc(arrayLess, indexLess*sizeof(int));


  // now the recursion part should be made
  if(indexMore > 1){
    quickSort(fit, arrayMore, indexMore);
  }

  if(indexLess > 1){
    quickSort(fit, arrayLess, indexLess);
  }

  int midle = input[index];
  int indexGlobal = 0;
  // first the less part is updates 
  for(int i=0; i<indexLess; i++){
    input[indexGlobal] = arrayLess[i];
    indexGlobal++;
  }

  // then midle added
  input[indexGlobal] = midle;
  indexGlobal++;

  // last the more part is updates 
  for(int i=0; i<indexMore; i++){
    input[indexGlobal] = arrayMore[i];
    indexGlobal++;
  }

  free(arrayLess);
  free(arrayMore);
}
