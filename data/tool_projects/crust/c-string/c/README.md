[![Build Status](https://github.com/vnkrtv/c-string/workflows/CI/badge.svg)](https://github.com/vnkrtv/c-string/actions)

# string_t

**One-header C lib for working with strings.**

## Usage

- download `string_t.h` header with lib:
    - `wget https://github.com/vnkrtv/c-string/blob/main/string_t.h -o string_t.h`
- include in your source code:

### Example

Can be run with `make example`.

```c
#include <stdio.h>
#include "string_t.h"

void print_str(const char *info, const string_t *str) {
    printf("%s: \"%s\"\n", info, string_bytes(str));
}

int main(int argc, char **argv) {
    string_t *hello_str = new_string_from_bytes(" Hello ");

    printf("string_t(bytes=\"%s\", size=%zu)\n", string_bytes(hello_str), string_len(hello_str));
    // string_t(bytes=" Hello ", size=7)
    
    print_str("concat strings", string_concat(hello_str, new_string_from_bytes(", world!")));
    // concat strings: " Hello , world!"
    
    print_str("string_strip", string_strip(hello_str));
    // string_strip: "Hello"
    
    print_str("string_substring(1, 2)", string_substr(hello_str, 1, 2));
    // string_substring(1, 2): "He"
    
    printf("string_find(\"ll\"): %d\n", string_find(hello_str, "ll"));
    // string_find("ll"): 3
    
    printf("string_startswith(\" Hel\"): %d\n", string_startswith(hello_str, " Hel"));
    // string_startswith(" Hel"): 0
    
    printf("string_endswith(\"Hel\"): %d\n", string_endswith(hello_str, "Hel"));
    // string_endswith("Hel"): 1
    
    size_t arr_size = 4;
    const string_t *arr[] = {
            string_strip(hello_str),
            new_string_from_bytes(", "),
            new_string_from_bytes("world"),
            new_string_from_bytes("!")
    };

    printf("%s", string_bytes(string_join_arr(arr, arr_size, "")));
    // Hello, world!
    return 0;
}
```

### Dependences

- `string.h`:
    - `string.h` - for strlen, strcmp
    - `stdlib.h` - for malloc/calloc
- `tests/test_string_t.c`
    - `assert.h` - for using asserts
    - `stdio.h` - for printf/fprintf

## Documentation

`string_t` type represents with structure:

```c 
typedef struct string_t {
    char *bytes;
    size_t size;
} string_t;
```

### Macros

| Macro                        | Definition   | Description                                                        |
|------------------------------|--------------|--------------------------------------------------------------------|
| STRING_T_INDEXES_BUFFER_SIZE | `512`        | Tmp array size for indexes in methods string_split/string_split_by |
| STRING_T_SPACE_CHARS_ARR     | `" \t\n\r"`  | Default space chars                                                |
| STRING_T_ARRAY               | `string_t**` | Represents array of strings                                        |

### Methods

| Method                                                                        | Description                                                                  |
|-------------------------------------------------------------------------------|------------------------------------------------------------------------------|
| string_t ***new_string**(size_t);                                             | Allocate new string by given size.                                           |
| string_t ***new_string_from_bytes**(const char *);                            | Allocate new string by given bytes.                                          |
| size_t **string_len**(const string_t *);                                      | Return string length.                                                        |
| size_t **string_free**(string_t *);                                           | Free string's allocated memory.                                              |
| char ***string_bytes**(const string_t *);                                     | Return string bytes.                                                         |
| bool_t **string_eq**(const string_t *, const string_t *);                     | Return true(0) if strings are equal.                                         |
| string_t ***string_copy**(const string_t *);                                  | Return copy of the string.                                                   |
| string_t ***string_concat**(const string_t *, const string_t *);              | Return result of 2 strings concatenation.                                    |
| string_t ***string_substr**(const string_t *, size_t, size_t);                | Return string's sub string.                                                  |
| bool_t **string_startswith**(const string_t *, const char[]);                 | Return true(0) if string starts with given prefix.                           |
| bool_t **string_endswith**(const string_t *, const char[]);                   | Return true(0) if string ends with given suffix.                             |
| int **string_find**(const string_t *, const char[]);                          | Returns the index of substring's occurrence in string. Otherwise returns -1. |
| string_t ***string_strip**(const string_t *);                                 | Return string without STRING_T_SPACE_CHARS_ARR at the start and end.         |
| STRING_T_ARRAY **string_split**(const string_t *, size_t *);                  | Split string by STRING_T_SPACE_CHARS_ARR symbols and return strings array.   |
| STRING_T_ARRAY **string_split_by**(const string_t *, size_t *, const char[]); | Split string by given chars and return strings array.                        |
| string_t ***string_join_arr**(const STRING_T_ARRAY, size_t, const char []);   | Join given strings array with separator chars into one string.               |

## Tests

Run tests:

- `make test`
