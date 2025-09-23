#ifndef ACES_INTERNAL_H
#define ACES_INTERNAL_H

#ifdef __cplusplus
extern "C" {
#endif

#include "Channel.h"
#include "Matrix.h"
#include "Polynomial.h"

#include <stdint.h>

#define EPROB 0.5

/**
 * @brief Generate an error element 'rm' over Zq[X]u.
 *
 * This function generates an error element 'rm' over the ring Zq[X]u,
 * such that its evaluation in Zq at the integer ω equals 'message'.
 *
 * @param q The modulus.
 * @param message The message to encode in the error.
 * @param rm Pointer to the polynomial structure to store the generated error
 * element.
 *
 * @return 0 if successful.
 */
int generate_error(uint64_t q, uint64_t message, Polynomial *rm);

/**
 * @brief Generate a vanisher vector 'e' over Zq[X]u.
 *
 * This function generates a vanisher vector 'e' of size N over the ring Zq[X]u,
 * such that the evaluation of each coefficient e_i' in Zq at the integer ω is
 * equal to a product pℓ, where ℓ ∈ {0,1}.
 *
 * @param p The product factor.
 * @param q The modulus.
 * @param e Pointer to the polynomial structure to store the generated vanisher
 * vector.
 *
 * @return 0 if successful.
 */
uint64_t generate_vanisher(uint64_t p, uint64_t q, Polynomial *e);

/**
 * @brief Generate a linear vector 'b' over Zq[X]u.
 *
 * This function generates a linear vector 'b' of size N over the ring Zq[X]u,
 * where each coefficient b_i(ω) is in the range [0, p].
 *
 * @param p The upper bound for coefficients.
 * @param q The modulus.
 * @param b Pointer to the polynomial structure to store the generated linear
 * vector.
 *
 * @return 0 if successful.
 */
uint64_t generate_linear(uint64_t p, uint64_t q, Polynomial *b);

/**
 * @brief Generate the polynomial 'u' for the arithmetic channel.
 *
 * This function generates the polynomial 'u' for the arithmetic channel using
 * the provided parameters.
 *
 * @param channel Pointer to the arithmetic channel.
 * @param param Pointer to the parameters structure.
 * @param u Pointer to the polynomial structure to store the generated
 * polynomial 'u'.
 *
 * @return 0 if successful.
 */
int generate_u(const Channel *channel, const Parameters *param, Polynomial *u);

/**
 * @brief Generate the secret key for the arithmetic channel.
 *
 * This function generates the secret key for the arithmetic channel using the
 * provided parameters, channel information, and intermediate values.
 *
 * @param channel Pointer to the arithmetic channel.
 * @param param Pointer to the parameters structure.
 * @param u Pointer to the polynomial 'u'.
 * @param secret Pointer to the polynomial array 'secret' to store the generated
 * secret key.
 * @param lambda Pointer to the 3D matrix 'lambda' Represents the set of
 * 3-tensors lambda in Zq for the arithmetic channel. This structure represents
 * the set of 3-tensors lambda in Zq for the arithmetic channel. Each element
 * λ[k][i][j] in the 3D matrix corresponds to the coefficients of the generated
 * secret key. The set H(x|C) of 3-tensors λ is defined such that for every pair
 * (i, j) of elements in [n], the equation holds in the ring Zq[X]u.
 *
 * @return 0 if successful.
 */
int generate_secret(const Channel *channel, const Parameters *param,
                    const Polynomial *u, PolyArray *secret, Matrix3D *lambda);

/**
 * @brief Generate the polynomial array 'f0' for the arithmetic channel public
 * key.
 *
 * This function generates the polynomial array 'f0' for the arithmetic channel
 * public key using the provided parameters and channel information.
 *
 * @param channel Pointer to the arithmetic channel.
 * @param param Pointer to the parameters structure.
 * @param f0 Pointer to the polynomial array structure to store the generated
 * polynomials.
 *
 * @return 0 if successful.
 */
int generate_f0(const Channel *channel, const Parameters *param, PolyArray *f0);

/**
 * @brief Generate the polynomial 'f1' for the arithmetic channel public key.
 *
 * This function generates the polynomial 'f1' for the arithmetic channel public
 * key using the provided parameters, channel information, and intermediate
 * values.
 *
 * @param channel Pointer to the arithmetic channel.
 * @param param Pointer to the parameters structure.
 * @param f0 Pointer to the polynomial array 'f0'.
 * @param x Pointer to the polynomial array secret.
 * @param u Pointer to the polynomial 'u'.
 * @param f1 Pointer to the polynomial structure to store the generated
 * polynomial 'f1'.
 *
 * @return 0 if successful.
 */
int generate_f1(const Channel *channel, const Parameters *param,
                const PolyArray *f0, const PolyArray *x, const Polynomial *u,
                Polynomial *f1);

#ifdef __cplusplus
}
#endif

#endif
