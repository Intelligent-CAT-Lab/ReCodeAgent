#include <assert.h>

#include "../../include/utf.h"

#define test_validate(string, len) \
  { \
    assert(utf16le_validate((utf16_t *) string, len)); \
  }

int
main () {
  // Single ASCII character, 'a'
  test_validate("\x61\x00", 1);

  // String of ASCII characters, "Hello"
  test_validate("\x48\x00\x65\x00\x6c\x00\x6c\x00\x6f\x00", 5);

  // Single 2-byte UTF-16LE character, 'è'
  test_validate("\xe8\x00", 1);

  // String of 2-byte UTF-16LE characters, "èéêë"
  test_validate("\xe8\x00\xe9\x00\xea\x00\xeb\x00", 4);

  // Single 4-byte UTF-16LE character (represented as a surrogate pair), '𐐷'
  test_validate("\x01\xd8\x37\xdc", 2);

  // String of 4-byte UTF-16LE characters (represented as surrogate pairs), "𐐷𐑏"
  test_validate("\x01\xd8\x37\xdc\x3f\xd8\x1f\xdc", 4);

  // A mix of 2-byte and 4-byte UTF-16LE characters, "Hello𐐷"
  test_validate("\x48\x00\x65\x00\x6c\x00\x6c\x00\x6f\x00\x01\xd8\x37\xdc", 7);
}
