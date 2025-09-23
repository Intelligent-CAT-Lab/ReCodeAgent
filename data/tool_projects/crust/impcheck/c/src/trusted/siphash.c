
#include "siphash.h"
#include "trusted_utils.h"
#include <stdbool.h>  // for true
#include <stdlib.h>   // for free, abort
#include <assert.h>   // for assert

#define cROUNDS 2
#define dROUNDS 4

#define SH_UINT64_C(c) c##UL

#define ROTL(x, b) (u64)(((x) << (b)) | ((x) >> (64 - (b))))

#define U32TO8_LE(p, v)                                                        \
    (p)[0] = (u8)((v));                                                   \
    (p)[1] = (u8)((v) >> 8);                                              \
    (p)[2] = (u8)((v) >> 16);                                             \
    (p)[3] = (u8)((v) >> 24);

#define U64TO8_LE(p, v)                                                        \
    U32TO8_LE((p), (unsigned int)((v)));                                           \
    U32TO8_LE((p) + 4, (unsigned int)((v) >> 32));

#define U8TO64_LE(p)                                                           \
    (((u64)((p)[0])) | ((u64)((p)[1]) << 8) |                        \
     ((u64)((p)[2]) << 16) | ((u64)((p)[3]) << 24) |                 \
     ((u64)((p)[4]) << 32) | ((u64)((p)[5]) << 40) |                 \
     ((u64)((p)[6]) << 48) | ((u64)((p)[7]) << 56))

#define SIPROUND                                                               \
    do {                                                                       \
        v0 += v1;                                                              \
        v1 = ROTL(v1, 13);                                                     \
        v1 ^= v0;                                                              \
        v0 = ROTL(v0, 32);                                                     \
        v2 += v3;                                                              \
        v3 = ROTL(v3, 16);                                                     \
        v3 ^= v2;                                                              \
        v0 += v3;                                                              \
        v3 = ROTL(v3, 21);                                                     \
        v3 ^= v0;                                                              \
        v2 += v1;                                                              \
        v1 = ROTL(v1, 17);                                                     \
        v1 ^= v2;                                                              \
        v2 = ROTL(v2, 32);                                                     \
    } while (0)

const unsigned char* kk;
u8* out;
const int outlen = 128 / 8;
u64 v0, v1, v2, v3;
u64 k0, k1;
u64 m;
int i;
u64 inlen;

u8* buf;
unsigned char buflen = 0;

void process_next_block() {
    m = U8TO64_LE(buf);
    v3 ^= m;
    for (i = 0; i < cROUNDS; ++i)
        SIPROUND;
    v0 ^= m;
}

void process_final_block() {
    const int left = inlen & 7;
    assert(left == buflen);
    u64 b = ((u64)inlen) << 56;
    u8* ni = buf;

    switch (left) {
    case 7:
        b |= ((u64)ni[6]) << 48;
        /* FALLTHRU */
    case 6:
        b |= ((u64)ni[5]) << 40;
        /* FALLTHRU */
    case 5:
        b |= ((u64)ni[4]) << 32;
        /* FALLTHRU */
    case 4:
        b |= ((u64)ni[3]) << 24;
        /* FALLTHRU */
    case 3:
        b |= ((u64)ni[2]) << 16;
        /* FALLTHRU */
    case 2:
        b |= ((u64)ni[1]) << 8;
        /* FALLTHRU */
    case 1:
        b |= ((u64)ni[0]);
        break;
    case 0:
        break;
    }

    v3 ^= b;

    for (i = 0; i < cROUNDS; ++i)
        SIPROUND;

    v0 ^= b;

    if (outlen == 16)
        v2 ^= 0xee;
    else
        v2 ^= 0xff;

    for (i = 0; i < dROUNDS; ++i)
        SIPROUND;

    b = v0 ^ v1 ^ v2 ^ v3;
    U64TO8_LE(out, b);

    v1 ^= 0xdd;

    for (i = 0; i < dROUNDS; ++i)
        SIPROUND;

    b = v0 ^ v1 ^ v2 ^ v3;
    U64TO8_LE(out + 8, b);
}

void siphash_init(const unsigned char* key_128bit) {
    kk = key_128bit;
    out = trusted_utils_malloc(128 / 8);
    buf = trusted_utils_malloc(8);
    if (kk) siphash_reset();
}
void siphash_reset() {
    v0 = SH_UINT64_C(0x736f6d6570736575);
    v1 = SH_UINT64_C(0x646f72616e646f6d);
    v2 = SH_UINT64_C(0x6c7967656e657261);
    v3 = SH_UINT64_C(0x7465646279746573);
    k0 = U8TO64_LE(kk);
    k1 = U8TO64_LE(kk + 8);
    v3 ^= k1;
    v2 ^= k0;
    v1 ^= k1;
    v0 ^= k0;
    inlen = 0;
    buflen = 0;
    if (outlen == 16)
        v1 ^= 0xee;
}
void siphash_update(const unsigned char* data, u64 nb_bytes) {
    u32 datapos = 0;
    while (true) {
        while (buflen < 8u && datapos < nb_bytes) {
            buf[buflen++] = data[datapos++];
        }
        if (buflen < 8u) {
            break;
        }
        process_next_block();
        buflen = 0;
    }
    inlen += nb_bytes;
}
void siphash_pad(u64 nb_bytes) {
    const unsigned char c = 0;
    for (u64 i = 0; i < nb_bytes; i++) siphash_update(&c, 1);
}
u8* siphash_digest() {
    process_final_block();
    return out;
}
void siphash_free() {
    free(buf);
    free(out);
}

#undef SH_UINT64_C
