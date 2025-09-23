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
utf8_length_from_utf16le (const utf16_t *data, size_t len) {
  size_t counter = 0;
  uint16_t word;

  for (size_t i = 0; i < len; i++) {
    word = utf_is_be() ? utf_swap_uint16(data[i]) : data[i];
    if (word <= 0x7f) {
      counter++;
    } else if (word <= 0x7ff) {
      counter += 2;
    } else if ((word <= 0xd7ff) || (word >= 0xe000)) {
      counter += 3;
    } else {
      counter += 2;
    }
  }

  return counter;
}
