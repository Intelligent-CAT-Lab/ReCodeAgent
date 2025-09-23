#ifndef _FFT_H_
#define _FFT_H_

#include <stddef.h>
#define USE_MATH_DEFINES 1
#include <math.h>

struct fft_complex {
  float real;
  float imag;
};

/* replace 'struct fft_complex' with your type */
typedef struct fft_complex fft_complex_t;

#define FFT_COMPLEX_ADD(result, a, b) do {                    \
  (result).real = (a).real + (b).real;                        \
  (result).imag = (a).imag + (b).imag;                        \
} while (0)

#define FFT_COMPLEX_SUB(result, a, b) do {                    \
  (result).real = (a).real - (b).real;                        \
  (result).imag = (a).imag - (b).imag;                        \
} while (0)

#define FFT_COMPLEX_MUL(result, a, b) do {                    \
  (result).real = (a).real * (b).real - (a).imag * (b).imag;  \
  (result).imag = (a).real * (b).imag + (a).imag * (b).real;  \
} while (0)

#define FFT_COMPLEX_SELFMUL(self, z) do {                     \
  float r = (self).real * (z).real - (self).imag * (z).imag;  \
  float i = (self).real * (z).imag + (self).imag * (z).real;  \
  (self).real = r;                                            \
  (self).imag = i;                                            \
} while (0)

#define FFT_COMPLEX_COPY(a, b) do { \
  a = b;                            \
} while (0)

#define FFT_COMPLEX_SWAP(a, b) do { \
  fft_complex_t tmp = a;            \
  a = b;                            \
  b = tmp;                          \
} while (0)

#define FFT_COMPLEX_SETONE(result) do { \
  (result).real = 1.0;                  \
  (result).imag = 0.0;                  \
} while (0)

/* set 'result' to the reciprocal of unit root of equation: z ^ N = 1, or e ^ (-i * 2 * pi / N) */
#define FFT_COMPLEX_UNITROOT_RECIP(result, N) do {  \
  (result).real = cos(2 * M_PI / (N));              \
  (result).imag = -sin(2 * M_PI / (N));             \
} while (0)



void fft(const fft_complex_t *restrict x, fft_complex_t *restrict X, size_t logsize);
void fft_inplace(fft_complex_t *x, size_t logsize);

#endif
