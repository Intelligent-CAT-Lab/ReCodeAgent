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

  utf8_string_view_t view = utf8_string_substring(&string, 2, 8);

  assert(memcmp(view.data, "llo worl", view.len) == 0);

  utf8_string_destroy(&string);
}
