#ifndef UTILS_H_HEADER_GUARD
#define UTILS_H_HEADER_GUARD

#include <string.h> /* strcpy, strlen, memset */
#include <stdlib.h> /* malloc */

#define ZERO_STRUCT(STRUCT) memset(STRUCT, 0, sizeof(*(STRUCT)))

char *strcpy_to_heap(const char *str);

#endif
