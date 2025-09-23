
#include <assert.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>

void do_assert(bool cond) {
#ifndef NDEBUG
    assert(cond);
#else
    if (!cond) {
        printf("Assertion failed! Compile with DEBUG mode for details.\n");
        abort();
    }
#endif
}
