#include <stdio.h>

#define TINYFSEQ_IMPLEMENTATION
#include "../tinyfseq.h"

int LLVMFuzzerTestOneInput(const uint8_t *data, const size_t size) {
    TFChannelRange range = {0};
    TFChannelRange_read(data, size, &range, NULL);
    return 0;
}
