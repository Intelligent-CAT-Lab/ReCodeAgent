#include <stdio.h>
#include "../src/string_t.h"

void print_str(const char *info, const string_t *str) {
    printf("%s: \"%s\"\n", info, string_bytes(str));
}

int main() {
    string_t *hello_str = new_string_from_bytes(" Hello ");

    printf("string_t(bytes=\"%s\", size=%zu)\n", string_bytes(hello_str), string_len(hello_str));
    print_str("concat strings", string_concat(hello_str, new_string_from_bytes(", world!")));
    print_str("string_strip", string_strip(hello_str));
    print_str("string_substring(1, 2)", string_substr(hello_str, 1, 2));
    printf("string_find(\"ll\"): %d\n", string_find(hello_str, "ll"));
    printf("string_startswith(\" Hel\"): %d\n", string_startswith(hello_str, " Hel"));
    printf("string_endswith(\"Hel\"): %d\n", string_endswith(hello_str, "Hel"));

    size_t arr_size = 4;
    const string_t *arr[] = {
            string_strip(hello_str),
            new_string_from_bytes(", "),
            new_string_from_bytes("world"),
            new_string_from_bytes("!")
    };

    printf("%s\n", string_bytes(string_join_arr(arr, arr_size, "")));
    return 0;
}