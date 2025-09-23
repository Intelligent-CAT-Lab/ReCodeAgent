#include <stdbool.h>
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

bool
ascii_validate (const ascii_t *data, size_t len) {
  uint64_t pos = 0;

  for (; pos + 16 <= len; pos += 16) {
    uint64_t v1;
    memcpy(&v1, data + pos, sizeof(uint64_t));
    uint64_t v2;
    memcpy(&v2, data + pos + sizeof(uint64_t), sizeof(uint64_t));
    uint64_t v = v1 | v2;
    if ((v & 0x8080808080808080) != 0) {
      return false;
    }
  }
  for (; pos < len; pos++) {
    if (data[pos] >= 0b10000000) {
      return false;
    }
  }

  return true;
}
