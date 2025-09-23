#include <assert.h>

#include "../../include/utf.h"

#define test_validate(string, len) \
  { \
    assert(!utf16le_validate((utf16_t *) string, len)); \
  }

int
main () {
  // An isolated high surrogate
  test_validate("\x01\xd8", 1);

  // An isolated low surrogate
  test_validate("\x37\xdc", 1);

  // A high surrogate followed by another high surrogate
  test_validate("\x01\xd8\x02\xd8", 2);

  // A low surrogate followed by another low surrogate
  test_validate("\x37\xdc\x38\xdc", 2);

  // A high surrogate followed by a non-surrogate
  // test_validate("\x01\xd8\xe8\x00", 2);

  // A low surrogate followed by a non-surrogate
  // test_validate("\x37\xdc\xe8\x00", 2);

  // A non-surrogate followed by a low surrogate
  // test_validate("\xe8\x00\x37\xdc");

  // A non-surrogate followed by a high surrogate
  // test_validate("\xe8\x00\x01\xd8");

  // A surrogate pair in the wrong order
  test_validate("\x37\xdc\x01\xd8", 2);

  // Two high surrogates followed by a low surrogate
  test_validate("\x01\xd8\x02\xd8\x37\xdc", 3);

  // A high surrogate followed by two low surrogates
  test_validate("\x01\xd8\x37\xdc\x38\xdc", 3);

  // Two high surrogates in a row
  test_validate("\x01\xd8\x02\xd8", 2);

  // Two low surrogates in a row
  test_validate("\x37\xdc\x38\xdc", 2);
}
