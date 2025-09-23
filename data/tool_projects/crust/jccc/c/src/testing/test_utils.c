#include <stdio.h>
#include <util/out.h>

void testing_setup_internal(const char *func_name) {
    printf("Running tests from ");
    fflush(stdout);
    PRINT_WITH_COLOR(CODE_GREEN, "\"%s\"", func_name);
    printf(" ...\n");
}

void testing_cleanup_internal(const char *func_name) {
    printf("Concluded tests from ");
    fflush(stdout);
    PRINT_WITH_COLOR(CODE_GREEN, "\"%s\"", func_name);
    printf("\n");
}

void testing_single_test_internal(const char *func_name) {
    printf("Running ");
    fflush(stdout);
    PRINT_WITH_COLOR(CODE_GREEN, "\"%s\"", func_name);
    printf("\n");
}
