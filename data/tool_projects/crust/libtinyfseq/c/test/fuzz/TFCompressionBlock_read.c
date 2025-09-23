#include <stdio.h>

#define TINYFSEQ_IMPLEMENTATION
#include "../tinyfseq.h"

int LLVMFuzzerTestOneInput(const uint8_t *data, const size_t size) {
    TFCompressionBlock block = {0};
    TFCompressionBlock_read(data, size, &block, NULL);
    return 0;
}
