#include <assert.h>

#include "../../include/utf.h"

#define test_length(string, len, expected) \
  { \
    assert(utf16_length_from_utf8((utf8_t *) string, len) == expected); \
  }

int
main () {
  // ASCII character 'A'
  test_length("\x41", 1, 1);

  // Greek capital letter 'Ω'
  test_length("\xce\xa9", 2, 1);

  // Snowman symbol '☃'
  test_length("\xe2\x98\x83", 3, 1);

  // Chinese character '中'
  test_length("\xe4\xb8\xad", 3, 1);

  // Face with tears of joy emoji '😂'
  test_length("\xf0\x9f\x98\x82", 4, 2);

  // Multiple characters "AΩ☃中😂"
  test_length("\x41\xce\xa9\xe2\x98\x83\xe4\xb8\xad\xf0\x9f\x98\x82", 13, 6);
}
