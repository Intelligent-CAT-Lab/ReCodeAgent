#include "aces-internal-test.h"
#include "../src/include/Aces-internal.h"
#include "../src/include/Channel.h"
#include "../src/include/Polynomial.h"
#include <stdio.h>
#include <assert.h>

int generate_u_test() {
  Polynomial u;
  Coeff mem[11];
  set_polynomial(&u, mem, 11);
  set_zero(&u);

  Channel channel;
  init_channel(&channel, 2, 33, 1);

  Parameters param;
  param.dim = 10;

  generate_u(&channel, &param, &u);
  assert(coef_sum(&u) == 33);

  return 1;
}

int generate_f0_test() {
  Polynomial polies[10];

  PolyArray f0;
  f0.size = 10;
  f0.polies = polies;

  Coeff mem[10][10];
  for (int i = 0; i < 10; ++i)
    set_polynomial(&polies[i], mem[i], 10);

  Channel channel;
  init_channel(&channel, 2, 33, 1);
  Parameters param;
  param.dim = 10;

  generate_f0(&channel, &param, &f0);

  return 1;
}

int generate_secret_test() {
  Channel channel;
  init_channel(&channel, 2, 33, 1);

  Parameters param;
  param.dim = 10;

  Coeff mem_u[11];
  Polynomial u;
  set_polynomial(&u, mem_u, 11);
  set_zero(&u);
  generate_u(&channel, &param, &u);

  Polynomial polies[10];
  Coeff mem_secret[10][10];
  for (int i = 0; i < 10; ++i)
    set_polynomial(&polies[i], mem_secret[i], 10);
  PolyArray secret;
  secret.size = 10;
  secret.polies = polies;

  Matrix2D data[10];
  uint64_t mem_lambda[10][100];
  for (int i = 0; i < 10; ++i) {
    data[i].data = mem_lambda[i];
    data[i].dim = 10;
  }
  Matrix3D lambda;
  lambda.size = 10;
  lambda.data = data;

  generate_secret(&channel, &param, &u, &secret, &lambda);
  return 1;
}

int run_aces_internal_tests() {
  assert(generate_u_test());
  assert(generate_f0_test());
  assert(generate_secret_test());
  printf("tests passed!\n");
  return 1;
}
