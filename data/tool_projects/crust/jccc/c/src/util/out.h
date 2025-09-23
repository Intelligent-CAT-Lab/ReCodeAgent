/**
 * This provides facilities for properly-formatted output. This is just taken
 * from yfc's output facilities.
 */

#pragma once

#include <stdio.h> // stderr

// All of the color codes.
#define CODE_RED 31
#define CODE_YELLOW 33
#define CODE_GREEN 32
#define CODE_BLUE 34
#define CODE_MAGENTA 35
#define CODE_CYAN 36
#define CODE_WHITE 37

// Print things with colors.
#define OUTPUT_STREAM stderr
#define SET_COLOR(color) fprintf(OUTPUT_STREAM, "\033[%dm", color)
#define RESET_COLOR(color) fprintf(OUTPUT_STREAM, "\033[0m")

#define PRINT_WITH_COLOR(color, ...) do { \
    SET_COLOR(color); \
    fprintf(OUTPUT_STREAM, __VA_ARGS__); \
    RESET_COLOR(color); \
} while (0)

#define PRINT_ERROR(msg, ...) \
    PRINT_WITH_COLOR(CODE_RED, "Error: jccc: " msg "\n" ,##__VA_ARGS__)

#define PRINT_WARNING(msg, ...) \
    PRINT_WITH_COLOR(CODE_YELLOW, "Warning: jccc: " msg "\n" ,##__VA_ARGS__)

#define PRINT_DEBUG(msg, ...) \
    PRINT_WITH_COLOR(CODE_BLUE, "Debug: jccc: " msg "\n" ,##__VA_ARGS__)

#define PRINT_DEFAULT(msg, ...) \
    PRINT_WITH_COLOR(CODE_WHITE, "jccc: " msg "\n" ,##__VA_ARGS__)
