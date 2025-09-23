/*
 * Run all of the tests at the end of each C file in the lexer module
 */

#include "lex.h"
#include <testing/test_utils.h>

int test_lexer() {
    testing_module_setup();

    test_ttype_name();
    test_ttype_from_string();
    test_ttype_many_chars();
    test_ttype_one_char();

    testing_module_cleanup();
    return 0;
}
