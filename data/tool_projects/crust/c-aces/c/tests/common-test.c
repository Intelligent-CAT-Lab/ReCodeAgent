#include "common-test.h"
#include <../src/include/Common.h>
#include <assert.h>
#include <stdio.h>
#include <string.h>

#define MAYBE_UNUSED __attribute__((unused))

int gcd_test() {
  assert(1ULL == gcd(1, 1));
  assert(1ULL == gcd(2, 1));
  assert(1ULL == gcd(1, 2));
  assert(2ULL == gcd(2, 2));
  assert(3ULL == gcd(6, 15));
  assert(3ULL == gcd(15, 6));
  assert(1ULL == gcd(7, 15));
  assert(1ULL == gcd(15, 7));
  assert(1ULL == gcd(7, 15));
  assert(3ULL == gcd(11112, 44445));
  return 1;
}

int xgcd_test() {
  MAYBE_UNUSED Xgcd result;

  result = xgcd(7, 7);
  assert(result.gcd == 7);
  assert(result.a == 0);
  assert(result.b == 1);

  result = xgcd(2, 2);
  assert(result.gcd == 2);
  assert(result.a == 0);
  assert(result.b == 1);

  result = xgcd(1, 1);
  assert(result.gcd == 1);
  assert(result.a == 0);
  assert(result.b == 1);

  result = xgcd(1, 2);
  assert(result.gcd == 1);
  assert(result.a == 1);
  assert(result.b == 0);

  result = xgcd(5, 6);
  assert(result.gcd == 1);
  assert(result.a == -1);
  assert(result.b == 1);

  result = xgcd(19, 13);
  assert(result.gcd == 1);
  assert(result.a == -2);
  assert(result.b == 3);

  return 1;
}

int run_common_tests() {
  assert(gcd_test());
  assert(xgcd_test());
  return 1;
}
