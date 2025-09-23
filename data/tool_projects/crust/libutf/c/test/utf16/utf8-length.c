#include <assert.h>

#include "../../include/utf.h"

#define test_length(string, len, expected) \
  { \
    assert(utf8_length_from_utf16le((utf16_t *) string, len) == expected); \
  }

int
main () {
  // ASCII character 'A'
  test_length("\x41\x00", 1, 1);

  // Greek capital letter 'Ω'
  test_length("\xa9\x03", 1, 2);

  // Snowman symbol '☃'
  test_length("\x03\x26", 1, 3);

  // Chinese character '中'
  test_length("\x2d\x4e", 1, 3);

  // Face with tears of joy emoji '😂'
  test_length("\x3d\xd8\x02\xde", 2, 4);

  // Multiple characters 'AΩ☃中😂'
  test_length("\x41\x00\xa9\x03\x03\x26\x2d\x4e\x3d\xd8\x02\xde", 6, 13);
}
