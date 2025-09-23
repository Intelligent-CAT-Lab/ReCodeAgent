#ifndef UTF_H
#define UTF_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef unsigned char utf8_t;
typedef uint_least16_t utf16_t;
typedef uint_least32_t utf32_t;
typedef unsigned char latin1_t;
typedef unsigned char ascii_t;

/**
 * UTF-8
 */

bool
utf8_validate (const utf8_t *data, size_t len);

size_t
utf8_length_from_utf16le (const utf16_t *data, size_t len);

size_t
utf8_length_from_utf32 (const utf32_t *data, size_t len);

size_t
utf8_length_from_latin1 (const latin1_t *data, size_t len);

size_t
utf8_convert_to_utf16le (const utf8_t *data, size_t len, utf16_t *result);

size_t
utf8_convert_to_utf32 (const utf8_t *data, size_t len, utf32_t *result);

size_t
utf8_convert_to_latin1 (const utf8_t *data, size_t len, latin1_t *result);

/**
 * UTF-16
 */

bool
utf16le_validate (const utf16_t *data, size_t len);

size_t
utf16_length_from_utf8 (const utf8_t *data, size_t len);

size_t
utf16_length_from_utf32 (const utf32_t *data, size_t len);

size_t
utf16_length_from_latin1 (const latin1_t *data, size_t len);

size_t
utf16le_convert_to_utf8 (const utf16_t *data, size_t len, utf8_t *result);

size_t
utf16le_convert_to_utf32 (const utf16_t *data, size_t len, utf32_t *result);

size_t
utf16le_convert_to_latin1 (const utf16_t *data, size_t len, latin1_t *result);

/**
 * UTF-32
 */

bool
utf32_validate (const utf32_t *data, size_t len);

size_t
utf32_length_from_utf8 (const utf8_t *data, size_t len);

size_t
utf32_length_from_utf16le (const utf16_t *data, size_t len);

size_t
utf32_length_from_latin1 (const latin1_t *data, size_t len);

size_t
utf32_convert_to_utf8 (const utf32_t *data, size_t len, utf8_t *result);

size_t
utf32_convert_to_utf16le (const utf32_t *data, size_t len, utf16_t *result);

size_t
utf32_convert_to_latin1 (const utf32_t *data, size_t len, latin1_t *result);

/**
 * Latin-1
 */

size_t
latin1_length_from_utf8 (const utf8_t *data, size_t len);

size_t
latin1_length_from_utf16le (const utf16_t *data, size_t len);

size_t
latin1_length_from_utf32 (const utf32_t *data, size_t len);

size_t
latin1_convert_to_utf8 (const latin1_t *data, size_t len, utf8_t *result);

size_t
latin1_convert_to_utf16le (const latin1_t *data, size_t len, utf16_t *result);

size_t
latin1_convert_to_utf32 (const latin1_t *data, size_t len, utf32_t *result);

/**
 * ASCII
 */

bool
ascii_validate (const ascii_t *data, size_t len);

/**
 * Strings
 */

#include <utf/string.h>

#ifdef __cplusplus
}
#endif

#endif // UTF_H
