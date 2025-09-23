#undef NDEBUG
#include <assert.h>
#include <stdio.h>
#include <string.h>

#define TINYFSEQ_IMPLEMENTATION
#include "../tinyfseq.h"

int main(const int argc, const char **argv) {
    (void) argc;
    (void) argv;

    // ensure the library is linked by invoking a function and checking a macro
    assert(strlen(TFError_string(TF_OK)) > 0);
    assert(strlen(TINYFSEQ_VERSION) > 0);

    // print the version
    printf("libtinyfseq version %s\n", TINYFSEQ_VERSION);

    return 0;
}
