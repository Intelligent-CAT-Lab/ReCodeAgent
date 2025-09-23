#ifndef UTF_ENDIANNESS_H
#define UTF_ENDIANNESS_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stdint.h>

typedef enum {
  utf_le,
  utf_be
} utf_endianness_t;

inline utf_endianness_t
utf_endianness (void) {
  const union {
    uint8_t u8[2];
    uint16_t u16;
  } byte_order = {{1, 0}};

  return byte_order.u16 == 1 ? utf_le : utf_be;
}

inline bool
utf_is_le (void) {
  return utf_endianness() == utf_le;
}

inline bool
utf_is_be (void) {
  return utf_endianness() == utf_be;
}

inline uint16_t
utf_swap_uint16 (uint16_t n) {
  return ((n & 0x00ff) << 8) |
         ((n & 0xff00) >> 8);
}

inline uint32_t
utf_swap_uint32 (uint32_t n) {
  return ((n & 0x000000ff) << 24) |
         ((n & 0x0000ff00) << 8) |
         ((n & 0x00ff0000) >> 8) |
         ((n & 0xff000000) >> 24);
}

#ifdef __cplusplus
}
#endif

#endif // UTF_ENDIANNESS_H
