#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "../src/parser.h"
#include "../src/env.h"

#define TEST_DIR "tests/"
#define NUM_TESTS 9

const char *test_files[NUM_TESTS] = {
    "comments.eml",
    "comparisons.eml",
    "count_to_10.eml",
    "error.eml",
    "hello_world.eml",
    "if.eml",
    "math.eml",
    "negative_nums.eml",
    "runtime_error.eml"
};
program_t test_parse(const char *path) {
	parser_t       *p = parser_new(DEFAULT_PROGRAM_CAP);
	parser_result_t result;

	if (parser_load_file(p, path) != 0) {
		fprintf(stderr, "Error: Failed to open file '%s'\n", path);
		exit(EXIT_FAILURE);
	}

	result = parser_parse(p);
	if (result.err != PARSER_OK) {
		fprintf(stderr, "Error at %s:%zu:%zu: %s\n",
		        result.path, result.row, result.col, parser_err_to_cstr(result.err));
		exit(EXIT_FAILURE);
	}

	parser_destroy(p);
	return result.prog;
}

void run_test(const char *filename) {
    char path[256];
    snprintf(path, sizeof(path), "%s%s", TEST_DIR, filename);

    printf("Running test: %s\n", filename);
    
    program_t prog = test_parse(path);
    env_t *e = env_new(DEFAULT_STACK_CAP, DEFAULT_POPPED_CAP);

    runtime_result_t result = env_run(e, &prog);

    if (strcmp(filename, "runtime_error.eml") == 0) {
        assert(result.err != RUNTIME_OK);
        printf("✔ Expected runtime error in %s\n", filename);
    } else {
        assert(result.err == RUNTIME_OK);
        printf("✔ Successfully ran %s\n", filename);
    }

    env_destroy(e);
    program_destroy(&prog);
}

int main() {
    for (size_t i = 0; i < NUM_TESTS; ++i) {
        run_test(test_files[i]);
    }

    printf("All tests completed successfully.\n");
    return EXIT_SUCCESS;
}