/* suite6.h - libpgn test suite
 *
 * Copyright (c) 2024, youkwhd <lolywk@tutanota.com>
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#ifndef SUITE6_H
#define SUITE6_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

/* ANSI color codes */
#define R "\033[31m"
#define G "\033[32m"
#define B "\033[34;4m"
#define C "\033[0m"

#define suite6_pass(expr)                                                         \
    do {                                                                          \
        printf("%s[PASS] %s%s:%d%s :: %s\n", G, B, __FILE__, __LINE__, C, #expr); \
    } while (0)

#define suite6_fail(expr)                                                         \
    do {                                                                          \
        printf("%s[FAIL] %s%s:%d%s :: %s\n", R, B, __FILE__, __LINE__, C, #expr); \
        abort();                                                                  \
    } while (0)

#define suite6_assert(expr)              \
    if ((bool)(expr)) suite6_pass(expr); \
    else suite6_fail(expr)

#endif // SUITE6_H
