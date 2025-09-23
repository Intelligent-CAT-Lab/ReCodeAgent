#ifndef COMMON_H
#define COMMON_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>

#define MAYBE_UNUSED __attribute__((unused))

/**
 * @brief Extended Euclidean Algorithm result.
 *
 * This structure represents the result of the Extended Euclidean Algorithm,
 * which computes the greatest common divisor (GCD) of two numbers along with
 * the coefficients (a and b) such that gcd = ax + by.
 *
 * @param gcd The greatest common divisor of the two numbers.
 * @param a The coefficient of the first number in the equation.
 * @param b The coefficient of the second number in the equation.
 */
typedef struct {
  uint64_t gcd;
  int64_t a;
  int64_t b;
} Xgcd;

/**
 * @brief Represents a pair of unsigned 64-bit integers.
 *
 * This structure encapsulates a pair of unsigned 64-bit integers.
 *
 * @param first The first value in the pair.
 * @param second The second value in the pair.
 */
typedef struct {
  uint64_t first;
  uint64_t second;
} Pair;

/**
 * @brief Compute the greatest common divisor (GCD) of two numbers.
 *
 * This function computes the greatest common divisor (GCD) of two unsigned
 * 64-bit integers.
 *
 * @param x The first number.
 * @param y The second number.
 *
 * @return The greatest common divisor of x and y.
 */
uint64_t gcd(uint64_t x, uint64_t y);

/**
 * @brief Compute the extended greatest common divisor (xgcd) of two numbers.
 *
 * This function computes the extended greatest common divisor (xgcd) of two
 * unsigned 64-bit integers. The xgcd algorithm computes the greatest common
 * divisor (GCD) of two numbers along with the coefficients (a and b) such that
 * gcd = ax + by.
 *
 * @param a The first number.
 * @param b The second number.
 *
 * @return The extended greatest common divisor (xgcd) result.
 */
Xgcd xgcd(uint64_t a, uint64_t b);

/**
 * @brief Check if two numbers are coprime.
 *
 * This function checks if two unsigned 64-bit integers are coprime,
 * meaning their greatest common divisor (GCD) is equal to 1.
 *
 * @param x The first number.
 * @param y The second number.
 *
 * @return 1 if the numbers are coprime, 0 otherwise.
 */
int are_coprime(uint64_t x, uint64_t y);

/**
 * @brief Generate a random pair of inverses.
 *
 * This function generates a random pair of inverses for a given value.
 * It selects a random number 'a' within the range [2, value - 1], ensuring
 * that 'a' is coprime with the given value. Then, it computes the modular
 * inverse of 'a' with respect to the given value and returns a pair consisting
 * of 'a' and its modular inverse.
 *
 * @param value The value for which inverses are generated.
 *
 * @return A pair containing 'a' and its modular inverse.
 */
Pair randinverse(uint64_t value);

/**
 * @brief Generate a random number within a specified range.
 *
 * This function generates a random unsigned 64-bit integer within the specified
 * range [lower, upper].
 *
 * @param lower The lower bound of the range.
 * @param upper The upper bound of the range.
 *
 * @return A random number within the specified range.
 */
uint64_t randrange(uint64_t lower, uint64_t upper);

/**
 * @brief Generate a random number from a normal distribution.
 *
 * This function generates a random number from a normal distribution
 * with the specified mean and standard deviation.
 *
 * @param mean The mean (average) of the normal distribution.
 * @param stddev The standard deviation of the normal distribution.
 *
 * @return A random number from the normal distribution.
 */
double normal_rand(double mean, double stddev);

static inline uint64_t max(uint64_t a, uint64_t b) { return a > b ? a : b; }

static inline uint64_t min(uint64_t a, uint64_t b) { return a < b ? a : b; }

/**
 * @brief Clamp a value within a specified range.
 *
 * This function clamps a value within a specified range defined by the minimum
 * and maximum values. If the given value is less than the minimum value, it
 * returns the minimum value. If the given value is greater than the maximum
 * value, it returns the maximum value. Otherwise, it returns the given value.
 *
 * @param min_value The minimum value of the range.
 * @param max_value The maximum value of the range.
 * @param value The value to clamp.
 *
 * @return The clamped value within the specified range.
 */
static inline uint64_t clamp(uint64_t min_value, uint64_t max_value,
                             uint64_t value) {
  return max(min_value, min(max_value, value));
}

#ifdef __cplusplus
}
#endif

#endif
