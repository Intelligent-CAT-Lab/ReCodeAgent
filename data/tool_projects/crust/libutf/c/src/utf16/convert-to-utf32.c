#include <stddef.h>
#include <stdint.h>

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
utf16le_convert_to_utf32 (const utf16_t *data, size_t len, utf32_t *result) {
  size_t pos = 0;
  uint16_t word, diff;
  utf32_t *start = result;

  while (pos < len) {
    word = utf_is_be() ? utf_swap_uint16(data[pos]) : data[pos];
    if ((word & 0xf800) != 0xd800) {
      *result++ = word;
      pos++;
    } else {
      diff = word - 0xd800;
      if (diff > 0x3ff) {
        return 0;
      }
      if (pos + 1 >= len) {
        return 0;
      }
      word = utf_is_be() ? utf_swap_uint16(data[pos + 1]) : data[pos + 1];
      uint32_t value = (diff << 10) + (word - 0xdc00) + 0x10000;
      diff = word - 0xdc00;
      if (diff > 0x3ff) {
        return 0;
      }
      *result++ = value;
      pos += 2;
    }
  }

  return result - start;
}
