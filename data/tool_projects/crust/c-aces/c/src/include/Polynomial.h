#ifndef POLYNOMIAL_H
#define POLYNOMIAL_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>

/// Typedef for coefficient in polynomial representation.
typedef int64_t Coeff;

/**
 * @brief Represents a polynomial.
 *
 * This structure encapsulates the coefficients of a polynomial and its size,
 * providing a convenient way to work with polynomial data in algorithms
 * and cryptographic operations.
 *
 * The coeffs field holds a pointer to an array of coefficients representing the
 * polynomial terms, while the size field indicates the size of the coefficients
 * array, corresponding to the degree of the polynomial plus one.
 */
typedef struct {
  Coeff *coeffs; //!< Pointer to array of coefficients
  size_t size;   //!< Size of the coefficients array
} Polynomial;

/**
 * @brief Represents an array of polynomials.
 *
 * This structure encapsulates an array of Polynomial structs and its size,
 * providing a convenient way to work with multiple polynomials collectively.
 *
 * The polies field holds a pointer to an array of Polynomial structs,
 * while the size field indicates the number of polynomials in the array.
 */
typedef struct {
  Polynomial *polies; //!< Pointer to array of Polynomial structs
  size_t size;        //!< Number of polynomials in the array
} PolyArray;

/**
 * @brief Check equality between two polynomials.
 *
 * This function compares two polynomial structures to determine if they are
 * equal.
 *
 * @param p1 Pointer to the first polynomial.
 * @param p2 Pointer to the second polynomial.
 *
 * @return 1 if the polynomials are equal, 0 otherwise.
 */
int poly_equal(const Polynomial *, const Polynomial *);

/**
 * @brief Free memory allocated for a polynomial.
 *
 * This function releases the memory allocated for the coefficients array of a
 * polynomial.
 *
 * @param poly Pointer to the polynomial to be freed.
 */
void poly_free(Polynomial *poly);

/**
 * @brief Set coefficients for a polynomial.
 *
 * This function sets the coefficients for a given polynomial structure.
 *
 * @param p Pointer to the polynomial structure.
 * @param coeffs Pointer to an array of coefficients.
 * @param size Size of the coefficients array.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int set_polynomial(Polynomial *p, Coeff *coeffs, size_t size);

/**
 * @brief Allocate memory for a polynomial using malloc.
 *
 * This function allocates memory for a polynomial structure using malloc.
 *
 * @param p Pointer to the polynomial structure.
 * @param size Size of the polynomial.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int get_polynomial(Polynomial *p, size_t size);

int get_polyarray(PolyArray *p, size_t size);

/**
 * @brief Set all coefficients of a polynomial to zero.
 *
 * This function sets all coefficients of a given polynomial to zero.
 *
 * @param p Pointer to the polynomial structure.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int set_zero(const Polynomial *p);

/**
 * @brief Calculate the sum of coefficients in a polynomial.
 *
 * This function calculates the sum of all coefficients in the given polynomial.
 *
 * @param poly Pointer to the polynomial structure.
 *
 * @return The sum of coefficients in the polynomial.
 */
int64_t coef_sum(const Polynomial *poly);

/**
 * @brief Calculate the degree of a polynomial.
 *
 * This function calculates the degree of the given polynomial.
 *
 * @param poly Pointer to the polynomial structure.
 *
 * @return The degree of the polynomial.
 */
uint64_t poly_degree(const Polynomial *poly);

/**
 * @brief Fit polynomial to a specified modulus and remove leading zeros.
 *
 * This function adjusts the coefficients of the polynomial to fit within the
 * specified modulus and removes leading zeros from the highest degrees (left of
 * array).
 *
 * @param poly Pointer to the polynomial structure.
 * @param mod The modulus to fit the coefficients within.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int poly_fit(Polynomial *poly, uint64_t mod);

/**
 * @brief Multiply two polynomials and store the result.
 *
 * This function multiplies two polynomials and stores the result in the
 * provided result polynomial, applying the specified modulus to each
 * coefficient.
 *
 * @param poly1 Pointer to the first polynomial.
 * @param poly2 Pointer to the second polynomial.
 * @param result Pointer to the polynomial structure to store the result.
 * @param mod The modulus to apply to each coefficient.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int poly_mul(const Polynomial *poly1, const Polynomial *poly2,
             Polynomial *result, uint64_t mod);

/**
 * @brief Add two polynomials and store the result.
 *
 * This function adds two polynomials and stores the result in the provided
 * result polynomial, applying the specified modulus to each coefficient.
 *
 * @param poly1 Pointer to the first polynomial.
 * @param poly2 Pointer to the second polynomial.
 * @param result Pointer to the polynomial structure to store the result.
 * @param mod The modulus to apply to each coefficient.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int poly_add(const Polynomial *poly1, const Polynomial *poly2,
             Polynomial *result, uint64_t mod);

/**
 * @brief Subtract one polynomial from another and store the result.
 *
 * This function subtracts the second polynomial from the first polynomial and
 * stores the result in the provided result polynomial, applying the specified
 * modulus to each coefficient.
 *
 * @param poly1 Pointer to the first polynomial.
 * @param poly2 Pointer to the second polynomial.
 * @param result Pointer to the polynomial structure to store the result.
 * @param mod The modulus to apply to each coefficient.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int poly_sub(const Polynomial *poly1, const Polynomial *poly2,
             Polynomial *result, uint64_t mod);

/**
 * @brief Left shift one polynomial by another and store the result.
 *
 * This function left shifts the first polynomial by the second polynomial and
 * stores the result in the provided result polynomial, applying the specified
 * modulus to each coefficient.
 *
 * @param poly1 Pointer to the first polynomial.
 * @param poly2 Pointer to the second polynomial.
 * @param result Pointer to the polynomial structure to store the result.
 * @param mod The modulus to apply to each coefficient.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int poly_lshift(const Polynomial *poly1, const Polynomial *poly2,
                Polynomial *result, uint64_t mod);

/**
 * @brief Compute the modulus of a polynomial.
 *
 * This function computes the modulus of the first polynomial by repeatedly left
 * shifting it by the second polynomial until the degree of the first polynomial
 * becomes less than the degree of the second polynomial.
 *
 * @param poly1 Pointer to the first polynomial (will be modified).
 * @param poly2 Pointer to the second polynomial.
 * @param mod The modulus to apply to each coefficient.
 *
 * @return 0 if successful, -1 if an error occurred.
 */
int poly_mod(Polynomial *poly1, const Polynomial *poly2, uint64_t mod);

/**
 * @brief Subtract a scaler from each coefficient of a polynomial and store the
 * result.
 *
 * This function subtracts a scaler from each coefficient of the given
 * polynomial and stores the result in the provided result polynomial, applying
 * the specified modulus.
 *
 * @param poly Pointer to the polynomial structure.
 * @param scaler The scaler value to subtract from each coefficient.
 * @param result Pointer to the polynomial structure to store the result.
 * @param mod The modulus to apply to each coefficient.
 *
 * @return 0 if successful.
 */
int poly_sub_scaler(const Polynomial *poly, uint64_t scaler, Polynomial *result,
                    uint64_t mod);

/**
 * @brief Add a scaler to each coefficient of a polynomial and store the result.
 *
 * This function adds a scaler to each coefficient of the given polynomial
 * and stores the result in the provided result polynomial, applying the
 * specified modulus.
 *
 * @param poly Pointer to the polynomial structure.
 * @param scaler The scaler value to add to each coefficient.
 * @param result Pointer to the polynomial structure to store the result.
 * @param mod The modulus to apply to each coefficient.
 *
 * @return 0 if successful.
 */
int poly_add_scaler(const Polynomial *poly, uint64_t scaler, Polynomial *result,
                    uint64_t mod);

#ifdef __cplusplus
}
#endif

#endif
