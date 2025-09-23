#include "../src/fft.h"

#include <stdio.h>
#include <assert.h>
#define ARRAYSIZE(arr)  (sizeof (arr) / sizeof ((arr)[0]))

static fft_complex_t data[] = {
  { 1.0, 0.0 },
  { -1.0, 0.0 },
  { 1.0, 0.0 },
  { -1.0, 0.0 },
  { 1.0, 0.0 },
  { -1.0, 0.0 },
  { 1.0, 0.0 },
  { -1.0, 0.0 },
};

static fft_complex_t output[] = {
    { 0.0, 0.0 },
    { 0.0, 0.0 },
    { 0.0, 0.0 },
    { 0.0, 0.0 },
    { 8.0, 0.0 },
    { 0.0, 0.0 },
    { 0.0, 0.0 },
    { 0.0, 0.0 },
};

int main(void) {
  
  fft_inplace(data, 3);
  for (size_t i = 0; i < ARRAYSIZE(data); ++i) {
    assert(data[i].real == output[i].real);
  }
  printf("Test passed\n");
}
