#include <assert.h>
#include <string.h>

#include "../../include/utf.h"
#include "../../include/utf/string.h"

int
main () {
  int e;

  utf8_string_t string;
  utf8_string_init(&string);

  e = utf8_string_prepend_literal(&string, (utf8_t *) " world", -1);
  assert(e == 0);

  e = utf8_string_prepend_literal(&string, (utf8_t *) "hello", -1);
  assert(e == 0);

  assert(memcmp(string.data, "hello world", string.len) == 0);

  utf8_string_destroy(&string);
}
