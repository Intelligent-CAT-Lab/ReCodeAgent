#include "include/Polynomial.h"
#include "include/Common.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void poly_free(Polynomial *poly) {
  free(poly->coeffs);
  poly->coeffs = NULL;
  poly->size = 0;
}

int get_polynomial(Polynomial *p, size_t size) {
  p->size = size;
  p->coeffs = (Coeff *)malloc(p->size * sizeof(Coeff));
  if (p->coeffs == NULL)
    return -1;
  return 0;
}

int get_polyarray(PolyArray *p, size_t size) {
  p->size = size;
  p->polies = (Polynomial *)malloc(p->size * sizeof(Polynomial));
  if (p->polies == NULL)
    return -1;
  return 0;
}

int set_polynomial(Polynomial *p, Coeff *coeffs, size_t size) {
  p->size = size;
  p->coeffs = coeffs;
  return 0;
}

int set_zero(const Polynomial *p) {
  memset(p->coeffs, 0, p->size * sizeof(Coeff));
  return 0;
}

int64_t coef_sum(const Polynomial *poly) {
  if (!poly || poly->size == 0)
    return 0;

  int64_t sum = 0;
  for (size_t i = 0; i < poly->size; ++i)
    sum += poly->coeffs[i];

  return sum;
}

uint64_t poly_degree(const Polynomial *poly) {
  if (poly->size == 0)
    return 0;

  for (size_t i = 0; i < poly->size; ++i) {
    if (poly->coeffs[i] != 0)
      return poly->size - i - 1;
  }

  return 0;
}

int poly_fit(Polynomial *poly, uint64_t mod) {
  if (!poly || poly->size == 0)
    return -1;

  uint64_t degree = poly_degree(poly);
  uint64_t idx = poly->size - 1 - degree;
  for (size_t i = 0; i + idx < poly->size; ++i)
    poly->coeffs[i] = poly->coeffs[i + idx] % mod;

  poly->size -= idx;
  return 0;
}

int poly_mul(const Polynomial *poly1, const Polynomial *poly2,
             Polynomial *result, uint64_t mod) {
  set_zero(result);
  uint64_t deg1 = poly1->size - 1;
  uint64_t deg2 = poly2->size - 1;
  uint64_t res_deg = result->size - deg2 - deg1 - 1;

  for (size_t i = 0; i < poly1->size; i++) {
    for (size_t j = 0; j < poly2->size; j++) {
      result->coeffs[res_deg + i + j] +=
          (poly1->coeffs[i] * poly2->coeffs[j]) % mod;
    }
  }

  poly_fit(result, mod);
  return 0;
}

int poly_equal(const Polynomial *poly1, const Polynomial *poly2) {
  if (poly1->size != poly2->size)
    return 0;

  return 0 == memcmp(poly1->coeffs, poly2->coeffs, poly1->size * sizeof(Coeff));
}

int poly_add(const Polynomial *poly1, const Polynomial *poly2,
             Polynomial *result, uint64_t mod) {
  if (poly1->size > result->size || poly2->size > result->size || mod == 0)
    return -1;

  if (poly1->size != poly2->size) {
    size_t size = result->size;
    size_t diff1 = size - poly1->size;
    size_t diff2 = size - poly2->size;

    for (size_t i = 0; i < size; ++i) {
      result->coeffs[i] = ((i >= diff1 ? poly1->coeffs[i - diff1] : 0) +
                           (i >= diff2 ? poly2->coeffs[i - diff2] : 0)) %
                          mod;
    }
    return 0;
  }

  for (size_t i = 0; i < poly1->size; ++i)
    result->coeffs[i] = (poly1->coeffs[i] + poly2->coeffs[i]) % mod;
  return 0;
}

int poly_sub(const Polynomial *poly1, const Polynomial *poly2,
             Polynomial *result, uint64_t mod) {
  if (poly1->size > result->size || poly2->size > result->size || mod == 0)
    return -1;

  if (poly1->size != poly2->size) {
    size_t size = result->size;
    size_t diff1 = size - poly1->size;
    size_t diff2 = size - poly2->size;

    for (size_t i = 0; i < size; ++i) {
      result->coeffs[i] = ((i >= diff1 ? poly1->coeffs[i - diff1] : 0) -
                           (i >= diff2 ? poly2->coeffs[i - diff2] : 0)) %
                          mod;
    }
    return 0;
  }

  for (size_t i = 0; i < poly1->size; ++i)
    result->coeffs[i] = (poly1->coeffs[i] - poly2->coeffs[i]) % mod;
  return 0;
}

int poly_lshift(const Polynomial *poly1, const Polynomial *poly2,
                Polynomial *result, uint64_t mod) {
  if (poly2->coeffs[0] != 1)
    return -1;

  uint64_t degree1 = poly_degree(poly1);
  uint64_t degree2 = poly_degree(poly2);

  if (degree1 < degree2)
    return -1;

  Coeff a_d = poly1->coeffs[0];

  for (size_t i = 0; i < poly1->size; ++i) {
    if (i < poly2->size) {
      int64_t res =
          (int64_t)(poly1->coeffs[i] - poly2->coeffs[i] * a_d) % (int64_t)mod;
      result->coeffs[i] = res < 0 ? (Coeff)(res + (int64_t)mod) : (Coeff)res;
    } else
      result->coeffs[i] = poly1->coeffs[i];
  }

  poly_fit(result, mod);
  return 0;
}

int poly_mod(Polynomial *poly1, const Polynomial *poly2, uint64_t mod) {
  while (poly_lshift(poly1, poly2, poly1, mod) == 0) {
  }
  return 0;
}

int poly_sub_scaler(const Polynomial *poly, uint64_t scaler, Polynomial *result,
                    uint64_t mod) {
  for (size_t i = 0; i < poly->size; ++i)
    result->coeffs[i] = (poly->coeffs[i] - scaler) % mod;
  return 0;
}

int poly_add_scaler(const Polynomial *poly, uint64_t scaler, Polynomial *result,
                    uint64_t mod) {
  for (size_t i = 0; i < poly->size; ++i)
    result->coeffs[i] = (poly->coeffs[i] + scaler) % mod;
  return 0;
}
