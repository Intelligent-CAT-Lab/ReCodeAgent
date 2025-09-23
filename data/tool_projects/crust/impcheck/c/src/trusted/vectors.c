
#include "trusted_utils.h"  // for u64, u8

#define TYPE u8
#define TYPED(THING) u8_ ## THING
#include "vec.c"
#undef TYPED
#undef TYPE

#define TYPE signed char
#define TYPED(THING) i8_ ## THING
#include "vec.c"
#undef TYPED
#undef TYPE

#define TYPE int
#define TYPED(THING) int_ ## THING
#include "vec.c"
#undef TYPED
#undef TYPE

#define TYPE u64
#define TYPED(THING) u64_ ## THING
#include "vec.c"
#undef TYPED
#undef TYPE
