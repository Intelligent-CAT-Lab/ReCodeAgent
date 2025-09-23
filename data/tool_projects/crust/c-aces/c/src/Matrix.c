#include "include/Matrix.h"
#include "include/Common.h"
#include <string.h>

Matrix2D *matrix3d_get(Matrix3D *lambda, size_t idx) {
  return &lambda->data[idx];
}

int matrix2d_multiply(Matrix2D *m, Matrix2D *invm, Matrix2D *result,
                      uint64_t mod) {
  size_t dim = m->dim;
  for (size_t i = 0; i < dim; i++) {
    for (size_t j = 0; j < dim; j++) {
      matrix2d_set(result, i, j, 0);
      for (size_t k = 0; k < dim; k++) {
        matrix2d_set(result, i, j,
                     matrix2d_get(result, i, j) +
                         matrix2d_get(m, i, k) * matrix2d_get(invm, k, j));
      }
      matrix2d_set(result, i, j, matrix2d_get(result, i, j) % mod);
    }
  }

  return 0;
}

int swap_transform(Matrix2D *m, Matrix2D *invm, MAYBE_UNUSED uint64_t mod) {
  size_t dim = m->dim;
  uint64_t tmp;
  uint64_t tmp_row[dim];

  size_t source = randrange(1, dim - 1);
  size_t target = randrange(0, source - 1);

  for (size_t row = 0; row < dim; ++row) {
    tmp = matrix2d_get(m, row, source);
    matrix2d_set(m, row, source, matrix2d_get(m, row, target));
    matrix2d_set(m, row, target, tmp);
  }

  memcpy(tmp_row, matrix2d_row(invm, source), sizeof(uint64_t) * dim);
  memcpy(matrix2d_row(invm, source), matrix2d_row(invm, target),
         sizeof(uint64_t) * dim);
  memcpy(matrix2d_row(invm, target), tmp_row, sizeof(uint64_t) * dim);

  return 0;
}

int linear_mix_transform(Matrix2D *m, Matrix2D *invm, uint64_t mod) {
  size_t dim = m->dim;
  size_t scale = randrange(1, mod - 1);
  size_t source = randrange(1, dim - 1);
  size_t target = randrange(0, source - 1);

  for (size_t row = 0; row < dim; ++row) {
    matrix2d_set(
        m, row, target,
        (matrix2d_get(m, row, target) + matrix2d_get(m, row, source) * scale) %
            mod);
    matrix2d_set(invm, source, row,
                 (matrix2d_get(invm, source, row) +
                  matrix2d_get(invm, target, row) * (mod - scale)) %
                     mod);
  }

  return 0;
}

int scale_transform(Matrix2D *m, Matrix2D *invm, uint64_t mod) {
  size_t dim = m->dim;
  Pair scale = randinverse(mod);
  for (size_t idx = 0; idx < dim * dim; ++idx) {
    m->data[idx] = (m->data[idx] * scale.first) % mod;
    invm->data[idx] = (invm->data[idx] * scale.second) % mod;
  }
  return 1;
}

int matrix2d_eye(Matrix2D *m) {
  size_t dim = m->dim;
  memset(m->data, 0, dim * dim * sizeof(uint64_t));

  for (size_t row = 0; row < dim; ++row)
    matrix2d_set(m, row, row, 1);

  return 0;
}

int fill_random_invertible_pairs(Matrix2D *m, Matrix2D *invm, uint64_t mod,
                                 size_t iterations) {
  const Transformer transformers[TRANSFORM_COUNT] = {
      &swap_transform, &scale_transform, &linear_mix_transform};

  matrix2d_eye(m);
  matrix2d_eye(invm);

  for (size_t i = 0; i < iterations; ++i) {
    size_t select = randrange(0, TRANSFORM_COUNT - 1);
    transformers[select](m, invm, mod);
  }
  return 0;
}
