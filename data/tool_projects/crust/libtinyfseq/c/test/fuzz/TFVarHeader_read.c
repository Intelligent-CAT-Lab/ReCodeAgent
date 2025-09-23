#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define TINYFSEQ_IMPLEMENTATION
#include "../tinyfseq.h"

int LLVMFuzzerTestOneInput(const uint8_t *data, const size_t size) {
    // attempt to read an initial header value
    TFVarHeader header = {0};
    if (TFVarHeader_read(data, size, &header, NULL, 0, NULL) != TF_OK) return 0;

    if (header.size == 0) return 0;

    // allocate a buffer for the variable data
    // manually subtract 4 to account for the header base size
    const uint16_t vdsize = header.size - 4;
    uint8_t *vd           = malloc(vdsize);
    assert(vd != NULL);

    TFVarHeader_read(data, size, &header, vd, vdsize, NULL);

    free(vd);

    return 0;
}
