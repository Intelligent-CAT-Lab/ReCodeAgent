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
#include <stdio.h>
#include <assert.h>


#include "../src/string_t.h"

#define DUMP_STRING(str) \
    fprintf(stderr, "string_t(bytes=\"%s\", size=%zu)", str->bytes, str->size)
#define LOG_STRING(arg, str) \
    fprintf(stderr, "%s=", arg); \
    DUMP_STRING(str); \
    fprintf(stderr, "\n")

/* Simple linked list C implementation. */
typedef struct node_t {
    struct node_t *next;
    void *value;
} node_t;

node_t *new_node(void *);

node_t *add_node(node_t *, void *);

node_t *new_node(void *value) {
    node_t *created_node = malloc(sizeof(node_t));
    created_node->next = NULL;
    created_node->value = value;
    return created_node;
}

node_t *add_node(node_t *node, void *value) {
    node_t *created_node = new_node(value);
    created_node->next = NULL;
    created_node->value = value;

    node->next = created_node;
    return created_node;
}

void node_free(node_t *node) {
    if (node->value != NULL) {
        free(node->value);
    }
    free(node);
}

/*
 * Test structures
 */
typedef void (*test_func_t)(void);

/* Test struct. */
typedef struct test_t {
    test_func_t test;
    char *description;
} test_t;

void test_free(test_t *test) {
    free(test->description);
}

/* Struct for organizing test running. */
typedef struct test_runner_t {
    node_t *first_test;
    node_t *last_test;
} test_runner_t;

/* Create new test runner. */
test_runner_t new_runner();

/* Register test for test runner. */
void register_test(test_runner_t *, test_func_t, char *);

/* Run registered tests. */
int run_tests(test_runner_t *);

/* Free test runner. */
void runner_free(test_runner_t *);

test_runner_t new_runner() {
    test_runner_t runner;
    runner.first_test = NULL;
    runner.last_test = NULL;
    return runner;
}

void register_test(test_runner_t *test_runner, test_func_t test, char *description) {
    test_t *new_test = malloc(sizeof(test_t));
    new_test->test = test;
    new_test->description = malloc(strlen(description));
    strcpy(new_test->description, description);
    if (test_runner->first_test == NULL) {
        test_runner->first_test = new_node(new_test);
        test_runner->last_test = test_runner->first_test;
    } else {
        test_runner->last_test = add_node(test_runner->last_test, new_test);
    }
}

int run_tests(test_runner_t *test_runner) {
    node_t *cur_test = test_runner->first_test;
    while (cur_test != NULL) {
        test_t *test = (test_t *) cur_test->value;
        test->test();
        printf("%s: OK\n", test->description);
        cur_test = cur_test->next;
    }
    return 0;
}

void runner_free(test_runner_t *test_runner) {
    node_t *cur_test = test_runner->first_test;
    while (cur_test != NULL) {
        test_t *test = (test_t *) cur_test->value;
        test_free(test);
        node_t *node = cur_test;
        cur_test = cur_test->next;
        node_free(node);
    }
}

/*
 * string_t tests
 */
void string_arr_free(STRING_T_ARRAY str_arr, size_t size) {
    for (size_t idx = 0; idx < size; ++idx) {
        string_free(str_arr[idx]);
    }
}

void test_new_string(void) {
    size_t sizes[] = {0, 1, 2, 3, 12, 100};
    for (size_t idx = 0; idx < 5; ++idx) {
        string_t *str = new_string(sizes[idx]);
        assert(str->size == sizes[idx]);
        string_free(str);
    }
}

void test_new_string_from_bytes(void) {
    char *bytes[] = {"", "test", "some another test"};
    for (size_t idx = 0; idx < 3; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        assert(str->size == strlen(bytes[idx]));
        assert(strcmp(str->bytes, bytes[idx]) == 0);
        string_free(str);
    }
}

void test_string_len(void) {
    char *bytes[] = {"", "test", "some another test"};
    size_t len[] = {0, 4, 17};
    for (size_t idx = 0; idx < 3; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        assert(str->size == strlen(bytes[idx]));
        assert(string_len(str) == len[idx]);
        string_free(str);
    }
}

void test_string_bytes(void) {
    char *bytes[] = {"", "test", "some another test"};
    for (size_t idx = 0; idx < 3; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        char *str_bytes = string_bytes(str);
        assert(strcmp(str_bytes, bytes[idx]) == 0);
        string_free(str);
        free(str_bytes);
    }
}

void test_string_eq(void) {
    char *left_bytes[] = {"", "\n\n", "test", "some another test"};
    char *right_bytes[] = {"", "\n\n", "test", "some another test"};

    for (size_t idx = 0; idx < 4; ++idx) {
        string_t *left_str = new_string_from_bytes(left_bytes[idx]);
        string_t *right_str = new_string_from_bytes(right_bytes[idx]);

        assert(left_str->size == right_str->size);
        assert(strcmp(left_str->bytes, right_str->bytes) == 0);

        string_free(left_str);
        string_free(right_str);
    }
}

void test_string_copy(void) {
    char *bytes[] = {"", "test", "some another test"};
    for (size_t idx = 0; idx < 3; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        string_t *str_copy = string_copy(str);

        assert(string_eq(str, str_copy) == true);

        string_free(str);
        string_free(str_copy);
    }
}

void test_string_concat(void) {
    char *first[] = {"", "first", "some another"};
    char *second[] = {"", "second", " test"};
    char *res[] = {"", "firstsecond", "some another test"};
    for (size_t idx = 0; idx < 3; ++idx) {
        string_t *first_str = new_string_from_bytes(first[idx]);
        string_t *second_str = new_string_from_bytes(second[idx]);
        string_t *res_str = new_string_from_bytes(res[idx]);
        string_t *concat_str = string_concat(first_str, second_str);

        assert(string_eq(concat_str, res_str) == true);

        string_free(first_str);
        string_free(second_str);
        string_free(res_str);
        string_free(concat_str);
    }
}

void test_string_strip(void) {
    char *bytes[] = {"", "vfv\n\n", "  test\t", " some another test  "};
    char *stripped_bytes[] = {"", "vfv", "test", "some another test"};

    for (size_t idx = 0; idx < 4; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        string_t *stripped_str =  string_strip(str);
        string_t *exp_stripped_str = new_string_from_bytes(stripped_bytes[idx]);

        assert(string_eq(stripped_str, exp_stripped_str) == true);

        string_free(str);
        string_free(stripped_str);
        string_free(exp_stripped_str);
    }
}

void test_string_substr(void) {
    char *bytes[] = {"", "vfv\n\n", "test string", " some another test  "};
    size_t substr_start_pos[] = {0, 1, 5, 0};
    size_t substr_len[] = {0, 2, 3, 5};
    char *substr_bytes[] = {"", "fv", "str", " some"};

    for (size_t idx = 0; idx < 4; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        string_t *substr_str = string_substr(str, substr_start_pos[idx], substr_len[idx]);
        string_t *expected_substr_str = new_string_from_bytes(substr_bytes[idx]);

        assert(string_eq(substr_str, expected_substr_str) == true);

        string_free(str);
        string_free(substr_str);
        string_free(expected_substr_str);
    }
}

void test_string_startswith(void) {
    char *bytes[] = {"", "vfv\n\n", "test string", " some another test  ", "1234"};
    char *startswith_bytes[] = {"", "vfv", "test string", " some", "2"};
    bool_t res[] = {true, true, true, true, false};

    for (size_t idx = 0; idx < 5; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        assert(string_startswith(str, startswith_bytes[idx]) == res[idx]);
        string_free(str);
    }
}

void test_string_endswith(void) {
    char *bytes[] = {"", "vfv\n\n", "test string", " some another test  ", "1234"};
    char *endswith_bytes[] = {"", "fv\n\n", "test string", "test  ", "2"};
    bool_t res[] = {true, true, true, true, false};

    for (size_t idx = 0; idx < 5; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        assert(string_endswith(str, endswith_bytes[idx]) == res[idx]);
        string_free(str);
    }
}

void test_string_find(void) {
    char *bytes[] = {"", "vfv\n\n", "test string", "test string", " some another test  "};
    char *chars[] = {"", "\n", "no", "", "another"};
    int expected_pos[] = {0, 3, -1, 0, 6};

    for (size_t idx = 0; idx < 5; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);
        assert(string_find(str, chars[idx]) == expected_pos[idx]);
        string_free(str);
    }
}

#ifndef _WIN32

void test_string_split(void) {
    char *bytes[] = {
            "",
            "1",
            "some",
            " some string 124!",
            "Some github account: vnkrtv"
    };
    size_t expected_arr_size[] = {
            1,
            1,
            1,
            4,
            4
    };

    string_t *first_arr[] = {
            new_string_from_bytes("")
    };
    string_t *second_arr[] = {
            new_string_from_bytes("1")
    };
    string_t *third_arr[] = {
            new_string_from_bytes("some")
    };
    string_t *fourth_arr[] = {
            new_string_from_bytes(""),
            new_string_from_bytes("some"),
            new_string_from_bytes("string"),
            new_string_from_bytes("124!")
    };
    string_t *fifth_arr[] = {
            new_string_from_bytes("Some"),
            new_string_from_bytes("github"),
            new_string_from_bytes("account:"),
            new_string_from_bytes("vnkrtv")
    };
    STRING_T_ARRAY arr_expected[] = {first_arr, second_arr, third_arr, fourth_arr, fifth_arr};

    for (size_t idx = 0; idx < 5; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);

        size_t arr_size = 0;
        STRING_T_ARRAY str_arr = string_split(str, &arr_size);
        STRING_T_ARRAY expected_str_arr = arr_expected[idx];

        assert(arr_size == expected_arr_size[idx]);
        for (size_t arr_idx = 0; arr_idx < expected_arr_size[idx]; ++arr_idx) {
            if (string_eq(str_arr[arr_idx], expected_str_arr[arr_idx]) != true) {
                LOG_STRING("str_arr[arr_idx]", str_arr[arr_idx]);
                LOG_STRING("expected_str_arr[arr_idx]", expected_str_arr[arr_idx]);
                assert(string_eq(str_arr[arr_idx], expected_str_arr[arr_idx]) == true);
            }
        }
        string_free(str);
        string_arr_free(str_arr, arr_size);
        free(str_arr);
    }

    string_arr_free(first_arr, 1);
    string_arr_free(second_arr, 1);
    string_arr_free(third_arr, 1);
    string_arr_free(fourth_arr, 4);
    string_arr_free(fifth_arr, 4);
}

void test_string_split_by(void) {
    const char split_by_chars[] = "W";
    char *bytes[] = {
            "",
            "1",
            "some",
            "WsomeWstringW124!",
            "SomeWgithubWaccount:Wvnkrtv"
    };
    size_t expected_arr_size[] = {
            1,
            1,
            1,
            4,
            4
    };

    string_t *first_arr[] = {
            new_string_from_bytes("")
    };
    string_t *second_arr[] = {
            new_string_from_bytes("1")
    };
    string_t *third_arr[] = {
            new_string_from_bytes("some")
    };
    string_t *fourth_arr[] = {
            new_string_from_bytes(""),
            new_string_from_bytes("some"),
            new_string_from_bytes("string"),
            new_string_from_bytes("124!")
    };
    string_t *fifth_arr[] = {
            new_string_from_bytes("Some"),
            new_string_from_bytes("github"),
            new_string_from_bytes("account:"),
            new_string_from_bytes("vnkrtv")
    };
    string_t **arr_expected[] = {first_arr, second_arr, third_arr, fourth_arr, fifth_arr};

    for (size_t idx = 0; idx < 5; ++idx) {
        string_t *str = new_string_from_bytes(bytes[idx]);

        size_t arr_size = 0;
        STRING_T_ARRAY str_arr = string_split_by(str, &arr_size, split_by_chars);
        STRING_T_ARRAY expected_str_arr = arr_expected[idx];

        assert(arr_size == expected_arr_size[idx]);
        for (size_t arr_idx = 0; arr_idx < expected_arr_size[idx]; ++arr_idx) {
            if (string_eq(str_arr[arr_idx], expected_str_arr[arr_idx]) != true) {
                LOG_STRING("str_arr[arr_idx]", str_arr[arr_idx]);
                LOG_STRING("expected_str_arr[arr_idx]", expected_str_arr[arr_idx]);
                assert(string_eq(str_arr[arr_idx], expected_str_arr[arr_idx]) == true);
            }
        }

        string_free(str);
        string_arr_free(str_arr, arr_size);
        free(str_arr);
    }

    string_arr_free(first_arr, 1);
    string_arr_free(second_arr, 1);
    string_arr_free(third_arr, 1);
    string_arr_free(fourth_arr, 4);
    string_arr_free(fifth_arr, 4);
}

#endif

void test_string_join_arr(void) {
    string_t *first_arr[] = {
            new_string_from_bytes("")
    };
    string_t *second_arr[] = {
            new_string_from_bytes("1")
    };
    string_t *third_arr[] = {
            new_string_from_bytes("some"),
            new_string_from_bytes("string")
    };
    string_t *fourth_arr[] = {
            new_string_from_bytes(""),
            new_string_from_bytes("some"),
            new_string_from_bytes("string")
    };
    string_t *fifth_arr[] = {
            new_string_from_bytes("some"),
            new_string_from_bytes("string")
    };
    STRING_T_ARRAY str_arr[] = {first_arr, second_arr, third_arr, fourth_arr, fifth_arr};
    size_t arr_size[] = {1, 1, 2, 3, 2};

    char *space_chars[] = {
            " ",
            " ",
            " ",
            "\n",
            "SOME"
    };
    string_t *expected_res_str[] = {
            new_string_from_bytes(""),
            new_string_from_bytes("1"),
            new_string_from_bytes("some string"),
            new_string_from_bytes("\nsome\nstring"),
            new_string_from_bytes("someSOMEstring")
    };


    for (size_t idx = 0; idx < 5; ++idx) {
        const STRING_T_ARRAY arr = (const STRING_T_ARRAY) str_arr[idx];
        string_t *res_str = string_join_arr(arr, arr_size[idx], space_chars[idx]);

        assert(string_eq(res_str, expected_res_str[idx]) == true);

        string_free(res_str);
    }

    string_arr_free(first_arr, 1);
    string_arr_free(second_arr, 1);
    string_arr_free(third_arr, 2);
    string_arr_free(fourth_arr, 3);
    string_arr_free(fifth_arr, 2);
    string_arr_free(expected_res_str, 5);
}

int main() {
    test_runner_t runner = new_runner();

    register_test(&runner, &test_new_string, "Test new_string");
    register_test(&runner, &test_new_string_from_bytes, "Test new_string_from_bytes");
    register_test(&runner, &test_string_len, "Test string_len");
    register_test(&runner, &test_string_bytes, "Test string_bytes");
    register_test(&runner, &test_string_eq, "Test string_eq");
    register_test(&runner, &test_string_copy, "Test string_copy");
    register_test(&runner, &test_string_concat, "Test string_concat");
    register_test(&runner, &test_string_substr, "Test string_substr");
    register_test(&runner, &test_string_startswith, "Test string_startswith");
    register_test(&runner, &test_string_endswith, "Test string_endswith");
    register_test(&runner, &test_string_find, "Test string_find");
    register_test(&runner, &test_string_strip, "Test string_strip");
#ifndef _WIN32
    register_test(&runner, &test_string_split, "Test string_split");
    register_test(&runner, &test_string_split_by, "Test string_split_by");
#endif
    register_test(&runner, &test_string_join_arr, "Test test_string_join_arr");

    run_tests(&runner);
    runner_free(&runner);

    return 0;
}