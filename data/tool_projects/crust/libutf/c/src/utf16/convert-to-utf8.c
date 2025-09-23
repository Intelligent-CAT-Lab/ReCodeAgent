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
utf16le_convert_to_utf8 (const utf16_t *data, size_t len, utf8_t *result) {
  size_t pos = 0;
  uint16_t word, diff;
  utf8_t *start = result;

  while (pos < len) {
    if (pos + 4 <= len) {
      uint64_t v;
      memcpy(&v, data + pos, sizeof(uint64_t));
      if (utf_is_be()) v = (v >> 8) | (v << (64 - 8));
      if ((v & 0xff80ff80ff80ff80) == 0) {
        size_t final_pos = pos + 4;
        while (pos < final_pos) {
          *result++ = utf_is_be() ? utf_swap_uint16(data[pos]) : data[pos];
          pos++;
        }
        continue;
      }
    }
    word = utf_is_be() ? utf_swap_uint16(data[pos]) : data[pos];
    if ((word & 0xff80) == 0) {
      *result++ = word;
      pos++;
    } else if ((word & 0xf800) == 0) {
      *result++ = (word >> 6) | 0b11000000;
      *result++ = (word & 0b111111) | 0b10000000;
      pos++;
    } else if ((word & 0xf800) != 0xd800) {
      *result++ = (word >> 12) | 0b11100000;
      *result++ = ((word >> 6) & 0b111111) | 0b10000000;
      *result++ = (word & 0b111111) | 0b10000000;
      pos++;
    } else {
      diff = word - 0xd800;
      if (pos + 1 >= len) {
        return 0;
      }
      word = utf_is_be() ? utf_swap_uint16(data[pos + 1]) : data[pos + 1];
      uint32_t value = (diff << 10) + (word - 0xdc00) + 0x10000;
      *result++ = (value >> 18) | 0b11110000;
      *result++ = ((value >> 12) & 0b111111) | 0b10000000;
      *result++ = ((value >> 6) & 0b111111) | 0b10000000;
      *result++ = (value & 0b111111) | 0b10000000;
      pos += 2;
    }
  }

  return result - start;
}
