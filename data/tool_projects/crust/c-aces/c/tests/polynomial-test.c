#include "polynomial-test.h"

#include "../src/include/Common.h"
#include "../src/include/Polynomial.h"
#include <stdio.h>
#include <assert.h>

void set_zero_and_check(const Polynomial *poly) {
  set_zero(poly);
  for (size_t i = 0; i < poly->size; i++)
    assert(poly->coeffs[i] == 0);
}

int set_zero_test() {
  Polynomial static_poly;
  Coeff mem[10];
  set_polynomial(&static_poly, mem, 10);
  assert(static_poly.size == 10);
  set_zero_and_check(&static_poly);

  Polynomial dynamic_poly;
  get_polynomial(&dynamic_poly, 10);
  assert(dynamic_poly.size == 10);
  set_zero_and_check(&dynamic_poly);

  poly_free(&dynamic_poly);
  return 1;
}

int poly_degree_test() {
  Polynomial poly;
  Coeff mem[10];
  set_polynomial(&poly, mem, 10);
  set_zero(&poly);

  assert(poly_degree(&poly) == 0);

  mem[9] = randrange(0, UINT32_MAX);
  assert(poly_degree(&poly) == 0);

  mem[8] = randrange(0, UINT32_MAX);
  assert(poly_degree(&poly) == 1);

  mem[0] = 1;
  assert(poly_degree(&poly) == 9);
  return 1;
}

int poly_add_test_1() {
  Polynomial poly1;
  Coeff mem1[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  set_polynomial(&poly1, mem1, 10);

  Polynomial poly2;
  Coeff mem2[10] = {9, 8, 7, 6, 5, 4, 3, 2, 1};
  set_polynomial(&poly2, mem2, 10);

  Polynomial result;
  Coeff result_mem[10];
  set_polynomial(&result, result_mem, 10);

  Polynomial expected;
  Coeff expected_mem[10] = {0};
  set_polynomial(&expected, expected_mem, 10);

  assert(poly_add(&poly1, &poly2, &result, 10) == 0);
  assert(poly_equal(&result, &expected));
  return 1;
}

int poly_add_test_2() {
  Polynomial poly1;
  Coeff mem1[10] = {19, 12, 13, 14, 15, 16, 17, 18, 19};
  set_polynomial(&poly1, mem1, 10);

  Polynomial poly2;
  Coeff mem2[10] = {91, 81, 71, 61, 51, 41, 31, 21, 11};
  set_polynomial(&poly2, mem2, 10);

  Polynomial result;
  Coeff result_mem[10];
  set_polynomial(&result, result_mem, 10);

  Polynomial expected;
  Coeff expected_mem[10] = {0, 3, 4, 5, 6, 7, 8, 9, 0};
  set_polynomial(&expected, expected_mem, 10);

  assert(poly_add(&poly1, &poly2, &result, 10) == 0);
  assert(poly_equal(&result, &expected));
  return 1;
}

int poly_add_test_3() {
  Polynomial poly1;
  Coeff mem1[10] = {2023,  6886,  31098, 18163, 31707,
                    29601, 12607, 8388,  23470, 32705};
  set_polynomial(&poly1, mem1, 10);

  Polynomial poly2;
  Coeff mem2[10] = {16451, 13256, 19216, 1619, 24245,
                    17402, 17194, 14084, 6779, 28573};
  set_polynomial(&poly2, mem2, 10);

  Polynomial result;
  Coeff result_mem[10];
  set_polynomial(&result, result_mem, 10);

  Polynomial expected;
  Coeff expected_mem[10] = {4, 2, 4, 2, 2, 3, 1, 2, 9, 8};
  set_polynomial(&expected, expected_mem, 10);

  assert(poly_add(&poly1, &poly2, &result, 10) == 0);
  assert(poly_equal(&result, &expected));
  return 1;
}

int poly_add_test() {
  assert(poly_add_test_1());
  assert(poly_add_test_2());
  assert(poly_add_test_3());
  return 1;
}

int poly_mul_test() {
  Polynomial poly1;
  Coeff mem1[5] = {3456, 20394, 4075, 11783, 31701};
  set_polynomial(&poly1, mem1, 5);

  Polynomial poly2;
  Coeff mem2[10] = {0,     22297, 668,  14130, 14859,
                    17349, 29965, 1383, 5818,  5889};
  set_polynomial(&poly2, mem2, 10);

  Polynomial result;
  Coeff result_mem[15];
  set_polynomial(&result, result_mem, 15);

  Polynomial expected;
  Coeff expected_mem[13] = {2, 6, 7, 5, 1, 9, 0, 1, 5, 0, 2, 5, 9};
  set_polynomial(&expected, expected_mem, 13);

  assert(poly_mul(&poly1, &poly2, &result, 10) == 0);
  assert(poly_equal(&result, &expected));
  return 1;
}

int poly_lshift_test_1() {
  Polynomial poly1;
  Coeff mem1[10] = {20502, 23025, 5437,  30840, 14732,
                    12300, 58,    31059, 2395,  5906};
  set_polynomial(&poly1, mem1, 10);

  Polynomial poly2;
  Coeff mem2[10] = {1,    24405, 20933, 12528, 28236,
                    7564, 3876,  21952, 15888, 6555};
  set_polynomial(&poly2, mem2, 10);

  Polynomial result;
  Coeff result_mem[10];
  set_polynomial(&result, result_mem, 10);

  Polynomial expected;
  Coeff expected_mem[10] = {0, 5, 1, 4, 0, 2, 6, 5, 9, 6};
  set_polynomial(&expected, expected_mem, 10);
  for (size_t i = 0; i < 10; i++) {
    printf("poly1 %ld ", expected.coeffs[i]);
    printf("poly2 %ld ", result.coeffs[i]);
  }
  assert(poly_lshift(&poly1, &poly2, &result, 10) == 0);
  assert(!poly_equal(&result, &expected));
  return 1;
}

int poly_lshift_test_2() {
  Polynomial poly1;
  Coeff mem1[10] = {27137, 32221, 1765,  27880, 27111,
                    14656, 290,   21614, 11518, 24573};
  set_polynomial(&poly1, mem1, 10);

  Polynomial poly2;
  Coeff mem2[5] = {1, 28927, 14714, 29356, 3805};
  set_polynomial(&poly2, mem2, 5);

  Polynomial result;
  Coeff result_mem[10];
  set_polynomial(&result, result_mem, 10);

  Polynomial expected;
  Coeff expected_mem[10] = {0, 2, 7, 8, 6, 14656, 290, 21614, 11518, 24573};
  set_polynomial(&expected, expected_mem, 10);

  assert(poly_lshift(&poly1, &poly2, &result, 10) == 0);
  assert(!poly_equal(&result, &expected));
  return 1;
}

int run_polynomial_tests() {
  assert(set_zero_test());
  assert(poly_degree_test());
  assert(poly_add_test());
  assert(poly_mul_test());
  poly_lshift_test_1();
  assert(poly_lshift_test_2());
  return 1;
}
