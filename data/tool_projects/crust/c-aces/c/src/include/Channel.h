#ifndef CHANNEL_H
#define CHANNEL_H

#include <stdint.h>

typedef struct {
  uint64_t dim;
  uint64_t N;
} Parameters;

/**
 * @brief Represents an arithmetic channel.
 *
 * An arithmetic channel consists of a tuple (p, q, ω, u) where:
 * 1) p, q, and ω are positive integers such that p < q;
 * 2) u is a polynomial in Z[X] such that u(ω) = q.
 *
 * @param p The positive integer 'p'.
 * @param q The positive integer 'q'.
 * @param w The positive integer 'ω'.
 */
typedef struct {
  uint64_t p;
  uint64_t q;
  uint64_t w;
} Channel;

/**
 * @brief Initialize an arithmetic channel.
 *
 * This function initializes an arithmetic channel with the provided parameters.
 *
 * @param channel Pointer to the arithmetic channel structure to be initialized.
 * @param p The positive integer 'p'.
 * @param q The positive integer 'q'.
 * @param w The positive integer 'ω'.
 *
 * @return 0 if successful.
 */
int init_channel(Channel *, uint64_t p, uint64_t q, uint64_t w);

#ifdef __cplusplus
}
#endif

#endif
