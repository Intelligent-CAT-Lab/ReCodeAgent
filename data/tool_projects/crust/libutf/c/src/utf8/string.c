#include <stdbool.h>
#include <stddef.h>

#include "../../include/utf.h"
#include "../../include/utf/string.h"

extern void
utf8_string_init (utf8_string_t *string);

extern utf8_string_view_t
utf8_string_view_init (const utf8_t *data, size_t len);

extern void
utf8_string_destroy (utf8_string_t *string);

extern int
utf8_string_reserve (utf8_string_t *string, size_t len);

extern int
utf8_string_shrink_to_fit (utf8_string_t *string);

extern utf8_string_view_t
utf8_string_view (const utf8_string_t *string);

extern void
utf8_string_clear (utf8_string_t *string);

extern bool
utf8_string_empty (const utf8_string_t *string);

extern bool
utf8_string_view_empty (const utf8_string_view_t view);

extern int
utf8_string_copy (const utf8_string_t *string, utf8_string_t *result);

extern int
utf8_string_view_copy (const utf8_string_view_t view, utf8_string_t *result);

extern int
utf8_string_append (utf8_string_t *string, const utf8_string_t *other);

extern int
utf8_string_append_view (utf8_string_t *string, const utf8_string_view_t view);

extern int
utf8_string_append_character (utf8_string_t *string, utf8_t c);

extern int
utf8_string_append_literal (utf8_string_t *string, const utf8_t *literal, size_t n);

extern int
utf8_string_prepend (utf8_string_t *string, const utf8_string_t *other);

extern int
utf8_string_prepend_view (utf8_string_t *string, const utf8_string_view_t view);

extern int
utf8_string_prepend_character (utf8_string_t *string, utf8_t c);

extern int
utf8_string_prepend_literal (utf8_string_t *string, const utf8_t *literal, size_t n);

extern int
utf8_string_insert (utf8_string_t *string, size_t pos, const utf8_string_t *other);

extern int
utf8_string_insert_view (utf8_string_t *string, size_t pos, const utf8_string_view_t other);

extern int
utf8_string_insert_character (utf8_string_t *string, size_t pos, utf8_t c);

extern int
utf8_string_insert_literal (utf8_string_t *string, size_t pos, const utf8_t *literal, size_t n);

extern int
utf8_string_replace (utf8_string_t *string, size_t pos, size_t len, const utf8_string_t *replacement);

extern int
utf8_string_replace_view (utf8_string_t *string, size_t pos, size_t len, const utf8_string_view_t replacement);

extern int
utf8_string_replace_character (utf8_string_t *string, size_t pos, size_t len, utf8_t c);

extern int
utf8_string_replace_literal (utf8_string_t *string, size_t pos, size_t len, const utf8_t *literal, size_t n);

extern int
utf8_string_erase (utf8_string_t *string, size_t pos, size_t len);

extern int
utf8_string_concat (const utf8_string_t *string, const utf8_string_t *other, utf8_string_t *result);

extern int
utf8_string_view_concat (const utf8_string_view_t view, const utf8_string_t *other, utf8_string_t *result);

extern int
utf8_string_concat_view (const utf8_string_t *string, const utf8_string_view_t other, utf8_string_t *result);

extern int
utf8_string_view_concat_view (const utf8_string_view_t view, const utf8_string_view_t other, utf8_string_t *result);

extern int
utf8_string_concat_character (const utf8_string_t *string, utf8_t c, utf8_string_t *result);

extern int
utf8_string_view_concat_character (const utf8_string_view_t view, utf8_t c, utf8_string_t *result);

extern int
utf8_string_concat_literal (const utf8_string_t *string, const utf8_t *literal, size_t n, utf8_string_t *result);

extern int
utf8_string_view_concat_literal (const utf8_string_view_t view, const utf8_t *literal, size_t n, utf8_string_t *result);

extern int
utf8_string_compare (const utf8_string_t *string, const utf8_string_t *other);

extern int
utf8_string_view_compare (const utf8_string_view_t view, const utf8_string_view_t other);

extern int
utf8_string_compare_literal (const utf8_string_t *string, const utf8_t *literal, size_t n);

extern int
utf8_string_view_compare_literal (const utf8_string_view_t view, const utf8_t *literal, size_t n);

extern utf8_string_view_t
utf8_string_substring (const utf8_string_t *string, size_t start, size_t end);

extern utf8_string_view_t
utf8_string_view_substring (const utf8_string_view_t view, size_t start, size_t end);

extern int
utf8_string_substring_copy (const utf8_string_t *string, size_t start, size_t end, utf8_string_t *result);

extern int
utf8_string_view_substring_copy (const utf8_string_view_t view, size_t start, size_t end, utf8_string_t *result);

extern size_t
utf8_string_index_of_character (const utf8_string_t *string, size_t pos, utf8_t c);

extern size_t
utf8_string_view_index_of_character (const utf8_string_view_t view, size_t pos, utf8_t c);

extern size_t
utf8_string_last_index_of_character (const utf8_string_t *string, size_t pos, utf8_t c);

extern size_t
utf8_string_view_last_index_of_character (const utf8_string_view_t view, size_t pos, utf8_t c);
