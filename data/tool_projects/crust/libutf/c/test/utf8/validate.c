#include <assert.h>

#include "../../include/utf.h"

#define test_validate(string, len) \
  { \
    assert(utf8_validate((utf8_t *) string, len)); \
  }

int
main () {
  test_validate("a", 1);                                                 // Single ASCII character
  test_validate("Hello, World!", 13);                                    // ASCII characters
  test_validate("\xc3\xa9", 2);                                          // Single 2-byte UTF-8 character
  test_validate("\xc3\xa9\xc3\xa8\xc3\xaa\xc3\xab\xc3\xaf", 10);         // String of 2-byte UTF-8 characters
  test_validate("\xe2\x82\xac", 3);                                      // Single 3-byte UTF-8 character
  test_validate("\xe2\x82\xac\xe2\x89\xa1\xe2\x9c\x93", 9);              // String of 3-byte UTF-8 characters
  test_validate("\xf0\x9f\x98\x80", 4);                                  // Single 4-byte UTF-8 character
  test_validate("\xf0\x9f\x98\x80\xf0\x9f\x98\x81\xf0\x9f\x98\x82", 12); // String of 4-byte UTF-8 characters
  test_validate("a \xc3\xa9 \xe2\x82\xac \xf0\x9f\x98\x80", 13);         // Mix of all
}
