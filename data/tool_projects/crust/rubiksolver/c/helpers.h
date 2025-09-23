#ifndef HELPERS_H
#define HELPERS_H 1

typedef enum {false, true} bool;

#define IS_ODD(x)       (x % 2 == 1)
#define IS_EVEN(x)      (x % 2 == 0)

#define assert_verbose(x) do { printf("asserting %s at %s:%d\n", #x, __FILE__, __LINE__); assert(x); }while(0)

#ifndef PDEBUG
  #ifdef DEBUG
    #define PDEBUG(fmt_str, args...) do { printf("DEBUG: "); printf(fmt_str, args); } while(0)
  #else
    #define PDEBUG(fmt_str, args...) do {}while(0) /* Debug msg ignored */
  #endif
#endif

#define SWAP_VAL(x,y)   do { x ^= y ^= x ^= y; }while(0)

#define SWAP_ENUM(type, x, y) do { type tmp = x; x = y; y = tmp; } while(0)

#define SWAP_ARRAY_ELEMENTS(a, i1, i2)        \
  do {                                        \
    void *tmp = a[i1];                        \
    a[i1] = a[i2];                            \
    a[i2] = tmp;                              \
  }while(0)

#endif
