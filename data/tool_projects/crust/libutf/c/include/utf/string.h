#ifndef UTF_STRING_H
#define UTF_STRING_H

#ifdef __cplusplus
extern "C" {
#endif

#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../utf.h"

/**
 * UTF-8
 */

typedef struct utf8_string_s utf8_string_t;
typedef struct utf8_string_view_s utf8_string_view_t;

struct utf8_string_s {
  utf8_t *data;
  size_t len;
  union {
    size_t cap;
    utf8_t buf[8];
  };
};

struct utf8_string_view_s {
  const utf8_t *data;
  size_t len;
};

inline void
utf8_string_init (utf8_string_t *string) {
  string->data = string->buf;
  string->len = 0;
}

inline utf8_string_view_t
utf8_string_view_init (const utf8_t *data, size_t len) {
  utf8_string_view_t view = {data, len};
  return view;
}

inline void
utf8_string_destroy (utf8_string_t *string) {
  if (string->data != string->buf) free(string->data);
}

inline int
utf8_string_reserve (utf8_string_t *string, size_t len) {
  size_t cap = string->data == string->buf ? sizeof(string->buf) : string->cap;

  if (len <= cap) return 0;

  // https://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
  len--;
  len |= len >> 1;
  len |= len >> 2;
  len |= len >> 4;
  len |= len >> 8;
  len |= len >> 16;
  if (sizeof(len) == 8) len |= len >> 32;
  len++;

  cap = len;

  utf8_t *data;

  if (string->data == string->buf) {
    data = (utf8_t *) malloc(cap);

    memcpy(data, string->data, string->len);
  } else {
    data = (utf8_t *) realloc(string->data, cap);
  }

  if (data == NULL) return -1;

  string->data = data;
  string->cap = cap;

  return 0;
}

inline int
utf8_string_shrink_to_fit (utf8_string_t *string) {
  if (string->data == string->buf) return 0;

  size_t cap = string->len;

  if (cap <= sizeof(string->buf)) {
    memcpy(string->buf, string->data, cap);

    free(string->data);

    string->data = string->buf;
  } else {
    utf8_t *data = (utf8_t *) realloc(string->data, cap);

    if (data == NULL) return -1;

    string->data = data;
    string->cap = cap;
  }

  return 0;
}

inline utf8_string_view_t
utf8_string_view (const utf8_string_t *string) {
  return utf8_string_view_init(string->data, string->len);
}

inline void
utf8_string_clear (utf8_string_t *string) {
  string->len = 0;
}

inline bool
utf8_string_empty (const utf8_string_t *string) {
  return string->len == 0;
}

inline bool
utf8_string_view_empty (const utf8_string_view_t view) {
  return view.len == 0;
}

inline int
utf8_string_copy (const utf8_string_t *string, utf8_string_t *result) {
  int err;

  err = utf8_string_reserve(result, string->len);
  if (err < 0) return err;

  memcpy(result->data, string->data, string->len);

  result->len = string->len;

  return 0;
}

inline int
utf8_string_view_copy (const utf8_string_view_t view, utf8_string_t *result) {
  int err;

  err = utf8_string_reserve(result, view.len);
  if (err < 0) return err;

  memcpy(result->data, view.data, view.len);

  result->len = view.len;

  return 0;
}

inline int
utf8_string_append (utf8_string_t *string, const utf8_string_t *other) {
  int err;

  err = utf8_string_reserve(string, string->len + other->len);
  if (err < 0) return -1;

  memcpy(&string->data[string->len], other->data, other->len);

  string->len += other->len;

  return 0;
}

inline int
utf8_string_append_view (utf8_string_t *string, const utf8_string_view_t view) {
  int err;

  err = utf8_string_reserve(string, string->len + view.len);
  if (err < 0) return -1;

  memcpy(&string->data[string->len], view.data, view.len);

  string->len += view.len;

  return 0;
}

inline int
utf8_string_append_character (utf8_string_t *string, utf8_t c) {
  int err;

  err = utf8_string_reserve(string, string->len + 1);
  if (err < 0) return -1;

  string->data[string->len] = c;

  string->len += 1;

  return 0;
}

inline int
utf8_string_append_literal (utf8_string_t *string, const utf8_t *literal, size_t n) {
  int err;

  if (n == (size_t) -1) n = strlen((const char *) literal);

  err = utf8_string_reserve(string, string->len + n);
  if (err < 0) return -1;

  memcpy(&string->data[string->len], literal, n);

  string->len += n;

  return 0;
}

inline int
utf8_string_prepend (utf8_string_t *string, const utf8_string_t *other) {
  int err;

  err = utf8_string_reserve(string, string->len + other->len);
  if (err < 0) return -1;

  memmove(&string->data[other->len], string->data, string->len);

  memcpy(string->data, other->data, other->len);

  string->len += other->len;

  return 0;
}

inline int
utf8_string_prepend_view (utf8_string_t *string, const utf8_string_view_t view) {
  int err;

  err = utf8_string_reserve(string, string->len + view.len);
  if (err < 0) return -1;

  memmove(&string->data[view.len], string->data, string->len);

  memcpy(string->data, view.data, view.len);

  string->len += view.len;

  return 0;
}

inline int
utf8_string_prepend_character (utf8_string_t *string, utf8_t c) {
  int err;

  err = utf8_string_reserve(string, string->len + 1);
  if (err < 0) return -1;

  memmove(&string->data[1], string->data, string->len);

  string->data[0] = c;

  string->len += 1;

  return 0;
}

inline int
utf8_string_prepend_literal (utf8_string_t *string, const utf8_t *literal, size_t n) {
  int err;

  if (n == (size_t) -1) n = strlen((const char *) literal);

  err = utf8_string_reserve(string, string->len + n);
  if (err < 0) return -1;

  memmove(&string->data[n], string->data, string->len);

  memcpy(string->data, literal, n);

  string->len += n;

  return 0;
}

inline int
utf8_string_insert (utf8_string_t *string, size_t pos, const utf8_string_t *other) {
  int err;

  if (pos > string->len) return -1;

  size_t inserted_len = string->len + other->len;

  err = utf8_string_reserve(string, inserted_len);
  if (err < 0) return err;

  memmove(&string->data[pos + other->len], &string->data[pos], string->len - pos);

  memcpy(&string->data[pos], other->data, other->len);

  string->len = inserted_len;

  return 0;
}

inline int
utf8_string_insert_view (utf8_string_t *string, size_t pos, const utf8_string_view_t other) {
  int err;

  if (pos > string->len) return -1;

  size_t inserted_len = string->len + other.len;

  err = utf8_string_reserve(string, inserted_len);
  if (err < 0) return err;

  memmove(&string->data[pos + other.len], &string->data[pos], string->len - pos);

  memcpy(&string->data[pos], other.data, other.len);

  string->len = inserted_len;

  return 0;
}

inline int
utf8_string_insert_character (utf8_string_t *string, size_t pos, utf8_t c) {
  int err;

  if (pos > string->len) return -1;

  size_t inserted_len = string->len + 1;

  err = utf8_string_reserve(string, inserted_len);
  if (err < 0) return err;

  memmove(&string->data[pos + 1], &string->data[pos], string->len - pos);

  string->data[pos] = c;

  string->len = inserted_len;

  return 0;
}

inline int
utf8_string_insert_literal (utf8_string_t *string, size_t pos, const utf8_t *literal, size_t n) {
  int err;

  if (pos > string->len) return -1;

  if (n == (size_t) -1) n = strlen((const char *) literal);

  size_t inserted_len = string->len + n;

  err = utf8_string_reserve(string, inserted_len);
  if (err < 0) return err;

  memmove(&string->data[pos + n], &string->data[pos], string->len - pos);

  memcpy(&string->data[pos], literal, n);

  string->len = inserted_len;

  return 0;
}

inline int
utf8_string_replace (utf8_string_t *string, size_t pos, size_t len, const utf8_string_t *replacement) {
  int err;

  if (pos > string->len) return -1;
  if (pos + len > string->len) len = string->len - pos;

  size_t replaced_len = string->len + replacement->len - len;

  err = utf8_string_reserve(string, replaced_len);
  if (err < 0) return err;

  memmove(&string->data[pos + replacement->len], &string->data[pos + len], string->len - pos - len);

  memcpy(&string->data[pos], replacement->data, replacement->len);

  string->len = replaced_len;

  return 0;
}

inline int
utf8_string_replace_view (utf8_string_t *string, size_t pos, size_t len, const utf8_string_view_t replacement) {
  int err;

  if (pos > string->len) return -1;
  if (pos + len > string->len) len = string->len - pos;

  size_t replaced_len = string->len + replacement.len - len;

  err = utf8_string_reserve(string, replaced_len);
  if (err < 0) return err;

  memmove(&string->data[pos + replacement.len], &string->data[pos + len], string->len - pos - len);

  memcpy(&string->data[pos], replacement.data, replacement.len);

  string->len = replaced_len;

  return 0;
}

inline int
utf8_string_replace_character (utf8_string_t *string, size_t pos, size_t len, utf8_t c) {
  int err;

  if (pos > string->len) return -1;

  size_t replaced_len = string->len + 1 - len;

  err = utf8_string_reserve(string, replaced_len);
  if (err < 0) return err;

  memmove(&string->data[pos + 1], &string->data[pos + len], string->len - pos - len);

  string->data[pos] = c;

  string->len = replaced_len;

  return 0;
}

inline int
utf8_string_replace_literal (utf8_string_t *string, size_t pos, size_t len, const utf8_t *literal, size_t n) {
  int err;

  if (pos > string->len) return -1;
  if (pos + len > string->len) len = string->len - pos;

  if (n == (size_t) -1) n = strlen((const char *) literal);

  size_t replaced_len = string->len + n - len;

  err = utf8_string_reserve(string, replaced_len);
  if (err < 0) return err;

  memmove(&string->data[pos + n], &string->data[pos + len], string->len - pos - len);

  memcpy(&string->data[pos], literal, n);

  string->len = replaced_len;

  return 0;
}

inline int
utf8_string_erase (utf8_string_t *string, size_t pos, size_t len) {
  int err;

  if (pos > string->len) return -1;
  if (pos + len > string->len) len = string->len - pos;

  memmove(&string->data[pos], &string->data[pos + len], string->len - pos - len);

  string->len -= len;

  return 0;
}

inline int
utf8_string_concat (const utf8_string_t *string, const utf8_string_t *other, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, string->len + other->len);
  if (err < 0) return err;

  err = utf8_string_append(result, string);
  assert(err == 0);

  err = utf8_string_append(result, other);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_view_concat (const utf8_string_view_t view, const utf8_string_t *other, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, view.len + other->len);
  if (err < 0) return err;

  err = utf8_string_append_view(result, view);
  assert(err == 0);

  err = utf8_string_append(result, other);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_concat_view (const utf8_string_t *string, const utf8_string_view_t other, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, string->len + other.len);
  if (err < 0) return err;

  err = utf8_string_append(result, string);
  assert(err == 0);

  err = utf8_string_append_view(result, other);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_view_concat_view (const utf8_string_view_t view, const utf8_string_view_t other, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, view.len + other.len);
  if (err < 0) return err;

  err = utf8_string_append_view(result, view);
  assert(err == 0);

  err = utf8_string_append_view(result, other);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_concat_character (const utf8_string_t *string, utf8_t c, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, string->len + 1);
  if (err < 0) return err;

  err = utf8_string_append(result, string);
  assert(err == 0);

  err = utf8_string_append_character(result, c);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_view_concat_character (const utf8_string_view_t view, utf8_t c, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, view.len + 1);
  if (err < 0) return err;

  err = utf8_string_append_view(result, view);
  assert(err == 0);

  err = utf8_string_append_character(result, c);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_concat_literal (const utf8_string_t *string, const utf8_t *literal, size_t n, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, string->len + n);
  if (err < 0) return err;

  err = utf8_string_append(result, string);
  assert(err == 0);

  err = utf8_string_append_literal(result, literal, n);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_view_concat_literal (const utf8_string_view_t view, const utf8_t *literal, size_t n, utf8_string_t *result) {
  int err;

  utf8_string_init(result);

  err = utf8_string_reserve(result, view.len + n);
  if (err < 0) return err;

  err = utf8_string_append_view(result, view);
  assert(err == 0);

  err = utf8_string_append_literal(result, literal, n);
  assert(err == 0);

  return 0;
}

inline int
utf8_string_compare (const utf8_string_t *string, const utf8_string_t *other) {
  size_t a_len = string->len;
  size_t b_len = other->len;

  int result = strncmp((const char *) string->data, (const char *) other->data, a_len < b_len ? a_len : b_len);

  if (result == 0) return a_len < b_len
                            ? -1
                          : a_len > b_len ? 1
                                          : 0;

  return result;
}

inline int
utf8_string_view_compare (const utf8_string_view_t view, const utf8_string_view_t other) {
  size_t a_len = view.len;
  size_t b_len = other.len;

  int result = strncmp((const char *) view.data, (const char *) other.data, a_len < b_len ? a_len : b_len);

  if (result == 0) return a_len < b_len
                            ? -1
                          : a_len > b_len ? 1
                                          : 0;

  return result;
}

inline int
utf8_string_compare_literal (const utf8_string_t *string, const utf8_t *literal, size_t n) {
  if (n == (size_t) -1) n = strlen((const char *) literal);

  size_t a_len = string->len;
  size_t b_len = n;

  int result = strncmp((const char *) string->data, (const char *) literal, a_len < b_len ? a_len : b_len);

  if (result == 0) return a_len < b_len
                            ? -1
                          : a_len > b_len ? 1
                                          : 0;

  return result;
}

inline int
utf8_string_view_compare_literal (const utf8_string_view_t view, const utf8_t *literal, size_t n) {
  if (n == (size_t) -1) n = strlen((const char *) literal);

  size_t a_len = view.len;
  size_t b_len = n;

  int result = strncmp((const char *) view.data, (const char *) literal, a_len < b_len ? a_len : b_len);

  if (result == 0) return a_len < b_len
                            ? -1
                          : a_len > b_len ? 1
                                          : 0;

  return result;
}

inline utf8_string_view_t
utf8_string_substring (const utf8_string_t *string, size_t start, size_t end) {
  if (end == (size_t) -1 || end > string->len) end = string->len;
  if (start > end) start = end;

  return utf8_string_view_init(&string->data[start], end - start);
}

inline utf8_string_view_t
utf8_string_view_substring (const utf8_string_view_t view, size_t start, size_t end) {
  if (end == (size_t) -1 || end > view.len) end = view.len;
  if (start > end) start = end;

  return utf8_string_view_init(&view.data[start], end - start);
}

inline int
utf8_string_substring_copy (const utf8_string_t *string, size_t start, size_t end, utf8_string_t *result) {
  int err;

  if (end == (size_t) -1 || end > string->len) end = string->len;
  if (start > end) start = end;

  utf8_string_init(result);

  size_t len = end - start;

  err = utf8_string_reserve(result, len);
  if (err < 0) return -1;

  memcpy(result->data, &string->data[start], len);

  result->len = len;

  return 0;
}

inline int
utf8_string_view_substring_copy (const utf8_string_view_t view, size_t start, size_t end, utf8_string_t *result) {
  int err;

  if (end == (size_t) -1 || end > view.len) end = view.len;
  if (start > end) start = end;

  size_t len = end - start;

  err = utf8_string_reserve(result, len);
  if (err < 0) return -1;

  memcpy(result->data, &view.data[start], len);

  result->len = len;

  return 0;
}

inline size_t
utf8_string_index_of_character (const utf8_string_t *string, size_t pos, utf8_t c) {
  for (size_t i = pos, n = string->len; i < n; i++) {
    if (string->data[i] == c) {
      return i;
    }
  }

  return (size_t) -1;
}

inline size_t
utf8_string_view_index_of_character (const utf8_string_view_t view, size_t pos, utf8_t c) {
  for (size_t i = pos, n = view.len; i < n; i++) {
    if (view.data[i] == c) {
      return i;
    }
  }

  return (size_t) -1;
}

inline size_t
utf8_string_last_index_of_character (const utf8_string_t *string, size_t pos, utf8_t c) {
  if (pos == (size_t) -1) pos = string->len - 1;
  else if (pos >= string->len) return (size_t) -1;

  for (size_t i = pos; i <= pos; i--) {
    if (string->data[i] == c) {
      return i;
    }
  }

  return (size_t) -1;
}

inline size_t
utf8_string_view_last_index_of_character (const utf8_string_view_t view, size_t pos, utf8_t c) {
  if (pos == (size_t) -1) pos = view.len - 1;
  else if (pos >= view.len) return (size_t) -1;

  for (size_t i = pos; i <= pos; i--) {
    if (view.data[i] == c) {
      return i;
    }
  }

  return (size_t) -1;
}

#ifdef __cplusplus
}
#endif

#endif // UTF_STRING_H
