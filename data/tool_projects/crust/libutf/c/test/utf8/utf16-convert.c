#include <assert.h>
#include <string.h>

#include "../../include/utf.h"

#define test_convert(string, len, expected, written) \
  { \
    utf16_t result[written]; \
    assert(utf8_convert_to_utf16le((utf8_t *) string, len, result) == written); \
    assert(memcmp(expected, result, written) == 0); \
  }

int
main () {
  // ASCII character 'A'
  test_convert("\x41", 1, "\x41\x00", 1);

  // Greek capital letter 'Ω'
  test_convert("\xce\xa9", 2, "\xa9\x03", 1);

  // Snowman symbol '☃'
  test_convert("\xe2\x98\x83", 3, "\x03\x26", 1);

  // Chinese character '中'
  test_convert("\xe4\xb8\xad", 3, "\x2d\x4e", 1);

  // Face with tears of joy emoji '😂'
  test_convert("\xf0\x9f\x98\x82", 4, "\x3d\xd8\x02\xde", 2);

  // Multiple characters "AΩ☃中😂"
  test_convert("\x41\xce\xa9\xe2\x98\x83\xe4\xb8\xad\xf0\x9f\x98\x82", 13, "\x41\x00\xa9\x03\x03\x26\x2d\x4e\x3d\xd8\x02\xde", 6);
}
