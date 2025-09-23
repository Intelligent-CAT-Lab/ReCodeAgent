#include <stdbool.h>
#include <stdint.h>

#include "endianness.h"

extern utf_endianness_t
utf_endianness (void);

extern bool
utf_is_le (void);

extern bool
utf_is_be (void);

extern uint16_t
utf_swap_uint16 (uint16_t n);

extern uint32_t
utf_swap_uint32 (uint32_t n);
