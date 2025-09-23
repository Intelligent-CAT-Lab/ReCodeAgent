#ifndef SRC_S21_MATH_H_
#define SRC_S21_MATH_H_

#include <limits.h>

#define S21_M_PI 3.14159265358979323846264338327950288
#define S21_M_E 2.71828182845904523536028747135266250
#define EPS_10 1e-10
#define EPS_6 1e-06

#define S21_NAN 0.0 / 0.0
#define S21_INFINITY 1.0 / 0.0

int castom_abs(int x);
long double castom_acos(double x);
long double castom_asin(double x);
long double castom_atan(double x);
long double castom_ceil(double x);
long double castom_cos(double x);
long double castom_exp(double x);
long double castom_fabs(double x);
long double castom_floor(double x);
long double castom_fmod(double x, double y);
long double castom_log(double x);
long double castom_pow(double base, double exp);
long double castom_sin(double x);
long double castom_sqrt(double x);
long double castom_tan(double x);
long double castom_trunc(double x);
unsigned long long castom_factorial(unsigned long long n);

#endif  // SRC_S21_MATH_H_
