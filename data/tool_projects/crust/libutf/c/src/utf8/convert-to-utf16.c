#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include "../../include/utf.h"
#include "../endianness.h"

/**
 * Modified from https://github.com/simdutf/simdutf
 *
 * Copyright 2020 The simdutf authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

size_t
utf8_convert_to_utf16le (const utf8_t *data, size_t len, utf16_t *result) {
  size_t pos = 0;
  utf16_t *start = result;

  while (pos < len) {
    if (pos + 8 <= len) {
      uint64_t v;
      memcpy(&v, data + pos, sizeof(uint64_t));
      if ((v & 0x8080808080808080) == 0) {
        size_t final_pos = pos + 8;
        while (pos < final_pos) {
          *result++ = utf_is_be() ? utf_swap_uint16(data[pos]) : data[pos];
          pos++;
        }
        continue;
      }
    }
    uint8_t leading_byte = data[pos];
    if (leading_byte < 0b10000000) {
      *result++ = utf_is_be() ? utf_swap_uint16(leading_byte) : leading_byte;
      pos++;
    } else if ((leading_byte & 0b11100000) == 0b11000000) {
      if (pos + 1 >= len) {
        break;
      }
      uint16_t code_point = ((leading_byte & 0b00011111) << 6) |
                            (data[pos + 1] & 0b00111111);
      if (utf_is_be()) {
        code_point = utf_swap_uint16(code_point);
      }
      *result++ = code_point;
      pos += 2;
    } else if ((leading_byte & 0b11110000) == 0b11100000) {
      if (pos + 2 >= len) {
        break;
      }
      uint16_t code_point = ((leading_byte & 0b00001111) << 12) |
                            ((data[pos + 1] & 0b00111111) << 6) |
                            (data[pos + 2] & 0b00111111);
      if (utf_is_be()) {
        code_point = utf_swap_uint16(code_point);
      }
      *result++ = code_point;
      pos += 3;
    } else if ((leading_byte & 0b11111000) == 0b11110000) {
      if (pos + 3 >= len) {
        break;
      }
      uint32_t code_point = ((leading_byte & 0b00000111) << 18) |
                            ((data[pos + 1] & 0b00111111) << 12) |
                            ((data[pos + 2] & 0b00111111) << 6) |
                            (data[pos + 3] & 0b00111111);
      code_point -= 0x10000;
      uint16_t high_surrogate = 0xd800 + (code_point >> 10);
      uint16_t low_surrogate = 0xdc00 + (code_point & 0x3ff);
      if (utf_is_be()) {
        high_surrogate = utf_swap_uint16(high_surrogate);
        low_surrogate = utf_swap_uint16(low_surrogate);
      }
      *result++ = high_surrogate;
      *result++ = low_surrogate;
      pos += 4;
    } else {
      return 0;
    }
  }

  return result - start;
}
