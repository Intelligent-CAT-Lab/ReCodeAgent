#include <stdio.h>

#define TINYFSEQ_IMPLEMENTATION
#include "../tinyfseq.h"

int LLVMFuzzerTestOneInput(const uint8_t *data, const size_t size) {
    TFHeader header = {0};
    TFHeader_read(data, size, &header, NULL);
    return 0;
}
