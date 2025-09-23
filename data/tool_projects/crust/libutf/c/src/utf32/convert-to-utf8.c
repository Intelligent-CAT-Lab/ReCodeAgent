#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include "../../include/utf.h"

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
utf32_convert_to_utf8 (const utf32_t *data, size_t len, utf8_t *result) {
  size_t pos = 0;
  uint32_t word;
  utf8_t *start = result;

  while (pos < len) {
    if (pos + 2 <= len) {
      uint64_t v;
      memcpy(&v, data + pos, sizeof(uint64_t));
      if ((v & 0xffffff80ffffff80) == 0) {
        *result++ = data[pos];
        *result++ = data[pos + 1];
        pos += 2;
        continue;
      }
    }
    word = data[pos];
    if ((word & 0xffffff80) == 0) {
      *result++ = word;
      pos++;
    } else if ((word & 0xfffff800) == 0) {
      *result++ = (word >> 6) | 0b11000000;
      *result++ = (word & 0b111111) | 0b10000000;
      pos++;
    } else if ((word & 0xffff0000) == 0) {
      if (word >= 0xd800 && word <= 0xdfff) {
        return 0;
      }
      *result++ = (word >> 12) | 0b11100000;
      *result++ = ((word >> 6) & 0b111111) | 0b10000000;
      *result++ = (word & 0b111111) | 0b10000000;
      pos++;
    } else {
      if (word > 0x10ffff) {
        return 0;
      }
      *result++ = (word >> 18) | 0b11110000;
      *result++ = ((word >> 12) & 0b111111) | 0b10000000;
      *result++ = ((word >> 6) & 0b111111) | 0b10000000;
      *result++ = (word & 0b111111) | 0b10000000;
      pos++;
    }
  }

  return result - start;
}
