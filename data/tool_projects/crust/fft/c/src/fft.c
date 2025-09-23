#include "fft.h"

#include <assert.h>
#include <limits.h>
#include <math.h>

#define INTBITS(t)  (sizeof (t) * CHAR_BIT)


#if defined (__GNUC__) || defined (__clang__)
#define fft_clz(n)                                \
  _Generic(n,                                     \
        unsigned int: __builtin_clz(n),           \
        unsigned long: __builtin_clzl(n),         \
        unsigned long long: __builtin_clzll(n))

#define likely(expr)    __builtin_expect(!!(expr), 1)
#define unlikely(expr)  __builtin_expect(!!(expr), 0)
#else

static inline size_t fft_clz(size_t n) {
  size_t count = 1;
  size_t half = INTBITS(size_t) / 2;
  do {
    while (!(n >> half))
      half >>= 1;
    n >>= half;
    count += half;
  } while (half >>= 1);
  return INTBITS(size_t) - count;
}


#define likely(expr)    (expr)
#define unlikely(expr)  (expr)
#endif

static inline size_t next_reversed_n(size_t reversed_n, size_t shift) {
  reversed_n <<= shift;
  size_t count_leading_ones = fft_clz(~reversed_n);
  reversed_n <<= count_leading_ones;  /* remove leading ones */
  reversed_n |= (size_t)1 << (INTBITS(size_t) - 1);
  reversed_n >>= (shift + count_leading_ones);
  return reversed_n;
}

static inline void rader(const fft_complex_t *restrict array, fft_complex_t *restrict target, size_t logsize) {
  size_t size = (size_t)1 << logsize;
  /* how many bits should be shift to move the number to the most significant bit */
  size_t shift = INTBITS(size_t) - logsize;
  for (size_t n = 0, reversed_n = 0; n < size; ++n) {
    FFT_COMPLEX_COPY(target[reversed_n], array[n]);

    /* get next reversed_n */
    reversed_n = next_reversed_n(reversed_n, shift);
  }
}

static inline void rader_inplace(fft_complex_t *array, size_t logsize) {
  size_t size = (size_t)1 << logsize;
  size_t shift = INTBITS(size_t) - logsize;
  /* nothing should be done for 0 and 0b111...11(size - 1). */
  for (size_t n = 1, reversed_n = size >> 1; n < size - 1; ++n) {
    if (n < reversed_n)
      FFT_COMPLEX_SWAP(array[n], array[reversed_n]);

    /* get next reversed_n */
    reversed_n = next_reversed_n(reversed_n, shift);
  }
}

#define DO_BUTTERFLY(begin, end, step) do {                       \
  fft_complex_t unit;                                             \
  FFT_COMPLEX_UNITROOT_RECIP(unit, step);                         \
  const size_t half = (step) / 2;                                 \
  for (fft_complex_t *p = (begin); p != (end); p += (step)) {     \
    /* i == 0, j == half */                                       \
    fft_complex_t t, u;                                           \
    FFT_COMPLEX_COPY(t, p[half]);                                 \
    FFT_COMPLEX_COPY(u, p[0]);                                    \
    FFT_COMPLEX_ADD(p[0], u, t);                                  \
    FFT_COMPLEX_SUB(p[half], u, t);                               \
    if (half <= 1) continue;                                      \
    /* i == 1, j == half + 1 */                                   \
    fft_complex_t root;                                           \
    FFT_COMPLEX_COPY(root, unit);                                 \
    FFT_COMPLEX_MUL(t, root, p[half + 1]);                        \
    FFT_COMPLEX_COPY(u, p[1]);                                    \
    FFT_COMPLEX_ADD(p[1], u, t);                                  \
    FFT_COMPLEX_SUB(p[half + 1], u, t);                           \
    for (size_t i = 2, j = half + 2; i < half; ++i, ++j) {        \
      FFT_COMPLEX_SELFMUL(root, unit);                            \
      fft_complex_t t, u;                                         \
      FFT_COMPLEX_MUL(t, root, p[j]);                             \
      FFT_COMPLEX_COPY(u, p[i]);                                  \
      FFT_COMPLEX_ADD(p[i], u, t);                                \
      FFT_COMPLEX_SUB(p[j], u, t);                                \
    }                                                             \
  }                                                               \
} while (0)

static void fft_raw(fft_complex_t *x, size_t logsize) {
  if (unlikely(logsize == 0))
    return;

  fft_complex_t *begin = x;
  fft_complex_t *end = begin + ((size_t)1 << logsize);

  DO_BUTTERFLY(begin, end, 2);

  if (unlikely(logsize == 1)) /* size == 2 ? */
    return;

  DO_BUTTERFLY(begin, end, 4);

  if (unlikely(logsize == 2)) /* size == 4 ? */
    return;

  /* do generic butterfly in a loop */
  for (size_t step = 8; step <= (size_t)1 << logsize; step *= 2)
    DO_BUTTERFLY(begin, end, step);

}

void fft(const fft_complex_t *restrict x, fft_complex_t *restrict X, size_t logsize) {
  rader(x, X, logsize);
  fft_raw(X, logsize);
}

void fft_inplace(fft_complex_t *x, size_t logsize) {
  rader_inplace(x, logsize);
  fft_raw(x, logsize);
}
