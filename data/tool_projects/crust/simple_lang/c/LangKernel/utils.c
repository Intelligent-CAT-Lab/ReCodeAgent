//
// Created by lixiaobai on 24-7-6.
//

#include "utils.h"

#ifdef _WIN32
#include "utils.h"
#include <stdlib.h>
#include <string.h>

char *strndup(const char *s, size_t n) {
    char *result;
    size_t len = strlen(s);

    if (len > n) len = n;

    result = (char *)malloc(len + 1);
    if (!result) return 0;

    result[len] = '\0';
    return (char *)memcpy(result, s, len);
}
#endif