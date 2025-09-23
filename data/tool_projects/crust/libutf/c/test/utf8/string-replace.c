#include <assert.h>
#include <string.h>

#include "../../include/utf.h"
#include "../../include/utf/string.h"

int
main () {
  int e;

  utf8_string_t string;
  utf8_string_init(&string);

  e = utf8_string_append_literal(&string, (utf8_t *) "hello world", -1);
  assert(e == 0);

  e = utf8_string_replace_literal(&string, 5, 1, (utf8_t *) " there ", -1);
  assert(e == 0);

  assert(memcmp(string.data, "hello there world", string.len) == 0);

  utf8_string_destroy(&string);
}
