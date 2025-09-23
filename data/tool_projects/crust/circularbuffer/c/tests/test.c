#include "../src/CircularBuffer.h"
#include <string.h>
#include <assert.h>

int main(int argc, char *argv[]) {
    CircularBuffer cb = CircularBufferCreate(8);
    char *a = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    char b[128];
    int len = 0, offset = 0, outLen = 0;

    // push 3 bytes
    len = 3;
    CircularBufferPush(cb, a + offset, len);
    offset += len;
    assert(CircularBufferGetDataSize(cb) == 3);
    
    // push 7 bytes
    len = 7;
    CircularBufferPush(cb, a + offset, len);
    offset += len;
    assert(CircularBufferGetDataSize(cb) == 8);

    // pop 3 bytes
    len = 3;
    memset(b, '\0', 128);
    outLen = (int)CircularBufferPop(cb, len, b);
    assert(outLen == 3 && strncmp(b, "234", len) == 0);
    assert(CircularBufferGetDataSize(cb) == 5);

    // read 2 bytes
    len = 2;
    memset(b, '\0', 128);
    outLen = (int)CircularBufferRead(cb, len, b);
    assert(outLen == 2 && strncmp(b, "56", len) == 0);
    assert(CircularBufferGetDataSize(cb) == 5);

    // push 10 bytes
    len = 10;
    CircularBufferPush(cb, a + offset, len);
    offset += len;
    assert(CircularBufferGetDataSize(cb) == 8);

    // pop 3 bytes
    len = 3;
    memset(b, '\0', 128);
    outLen = (int)CircularBufferPop(cb, len, b);
    assert(outLen == 3 && strncmp(b, "cde", len) == 0);
    assert(CircularBufferGetDataSize(cb) == 5);

    // pop 30 bytes, but only expect 5 available
    len = 30;
    memset(b, '\0', 128);
    outLen = (int)CircularBufferPop(cb, len, b);
    assert(outLen == 5 && strncmp(b, "fghij", outLen) == 0);
    assert(CircularBufferGetDataSize(cb) == 0);

    // push 5 bytes
    len = 5;
    CircularBufferPush(cb, a + offset, len);
    offset += len;
    assert(CircularBufferGetDataSize(cb) == 5);

    // pop 2 bytes
    len = 2;
    memset(b, '\0', 128);
    outLen = (int)CircularBufferPop(cb, len, b);
    // assert(outLen == 2 && strncmp(b, "kl", len) == 0);
    assert(CircularBufferGetDataSize(cb) == 3);

    // push 10 bytes
    len = 10;
    CircularBufferPush(cb, a + offset, len);
    offset += len;
    assert(CircularBufferGetDataSize(cb) == 8);

    // pop 6 bytes
    len = 6;
    memset(b, '\0', 128);
    outLen = (int)CircularBufferPop(cb, len, b);
    assert(outLen == 6 && strncmp(b, "rstuvw", len) == 0);
    assert(CircularBufferGetDataSize(cb) == 2);

    // push 4 bytes
    len = 4;
    CircularBufferPush(cb, a + offset, len);
    offset += len;
    assert(CircularBufferGetDataSize(cb) == 6);
    printf("All tests passed!\n");
    return 0;
}
