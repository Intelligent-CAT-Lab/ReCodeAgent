#pragma once

#include <assert.h>
#include <stdio.h> // printf

#define tassert(e) ((void)((e) ? 0 : __assert(#e, __FILE__, __LINE__)))
#define __assert(e, file, line)                                                \
    ((void)printf("%s:%u: failed assertion `%s'\n", file, line, e), 0)
