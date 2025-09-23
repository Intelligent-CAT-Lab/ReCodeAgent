#include "castom_math.h"

int castom_abs(int x) { return x > 0 ? x : -x; }

long double castom_acos(double x) { return S21_M_PI / 2 - castom_asin(x); }

long double castom_asin(double x) {
  long double term = x, sum = S21_NAN;
  if (-1.0 < x && x < 1.0) {
    sum = term;
    x *= x;
    for (int k = 1; castom_fabs(term) > EPS_10; k += 2)
      sum += (term *= x * k / (k + 1)) / (k + 2);
  } else if (x == 1.0) {
    sum = S21_M_PI / 2.0;
  } else if (x == -1.0) {
    sum = -S21_M_PI / 2.0;
  }
  return sum;
}

long double castom_atan(double x) {
  return castom_asin(x / castom_sqrt(1.0 + x * x));
}

long double castom_ceil(double x) {
  if (x >= LLONG_MAX || x <= LLONG_MIN || x != x) {
    return (long double)x;
  }
  long double truncation = (long long)x;
  return truncation + (truncation < x);
}

long double castom_cos(double x) {
  x = castom_fmod(x, 2.0 * S21_M_PI);
  long double t_s = 0, last = 1;
  for (int k = 1; castom_fabs(last) > EPS_10; ++k) {
    t_s += last;
    last *= -x * x / (2.0 * k - 1.0) / (2.0 * k);
  }
  return t_s;
}

long double castom_exp(double x) {
  long double sum = 1.0;
  long double cur = 1.0;
  unsigned int n = 1;

  while (castom_fabs(cur) > EPS_10) {
    sum += (cur *= ((long double)x) / n++);
  }

  return sum;
}

long double castom_fabs(double x) {
  return x > 0.0 ? (long double)x : -(long double)x;
}

long double castom_floor(double x) {
  if (x >= LLONG_MAX || x <= LLONG_MIN || x != x) {
    return (long double)x;
  }
  long double truncation = (long long)x;
  return truncation - (truncation > x);
}

long double castom_fmod(double x, double y) {
  return (long double)(x - castom_trunc(x / y) * y);
}

long double castom_log(double x) {
  unsigned a = 0;
  double b = 0;
  if (x > 0) {
    unsigned d;
    double e = 0, c = 0, f = 0;
    for (c = ((x < 1.0) ? 1 / x : x); (c /= S21_M_E) > 1; ++a) {
    }
    c = 1 / (c * S21_M_E - 1), c = c + c + 1, f = c * c, b = 0;
    for (d = 1, c /= 2; e = b, b += 1 / (d * c), b - e;) d += 2, c *= f;
  } else {
    b = (x == 0) / 0.;
  }
  return x < 1 ? -(a + b) : a + b;
}

long double castom_pow(double base, double exp) {
  long double result;
  if (base == 0.0 && exp != 0.0) {
    result = 0.0;
  } else if (exp < 0) {
    result = castom_pow(1 / base, -exp);
  } else if (base < 0.0 && exp != (long)exp) {
    result = -S21_NAN;
  } else if (exp == 0.0) {
    result = 1.0;
  } else {
    long double flag = 1.0;
    if (base < 0 && (long)exp % 2) {
      flag = -1.0;
    }
    if (base < 0) base = -base;
    long div = (long)exp;
    long double integerPart = 1;
    for (int i = 0; i < div; i++) integerPart *= base;
    result =
        integerPart * castom_exp((exp - (double)div) * castom_log(base)) * flag;
  }
  return result;
}

long double castom_sin(double x) {
  x = castom_fmod(x, 2.0 * S21_M_PI);
  long double sum = 0.0;
  for (int i = 0; i <= 20; i++) {
    double fa = 1.0, pow = 1.0;
    for (int j = 1; j <= 2 * i + 1; j++) {
      fa *= j;
      pow *= x;
    }
    sum += ((i % 2 ? -1.0 : 1.0) / fa) * pow;
  }
  return sum;
}

long double castom_sqrt(double x) {
  if (x != x || x < 0) return S21_NAN;
  double sqrt, temp;
  sqrt = x / 2;
  temp = 0;
  while (sqrt != temp) {
    temp = sqrt;
    sqrt = (x / temp + temp) / 2;
  }
  return sqrt;
}

long double castom_tan(double x) {
  long double result = 0.0;
  if (x == 0.0) {
    result = 0.0;
  } else if (S21_M_PI / 6 == x) {
    result = 1 / castom_sqrt(3);
  } else if (S21_M_PI / 4 == x) {
    result = 1.0;
  } else if (S21_M_PI / 3 == x) {
    result = castom_sqrt(3);
  } else if (S21_M_PI / 2 == x) {
    result = S21_INFINITY;
  } else if (S21_M_PI == x) {
    result = 0.0;
  } else if (3.0 * S21_M_PI / 2 == x) {
    result = S21_INFINITY;
  } else if (2.0 * S21_M_PI == x) {
    result = 0.0;
  } else {
    result = castom_sin(x) / castom_cos(x);
  }
  return result;
}

long double castom_trunc(double x) {
  return (x > 0.0) ? castom_floor(x) : castom_ceil(x);
}
