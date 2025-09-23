#include "include/Common.h"

#include <math.h>
#include <stdlib.h>

#define PI 3.14159265

static double rand_val() { return (double)rand() / (double)RAND_MAX; }

double normal_rand(double mean, double stddev) {
  double u, r, theta; // Variables for Box-Muller method
  double x;           // Normal(0, 1) rv
  double norm_rv;     // The adjusted normal rv

  // Generate u
  u = 0.0;
  while (u == 0.0)
    u = rand_val();

  r = sqrt(-2.0 * log(u));

  // Generate theta
  theta = 0.0;
  while (theta == 0.0)
    theta = 2.0 * PI * rand_val();

  x = r * cos(theta);

  // Adjust x value for specified mean and variance
  norm_rv = (x * stddev) + mean;

  return norm_rv;
}

uint64_t gcd(uint64_t x, uint64_t y) {
  while (x != y) {
    if (x > y)
      x -= y;
    else
      y -= x;
  }

  return x;
}

Xgcd xgcd(uint64_t x, uint64_t y) {
  int64_t prev_a = 1;
  int64_t a = 0;
  int64_t prev_b = 0;
  int64_t b = 1;

  while (y != 0) {
    int64_t q = (int64_t)(x / y);
    int64_t temp = (int64_t)(x % y);
    x = y;
    y = (uint64_t)temp;

    temp = a;
    a = prev_a - (q * a);
    prev_a = temp;

    temp = b;
    b = prev_b - (q * b);
    prev_b = temp;
  }

  Xgcd result = {.gcd = x, .a = prev_a, .b = prev_b};
  return result;
}

int are_coprime(uint64_t x, uint64_t y) { return !(gcd(x, y) > 1); }

uint64_t randrange(uint64_t lower, uint64_t upper) {
  return (rand() % (upper - lower + 1)) + lower;
}

Pair randinverse(uint64_t value) {
  uint64_t a = randrange(2, value - 1);
  while (!are_coprime(a, value))
    a = randrange(2, value - 1);

  Xgcd result = xgcd(a, value);

  Pair p = {.first = a,
            .second = result.a > 0 ? (uint64_t)result.a
                                   : (uint64_t)(result.a + (int64_t)value)};
  return p;
}
