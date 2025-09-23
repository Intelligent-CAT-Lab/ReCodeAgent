#include "tests.h"
#include <stdarg.h>

void print_green(const char *format, ...) {
    va_list args;
    va_start(args, format);

    printf(ANSI_COLOR_GREEN);
    vprintf(format, args);
    printf(ANSI_COLOR_RESET);

    va_end(args);
}

void print_red(const char *format, ...) {
    va_list args;
    va_start(args, format);

    printf(ANSI_COLOR_RED);
    vprintf(format, args);
    printf(ANSI_COLOR_RESET);

    va_end(args);
}
