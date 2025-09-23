#pragma once

/* Testing utility
 * Here is an example usage of the testing
 *
 * This is from "lexer/test_lexer.c"
 *
 * For each module of code, create a test_{module_name} file.
 * This file should include a test_{module_name} function that includes the
 * testing_setup and the testing_cleanup functions.

#include "lex.h"
#include <testing/test_utils.h>

int test_lexer() {
    testing_module_setup();

    test_ttype_from_string();

    testing_module_cleanup();
    return 0;
}

 * After this, include a call to this function in the "testing/main.c" file like
how it's done for test_lexer.

#include "lexer/test_lexer.h"

int main() {
        test_lexer();

        return 0;
}

 * Finally, here is what a test might look like. Make sure to include a call to testing_func_setup at the start.

int test_ttype_from_string() {
	testing_func_setup();

    tassert(ttype_from_string("1") == TT_LITERAL);
    tassert(ttype_from_string("1.2") == TT_LITERAL);

	// ...

    tassert(ttype_from_string(";") == TT_SEMI);

    return 0;
}
*/

#define testing_module_setup() testing_setup_internal(__func__)

#define testing_module_cleanup() testing_cleanup_internal(__func__)

#define testing_func_setup() testing_single_test_internal(__func__)

void testing_setup_internal(const char *func_name);

void testing_cleanup_internal(const char *func_name);

void testing_single_test_internal(const char *func_name);
