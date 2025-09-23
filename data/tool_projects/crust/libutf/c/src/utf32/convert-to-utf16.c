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
utf32_convert_to_utf16le (const utf32_t *data, size_t len, utf16_t *result) {
  size_t pos = 0;
  uint32_t word;
  utf16_t *start = result;

  while (pos < len) {
    word = data[pos];
    if ((word & 0xffff0000) == 0) {
      if (word >= 0xd800 && word <= 0xdfff) {
        return 0;
      }
      *result++ = utf_is_be() ? utf_swap_uint16(data[pos]) : data[pos];
    } else {
      if (word > 0x10ffff) {
        return 0;
      }
      word -= 0x10000;
      uint16_t high_surrogate = 0xd800 + (word >> 10);
      uint16_t low_surrogate = 0xdc00 + (word & 0x3ff);
      if (utf_is_be()) {
        high_surrogate = utf_swap_uint16(high_surrogate);
        low_surrogate = utf_swap_uint16(low_surrogate);
      }
      *result++ = high_surrogate;
      *result++ = low_surrogate;
    }
    pos++;
  }

  return result - start;
}
