/*
    MIT License

    Copyright (c) 2023 Ivan Korotaev <vnkrtv@yandex.ru>

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
 */
#ifndef STRING_T_H
#define STRING_T_H

#include <string.h>
#include <stdlib.h>

/*
 * Custom types
 */
#ifndef size_t
#define size_t unsigned long
#endif
#ifndef bool_t
#define bool_t int
#endif
#ifndef true
#define true 1
#endif
#ifndef false
#define false 0
#endif

#ifndef STRING_T_INDEXES_BUFFER_SIZE
#define STRING_T_INDEXES_BUFFER_SIZE 512
#endif

/* C string implementation. */
typedef struct string_t {
    char *bytes;
    size_t size;
} string_t;

#ifndef STRING_T_ARRAY
#define STRING_T_ARRAY string_t **
#endif
#ifndef STRING_T_SPACE_CHARS_ARR
#define STRING_T_SPACE_CHARS_ARR " \t\n\r"
#endif

const char STRING_T_SPACE_CHARS[] = STRING_T_SPACE_CHARS_ARR;

/* Allocate new string by given size. */
string_t *new_string(size_t);

/* Allocate new string by given bytes. */
string_t *new_string_from_bytes(const char *);

/* Free string's allocated memory. */
void string_free(string_t *);

/* Return string length. */
size_t string_len(const string_t *);

/* Return string bytes. */
char *string_bytes(const string_t *);

/* Return true(0) if strings are equal. */
bool_t string_eq(const string_t *, const string_t *);

/* Return copy of the string. */
string_t *string_copy(const string_t *);

/* Return result of 2 strings concatenation. */
string_t *string_concat(const string_t *, const string_t *);

/* Return string's sub string. */
string_t *string_substr(const string_t *, size_t, size_t);

/* Return true(0) if string starts with given prefix. */
bool_t string_startswith(const string_t *, const char[]);

/* Return true(0) if string ends with given suffix. */
bool_t string_endswith(const string_t *, const char[]);

/* Returns the index of substring's occurrence in string. Otherwise returns -1. */
int string_find(const string_t *, const char[]);

/* Return string without STRING_T_SPACE_CHARS_ARR at the start and end. */
string_t *string_strip(const string_t *);

#ifndef _WIN32
/* Split string by STRING_T_SPACE_CHARS_ARR symbols and return strings array. */
STRING_T_ARRAY string_split(const string_t *, size_t *);

/* Split string by given chars and return strings array. */
STRING_T_ARRAY string_split_by(const string_t *, size_t *, const char[]);

#endif
/* Join given strings array with separator chars into one string. */
string_t *string_join_arr(const STRING_T_ARRAY, size_t, const char []);

string_t *new_string(size_t size) {
    string_t *str = malloc(sizeof(string_t));
    str->size = size;
    str->bytes = calloc(size, sizeof(char));
    return str;
}

string_t *new_string_from_bytes(const char *bytes) {
    string_t *str = new_string(strlen(bytes));
    strcpy(str->bytes, bytes);
    return str;
}

void string_free(string_t *str) {
    free(str->bytes);
    free(str);
}

size_t string_len(const string_t *str) {
    return str->size;
}

char *string_bytes(const string_t *str) {
    char *buf = calloc(sizeof(char), str->size);
    strcpy(buf, str->bytes);
    return buf;
}

bool_t string_eq(const string_t *left, const string_t *right) {
    if (left->size != right->size || strcmp(left->bytes, right->bytes) != 0) {
        return false;
    }
    return true;
}

string_t *string_copy(const string_t *str) {
    string_t *copied_str = new_string(str->size);
    strcpy(copied_str->bytes, str->bytes);
    return copied_str;
}

string_t *string_concat(const string_t *first, const string_t *second) {
    string_t *new_str = new_string(first->size + second->size);
    strcpy(new_str->bytes, strcat(first->bytes, second->bytes));
    return new_str;
}

int string_t_is_space_char(char byte) {
    for (size_t idx = 0; idx < strlen(STRING_T_SPACE_CHARS); ++idx) {
        if (STRING_T_SPACE_CHARS[idx] == byte) {
            return 1;
        }
    }
    return 0;
}

string_t *string_substr(const string_t *str, size_t pos, size_t len) {
    string_t *sub_str = new_string(len);
    for (size_t idx = 0; idx < len; idx++) {
        sub_str->bytes[idx] = str->bytes[pos + idx];
    }

    return sub_str;
}

bool_t string_startswith(const string_t *str, const char suffix[]) {
    if (str->size < strlen(suffix)) {
        return false;
    }
    string_t *exp_suffix = new_string_from_bytes(suffix);
    string_t *str_suffix = string_substr(str, 0, exp_suffix->size);

    bool_t is_suffix = string_eq(str_suffix, exp_suffix);
    string_free(str_suffix);
    string_free(exp_suffix);

    return is_suffix;
}

bool_t string_endswith(const string_t *str, const char prefix[]) {
    if (str->size < strlen(prefix)) {
        return false;
    }
    string_t *exp_prefix = new_string_from_bytes(prefix);
    string_t *str_prefix = string_substr(str, str->size - exp_prefix->size, exp_prefix->size);

    bool_t is_suffix = string_eq(str_prefix, exp_prefix);
    string_free(str_prefix);
    string_free(exp_prefix);

    return is_suffix;
}

int string_find(const string_t *str, const char chars[]) {
    string_t *chars_str = new_string_from_bytes(chars);
    if (chars_str->size == 0) {
        string_free(chars_str);
        return 0;
    }
    for (size_t pos = 0; pos < str->size; ++pos) {
        string_t *sub_str = string_substr(str, pos, chars_str->size);
        if (string_eq(sub_str, chars_str) == true) {
            string_free(chars_str);
            string_free(sub_str);
            return (int) pos;
        }
        string_free(sub_str);
    }
    string_free(chars_str);

    return -1;
}

string_t *string_strip(const string_t *str) {
    size_t start_pos = 0;
    for (; start_pos < str->size && string_t_is_space_char(str->bytes[start_pos]); ++start_pos);
    int end_pos = (int) str->size - 1;
    for (; end_pos >= 0 && string_t_is_space_char(str->bytes[end_pos]); --end_pos);

    if ((int)start_pos >= end_pos) {
        return string_copy(str);
    }
    return string_substr(str, start_pos, end_pos - start_pos + 1);
}

#ifndef _WIN32

STRING_T_ARRAY string_split(const string_t *str, size_t *arr_size) {
    size_t str_count = 0;
    size_t indexes[STRING_T_INDEXES_BUFFER_SIZE];

    if (str->size == 0) {
        str_count = 1;
        STRING_T_ARRAY str_arr = calloc(str_count, sizeof(string_t));
        str_arr[0] = string_copy(str);
        if (arr_size) {
            *arr_size = str_count;
        }
        return str_arr;
    }

    size_t start_pos = 0;
    size_t pos = 0;
    for (; pos < str->size;) {
        if (string_t_is_space_char(str->bytes[pos])) {

            indexes[str_count * 2] = start_pos;
            indexes[str_count * 2 + 1] = pos;

            for (; string_t_is_space_char(str->bytes[pos]); pos++);

            start_pos = pos;
            ++str_count;
        } else {
            ++pos;
        }
    }
    if (pos != start_pos) {
        indexes[str_count * 2] = start_pos;
        indexes[str_count * 2 + 1] = pos;
        ++str_count;
    }

    STRING_T_ARRAY str_arr = calloc(str_count, sizeof(string_t));
    for (size_t idx = 0; idx < str_count; ++idx) {
        size_t sub_str_start_pos = indexes[idx * 2];
        size_t sub_str_end_pos = indexes[idx * 2 + 1];

        str_arr[idx] = string_substr(str, sub_str_start_pos, sub_str_end_pos - sub_str_start_pos);
    }
    if (arr_size) {
        *arr_size = str_count;
    }

    return str_arr;
}

STRING_T_ARRAY string_split_by(const string_t *str, size_t *arr_size, const char split_chars[]) {
    size_t str_count = 0;
    size_t indexes[STRING_T_INDEXES_BUFFER_SIZE];

    string_t *split_str = new_string_from_bytes(split_chars);
    if (str->size <= split_str->size) {
        str_count = 1;
        STRING_T_ARRAY str_arr = calloc(str_count, sizeof(string_t));
        str_arr[0] = string_copy(str);
        if (arr_size) {
            *arr_size = str_count;
        }
        string_free(split_str);
        return str_arr;
    }

    size_t start_pos = 0;
    size_t pos = 0;
    for (; pos < str->size - split_str->size;) {
        string_t *sub_str = string_substr(str, pos, split_str->size);
        if (string_eq(sub_str, split_str) == true) {

            indexes[str_count * 2] = start_pos;
            indexes[str_count * 2 + 1] = pos;
            start_pos = pos + split_str->size;

            ++str_count;
            pos += split_str->size;
        } else {
            ++pos;
        }
        string_free(sub_str);
    }
    if (pos != start_pos) {
        indexes[str_count * 2] = start_pos;
        indexes[str_count * 2 + 1] = pos + 1;
        ++str_count;
    }

    string_t **str_arr = calloc(str_count, sizeof(string_t *));
    for (size_t idx = 0; idx < str_count; ++idx) {
        size_t sub_str_start_pos = indexes[idx * 2];
        size_t sub_str_end_pos = indexes[idx * 2 + 1];

        str_arr[idx] = string_substr(str, sub_str_start_pos, sub_str_end_pos - sub_str_start_pos);
    }
    if (arr_size) {
        *arr_size = str_count;
    }
    string_free(split_str);

    return str_arr;
}
#endif

string_t *string_join_arr(const STRING_T_ARRAY str_arr, size_t arr_size, const char space_chars[]) {
    size_t str_size = strlen(space_chars) * (arr_size - 1);  // for space chars
    for (size_t idx = 0; idx < arr_size; ++idx) {
        str_size += str_arr[idx]->size;
    }

    string_t *join_str = new_string(str_size);
    size_t offset = 0;
    for (size_t idx = 0; idx < arr_size; ++idx) {
        strcpy(join_str->bytes + offset, str_arr[idx]->bytes);
        offset += str_arr[idx]->size;
        if (idx != arr_size - 1) {
            strcpy(join_str->bytes + offset, space_chars);
            offset += strlen(space_chars);
        }
    }

    return join_str;
}
#endif //STRING_T_H
