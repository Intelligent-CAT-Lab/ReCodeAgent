#include "include/Aces-internal.h"
#include "include/Common.h"

#include <string.h>

int generate_error(uint64_t q, uint64_t message, Polynomial *rm) {
  for (size_t i = 0; i < rm->size; ++i)
    rm->coeffs[i] = randrange(0, q);

  Coeff shift =
      (rm->coeffs[rm->size - 1] + ((Coeff)message - (Coeff)coef_sum(rm))) %
      (Coeff)q;
  rm->coeffs[rm->size - 1] = shift > 0 ? shift : shift + (Coeff)q;
  return 0;
}

uint64_t generate_vanisher(uint64_t p, uint64_t q, Polynomial *e) {
  int64_t k = randrange(0, 1) < EPROB ? 0 : 1;
  for (size_t i = 0; i < e->size; ++i)
    e->coeffs[i] = randrange(0, q);

  Coeff shift =
      (e->coeffs[e->size - 1] + (Coeff)p * k - coef_sum(e)) % (Coeff)q;
  e->coeffs[e->size - 1] = shift > 0 ? shift : shift + (Coeff)q;
  return k;
}

uint64_t generate_linear(uint64_t p, uint64_t q, Polynomial *b) {
  int64_t k = randrange(0, p);
  for (size_t i = 0; i < b->size; ++i)
    b->coeffs[i] = randrange(0, q);

  Coeff shift = (b->coeffs[b->size - 1] + k - coef_sum(b)) % (Coeff)q;
  b->coeffs[b->size - 1] = shift > 0 ? shift : shift + (Coeff)q;
  return k;
}

int generate_u(const Channel *channel, const Parameters *param, Polynomial *u) {
  uint64_t nonzeros = randrange(param->dim / 2, param->dim - 1);
  uint64_t zeroes = param->dim - nonzeros;
  u->coeffs[0] = 1;

  for (size_t i = 1; i < param->dim;) {
    if (nonzeros > 1) {
      u->coeffs[i++] = randrange(0, channel->q);
      nonzeros--;
    } else {
      break;
    }

    if (zeroes > 0) {
      uint64_t samp = clamp(0, zeroes, normal_rand(zeroes / 2, zeroes / 2));
      uint64_t zero = randrange(0, samp);
      for (size_t j = 0; j < zero; ++j)
        u->coeffs[i++] = 0;
      zeroes -= zero;
    }
  }

  u->coeffs[param->dim] = 0;
  u->coeffs[param->dim] = (int64_t)channel->q - (int64_t)coef_sum(u);
  return 0;
}

int generate_secret(const Channel *channel, const Parameters *param,
                    const Polynomial *u, PolyArray *secret, Matrix3D *lambda) {
  Matrix2D m;
  Matrix2D invm;

  m.dim = param->dim;
  invm.dim = param->dim;

  size_t required_mem = m.dim * m.dim;
  uint64_t mem[2 * required_mem];
  m.data = mem;
  invm.data = mem + required_mem;

  fill_random_invertible_pairs(&m, &invm, channel->q, 600);

  secret->size = m.dim;
  for (size_t i = 0; i < m.dim; ++i) {
    for (size_t j = 0; j < m.dim; ++j) {
      secret->polies[i].coeffs[j] = matrix2d_get(&m, j, i);
    }
  }

  Matrix2D *arr = &m;
  Polynomial a_ij_poly;

  Coeff miim[2 * secret->size];

  for (size_t i = 0; i < secret->size; ++i) {
    for (size_t j = 0; j < secret->size; ++j) {
      set_polynomial(&a_ij_poly, miim, 2 * secret->size);
      poly_mul(&secret->polies[i], &secret->polies[j], &a_ij_poly, channel->q);
      poly_mod(&a_ij_poly, u, channel->q);

      for (size_t row = 0; row < arr->dim; row++) {
        matrix2d_set(arr, row, j, a_ij_poly.coeffs[arr->dim - row - 1]);
      }
    }

    matrix2d_multiply(&invm, arr, matrix3d_get(lambda, i), channel->q);
  }

  return 0;
}

int generate_f0(const Channel *channel, const Parameters *param,
                PolyArray *f0) {
  (void)param; // TODO: we need to consider N
  for (size_t i = 0; i < f0->size; ++i) {
    for (size_t j = 0; j < f0->polies[i].size; j++)
      f0->polies[i].coeffs[j] = randrange(0, channel->q - 1);
  }

  return 0;
}

int generate_f1(const Channel *channel, const Parameters *param,
                const PolyArray *f0, const PolyArray *x, const Polynomial *u,
                Polynomial *f1) {
  Polynomial tmp;
  Coeff mem[param->dim * 2];

  Polynomial f_pre;
  Coeff f_pre_mem[param->dim * 2];
  set_polynomial(&f_pre, f_pre_mem, param->dim * 2);

  set_zero(&f_pre);

  for (size_t i = 0; i < param->dim; ++i) {
    set_polynomial(&tmp, mem, param->dim * 2);
    poly_mul(&f0->polies[i], &x->polies[i], &tmp, channel->q);
    poly_add(&f_pre, &tmp, &f_pre, channel->q);
  }
  poly_mod(&f_pre, u, channel->q);

  size_t diff = f1->size - f_pre.size;
  f1->coeffs += diff;
  f1->size -= diff;
  memcpy(f1->coeffs, f_pre.coeffs, f_pre.size * sizeof(Coeff));

  return 0;
}
