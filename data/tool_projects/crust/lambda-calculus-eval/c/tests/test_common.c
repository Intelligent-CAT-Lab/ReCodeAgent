#include <assert.h>
#include <string.h>
#include "../src/common.h"
#include "tests.h"

void test_format() {
    char* result = format("Hello, %s! You are %d years old.", "John", 30);
    assert(strcmp(result, "Hello, John! You are 30 years old.") == 0);
    free(result);
}


int main() {
    printf("Test common ...\t");
    test_format();
    print_green("PASSED\n");
    return 0;
}
