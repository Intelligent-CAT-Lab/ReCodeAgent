
#include "trusted_utils.h"

#ifndef TYPE
#define TYPE int
#endif
#ifndef TYPED
#define TYPED(THING) int_ ## THING
#endif

struct TYPED(vec) {
    u64 capacity;
    u64 size;
    TYPE* data;
};

struct TYPED(vec)* TYPED(vec_init)(u64 capacity);
void TYPED(vec_free)(struct TYPED(vec)* vec);
void TYPED(vec_reserve)(struct TYPED(vec)* vec, u64 new_size);
void TYPED(vec_push)(struct TYPED(vec)* vec, TYPE elem);
void TYPED(vec_clear)(struct TYPED(vec)* vec);
