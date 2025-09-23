#ifndef ACES_H
#define ACES_H

#ifdef __cplusplus
extern "C" {
#endif

#include "Channel.h"
#include "Matrix.h"
#include "Polynomial.h"

/**
 * @brief Represents the public key for the arithmetic channel.
 *
 * This structure represents the public key for the arithmetic channel,
 * consisting of the polynomial 'u' and 3D matrix 'lambda'.
 *
 * @param u Polynomial representing the parameter 'u'.
 * @param lambda 3D matrix representing intermediate values. Each element
 * λ[k][i][j] in the matrix corresponds to the coefficients of the generated
 * secret key.
 */
typedef struct {
  Polynomial u;
  Matrix3D lambda;
} PublicKey;

/**
 * @brief Represents the private key for the arithmetic channel.
 *
 * This structure represents the private key for the arithmetic channel,
 * consisting of the polynomial array 'x', polynomial array 'f0', and polynomial
 * 'f1'.
 *
 * @param x Polynomial array representing the secret key.
 * @param f0 Polynomial array representing the public key component 'f0'.
 * @param f1 Polynomial representing the public key component 'f1'.
 */
typedef struct {
  PolyArray x;
  PolyArray f0;
  Polynomial f1;
} PrivateKey;

/**
 * @brief Represents the shared information for the arithmetic channel.
 *
 * This structure represents the shared information for the arithmetic channel,
 * including the channel parameters, public key, and other necessary parameters.
 *
 * @param channel The arithmetic channel.
 * @param param Parameters for the arithmetic channel.
 * @param pk Public key for the arithmetic channel.
 */
typedef struct {
  Channel channel;
  Parameters param;
  PublicKey pk;
} SharedInfo;

/**
 * @brief Represents an instance of the ACES (Arithmetic Channel Encryption
 * Scheme).
 *
 * This structure represents an instance of the Arithmetic Channel Encryption
 * Scheme (ACES), including shared information and private key components.
 *
 * @param shared_info Shared information for the ACES instance.
 * @param private_key Private key components for the ACES instance.
 */
typedef struct {
  SharedInfo shared_info;
  PrivateKey private_key;
} Aces;

/**
 * @brief Represents a ciphertext in the ACES framework.
 *
 * The ACES framework encrypts a message m ∈ ℤₚ as a ciphertext (c,c') according
 * to the following procedure:
 *
 * 1. The first component c is an n-vector over ℤ_q[X]_u given by c = f₀ᵀb,
 * where:
 *    - f₀ is an n × N matrix over ℤ_q[X]_u (chosen during key generation).
 *    - b = (bᵢ)ᵢ is an N-vector over ℤ_q[X]_u such that bᵢ(ω) ∈ {0,1,…,p}
 * (selected by the sender).
 *
 * 2. The second component c' is an element of ℤ_q[X] defined as
 * c' = r_m + cᵀx + e, where:
 *    - r_m is an element over ℤ_q[X]_u chosen such that its evaluation in ℤ_q
 * at the integer ω equals m (chosen by the sender).
 *    - x is an n-vector over ℤ_q[X]_u (considered the private key).
 *    - e is a scalar product in the form of bᵀe', where e' = (eᵢ')ᵢ is an
 * N-vector over ℤ_q[X]_u such that the evaluation of eᵢ' in ℤ_q at the integer
 * ω is equal to a product pℓ where ℓ ∈ {0,1}.
 *
 * @param c1 First component of the ciphertext, represented as a polynomial
 * array.
 * @param c2 Second component of the ciphertext, represented as a polynomial.
 * @param level The level parameter associated with the ciphertext.
 */
typedef struct {
  PolyArray c1;
  Polynomial c2;
  uint64_t level;
} CipherMessage;

int set_aces(Aces *aces, size_t dim, void *mem, size_t size);

/**
 * @brief Initializes an instance of the ACES (Arithmetic Channel Encryption
 * Scheme).
 *
 * @param p Parameter p for the ACES channel.
 * @param q Parameter q for the ACES channel.
 * @param dim Dimension of the ACES parameters.
 * @param[out] aces Pointer to the ACES structure to be initialized.
 * @return int Returns 0 upon successful initialization, or an error code
 * otherwise.
 */
int init_aces(uint64_t p, uint64_t q, uint64_t dim, Aces *aces);

/**
 * @brief Encrypts a message using the ACES (Arithmetic Channel Encryption
 * Scheme).
 *
 * @param aces Pointer to the initialized ACES structure.
 * @param message Pointer to the message to be encrypted.
 * @param size Size of the message array.
 * @param[out] result Pointer to the structure where the ciphertext will be
 * stored.
 * @return int Returns 0 upon successful encryption, or an error code otherwise.
 */
int aces_encrypt(const Aces *aces, const uint64_t *message, size_t size,
                 CipherMessage *result);

/**
 * @brief Decrypts a ciphertext using the ACES (Arithmetic Channel Encryption
 * Scheme).
 *
 * @param aces Pointer to the initialized ACES structure.
 * @param message Pointer to the ciphertext to be decrypted.
 * @param size Size of the ciphertext array.
 * @param[out] result Pointer to the array where the decrypted message will be
 * stored.
 * @return int Returns 0 upon successful decryption, or an error code otherwise.
 */
int aces_decrypt(const Aces *aces, const CipherMessage *message, size_t size,
                 uint64_t *result);

/**
 * @brief Performs addition on two ciphertexts encrypted under the ACES scheme.
 *
 * @param a Pointer to the first ciphertext.
 * @param b Pointer to the second ciphertext.
 * @param info Pointer to the shared information including parameters and keys.
 * @param[out] result Pointer to the structure where the result of addition will
 * be stored.
 * @return int Returns 0 upon successful addition, or an error code otherwise.
 */
int aces_add(const CipherMessage *, const CipherMessage *, const SharedInfo *,
             CipherMessage *);

/**
 * @brief Performs multiplication on two ciphertexts encrypted under the ACES
 * scheme.
 *
 * @param a Pointer to the first ciphertext.
 * @param b Pointer to the second ciphertext.
 * @param info Pointer to the shared information including parameters and keys.
 * @param[out] result Pointer to the structure where the result of
 * multiplication will be stored.
 * @return int Returns 0 upon successful multiplication, or an error code
 * otherwise.
 */
int aces_mul(const CipherMessage *, const CipherMessage *, const SharedInfo *,
             CipherMessage *);

/**
 * @brief Refreshes a ciphertext to mitigate level increase resulting from
 * arithmetic operations.
 *
 * ACES is a fully homomorphic encryption scheme that initially relies on a
 * leveled FHE framework. This function is designed to refresh a ciphertext,
 * mitigating the level increase resulting from arithmetic operations.
 *
 * @param info Pointer to the shared information including parameters and keys.
 * @param[in,out] message Pointer to the ciphertext to be refreshed.
 * @param level The desired level of the refreshed ciphertext.
 * @return int Returns 0 upon successful refresh, or an error code otherwise.
 */
int aces_refresh(const SharedInfo *info, CipherMessage *a, uint64_t k);

#ifdef __cplusplus
}
#endif

#endif
