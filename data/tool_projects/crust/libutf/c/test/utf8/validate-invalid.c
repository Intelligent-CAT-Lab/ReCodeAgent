#include <assert.h>

#include "../../include/utf.h"

#define test_validate(string, len) \
  { \
    assert(!utf8_validate((utf8_t *) string, len)); \
  }

int
main () {
  test_validate("\x80", 1);                     // The byte is 10000000 in binary. The first bit suggests it's a continuation byte, but there's no start byte.
  test_validate("\xc3", 1);                     // This byte indicates the start of a 2-byte sequence, but it's missing the second byte.
  test_validate("\xc0\x80", 2);                 // Overlong encoding of NULL character, which should be 0x00 in UTF-8
  test_validate("\xc3\xa9\xa8", 3);             // This starts as a valid 2-byte sequence, but then has an unexpected third byte.
  test_validate("\x80\x80", 2);                 // This sequence starts with a continuation byte, which is not allowed.
  test_validate("\xfe", 1);                     // This byte indicates the start of a sequence of length 7, but UTF-8 only supports up to 4-byte sequences.
  test_validate("\xe2\x82", 2);                 // This start byte indicates a 3-byte sequence, but only one additional byte is provided.
  test_validate("\xf4\x90\x80\x80", 4);         // This sequence decodes to U+110000, which is beyond the U+10FFFF limit of Unicode.
  test_validate("\xed\xa0\x80", 3);             // This sequence decodes to U+D800, a high surrogate.
  test_validate("\xed\xbf\xbf", 3);             // This sequence decodes to U+DFFF, a low surrogate.
  test_validate("\xe2\x82\xc0", 3);             // The last byte should be a continuation byte (10xxxxxx), but it's a start byte (110xxxxx).
  test_validate("\x80\xc2", 2);                 // This sequence starts with a continuation byte, which is not allowed.
  test_validate("\xf8\x88\x88\x88\x88", 5);     // UTF-8 only supports up to 4-byte sequences.
  test_validate("\xfc\x84\x84\x84\x84\x84", 6); // UTF-8 only supports up to 4-byte sequences.
}
