#include "aces-internal-test.h"
#include "common-test.h"
#include "polynomial-test.h"

#include <assert.h>

int main(void) {
  assert(run_polynomial_tests());
  assert(run_aces_internal_tests());
  assert(run_common_tests());
  return 0;
}
