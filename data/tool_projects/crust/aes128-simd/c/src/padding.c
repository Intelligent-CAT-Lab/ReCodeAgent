#include "aes.h"

void PadBuffer(uint8_t *input, size_t inputLen, uint8_t **output, size_t *outputLen) {
    size_t blockSize = 16;
    size_t paddedLen = ((inputLen / blockSize) + 1) * blockSize;
    uint8_t *paddedData = _mm_malloc(paddedLen, 16);
    memcpy(paddedData, input, inputLen);
    uint8_t padValue = paddedLen - inputLen;
    memset(paddedData + inputLen, padValue, padValue);
    *output = paddedData;
    *outputLen = paddedLen;
}

size_t RemovePadding(uint8_t *input, size_t inputLen) {
    if (inputLen == 0) return 0;
    uint8_t padValue = input[inputLen - 1];
    if (padValue < 1 || padValue > 16) {
        return inputLen;
    }
    for (size_t i = inputLen - padValue; i < inputLen; ++i) {
        if (input[i] != padValue) {
            return inputLen;
        }
    }
    return inputLen - padValue;
}
