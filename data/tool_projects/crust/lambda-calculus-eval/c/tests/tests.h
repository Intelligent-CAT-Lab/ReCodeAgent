#ifndef TESTS_H
#define TESTS_H

#include <stdio.h>

// ANSI color codes
#define ANSI_COLOR_GREEN   "\x1b[32m"
#define ANSI_COLOR_RED     "\x1b[31m"
#define ANSI_COLOR_RESET   "\x1b[0m"

void print_green(const char *format, ...);

void print_red(const char *format, ...);

#endif
