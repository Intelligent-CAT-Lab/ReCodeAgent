#ifndef MATRIX_H
#define MATRIX_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>

#define TRANSFORM_COUNT 3

/**
 * @brief Represents a square 2D matrix.
 *
 * This structure encapsulates a 2D matrix with its dimensions and data.
 *
 * @param dim The dimension (number of rows/columns) of the matrix.
 * @param data Pointer to the array storing the matrix elements in row-major
 * order.
 */
typedef struct {
  size_t dim;     //!< The dimension (number of rows/columns) of the matrix.
  uint64_t *data; //!< Pointer to the array storing the matrix elements in
                  //!< row-major order.
} Matrix2D;

typedef int (*Transformer)(Matrix2D *m, Matrix2D *invm, uint64_t mod);

/**
 * @brief Represents a 3D matrix.
 *
 * This structure encapsulates a 3D matrix with its dimensions and data.
 *
 * @param data Pointer to the array storing the 2D matrices.
 * @param size The size (number of 2D matrices) in the 3D matrix.
 */
typedef struct {
  Matrix2D *data; //!< Pointer to the array storing the 2D matrices.
  size_t size;    //!< The size (number of 2D matrices) in the 3D matrix.
} Matrix3D;

/**
 * @brief Get a 2D matrix from a 3D matrix.
 *
 * This function retrieves a pointer to a 2D matrix from the specified index of
 * the 3D matrix.
 *
 * @param lambda Pointer to the 3D matrix.
 * @param idx The index of the desired 2D matrix.
 *
 * @return Pointer to the specified 2D matrix.
 */
Matrix2D *matrix3d_get(Matrix3D *lambda, size_t idx);

/**
 * @brief Get a pointer to a specific row in a 2D matrix.
 *
 * This function returns a pointer to the specified row in the given 2D matrix.
 *
 * @param matrix Pointer to the 2D matrix.
 * @param row The index of the desired row.
 *
 * @return Pointer to the specified row.
 */
static inline uint64_t *matrix2d_row(Matrix2D *matrix, size_t row) {
  return matrix->data + row * matrix->dim;
}

/**
 * @brief Get the value at a specific position in a 2D matrix.
 *
 * This function returns the value at the specified row and column in the given
 * 2D matrix.
 *
 * @param matrix Pointer to the 2D matrix.
 * @param row The index of the desired row.
 * @param col The index of the desired column.
 *
 * @return The value at the specified position.
 */
static inline uint64_t matrix2d_get(Matrix2D *matrix, size_t row, size_t col) {
  size_t dim = matrix->dim;
  return matrix->data[row * dim + col];
}

/**
 * @brief Set the value at a specific position in a 2D matrix.
 *
 * This function sets the value at the specified row and column in the given 2D
 * matrix.
 *
 * @param matrix Pointer to the 2D matrix.
 * @param row The index of the desired row.
 * @param col The index of the desired column.
 * @param value The value to set at the specified position.
 */
static inline void matrix2d_set(Matrix2D *matrix, size_t row, size_t col,
                                uint64_t value) {
  size_t dim = matrix->dim;
  matrix->data[row * dim + col] = value;
}

/**
 * @brief Multiply two 2D matrices and store the result.
 *
 * This function multiplies two 2D matrices and stores the result in the
 * provided result matrix, applying the specified modulus to each element.
 *
 * @param m Pointer to the first matrix.
 * @param invm Pointer to the second matrix.
 * @param result Pointer to the matrix structure to store the result.
 * @param mod The modulus to apply to each element.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int matrix2d_multiply(Matrix2D *m, Matrix2D *invm, Matrix2D *result,
                      uint64_t mod);

/**
 * @brief Perform a random swap of rows and corresponding columns between two
 * matrices.
 *
 * This function randomly selects two rows in the matrix `m` and swaps their
 * positions, simultaneously swapping the corresponding columns in the matrix
 * `invm`.
 *
 * @param m Pointer to the first matrix.
 * @param invm Pointer to the second matrix.
 * @param mod The modulus to apply to each element.
 *
 * @return 0 if successful.
 */
int swap_transform(Matrix2D *m, Matrix2D *invm, uint64_t mod);

/**
 * @brief Apply a linear mixing transformation between two matrices.
 *
 * This function applies a linear mixing transformation between two matrices `m`
 * and `invm`. It randomly selects two columns in matrix `m`, and for each row,
 * it adds a scaled value from the source column to the target column, where the
 * scale factor is randomly chosen. The operation is performed modulo `mod`.
 *
 * Additionally, the corresponding rows in matrix `invm` are adjusted to
 * maintain the inverse relationship with matrix `m`.
 *
 * @param m Pointer to the first matrix.
 * @param invm Pointer to the second matrix.
 * @param mod The modulus to apply to each element.
 *
 * @return 0 if successful.
 */
int linear_mix_transform(Matrix2D *m, Matrix2D *invm, uint64_t mod);

/**
 * @brief Apply a scaling transformation to two matrices.
 *
 * This function applies a scaling transformation to the elements of two
 * matrices `m` and `invm`. It randomly selects a scaling factor and its modular
 * inverse, and then scales each element of both matrices by their respective
 * factors. The operation is performed modulo `mod`.
 *
 * @param m Pointer to the first matrix.
 * @param invm Pointer to the second matrix.
 * @param mod The modulus to apply to each element.
 *
 * @return 1 if successful (typo correction: should be 0 if successful).
 */
int scale_transform(Matrix2D *m, Matrix2D *invm, uint64_t mod);

/**
 * @brief Set the given matrix to the identity matrix.
 *
 * This function sets the elements of the given matrix `m` to form an identity
 * matrix. An identity matrix is a square matrix in which all elements are zero
 * except for the diagonal elements, which are set to one.
 *
 * @param m Pointer to the matrix to be set to the identity matrix.
 *
 * @return 0 if successful.
 */
int matrix2d_eye(Matrix2D *m);

/**
 * @brief Fill randomly generated invertible matrix pairs using a set of simple
 * linear transformations.
 *
 * This function fills randomly generated invertible matrix pairs `m` and `invm`
 * using a set of simple linear transformations. For each iteration, it randomly
 * selects a transformation from the available set of transformations and
 * applies it to both matrices `m` and `invm`.
 *
 * The set of available transformations includes swapping rows and columns,
 * scaling each element by a random factor, and applying a linear mixing
 * transformation.
 *
 * @param m Pointer to the first matrix.
 * @param invm Pointer to the second matrix.
 * @param mod The modulus to apply to each element.
 * @param iterations The number of iterations to perform, each representing an
 * application of a transformation.
 */
int fill_random_invertible_pairs(Matrix2D *m, Matrix2D *invm, uint64_t mod,
                                 size_t iterations);

#ifdef __cplusplus
}
#endif

#endif
