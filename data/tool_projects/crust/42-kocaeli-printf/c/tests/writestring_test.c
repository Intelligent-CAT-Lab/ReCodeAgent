#include "../src/ft_printf.h"
#include <unistd.h>
#include <assert.h>
#include <stdio.h>


int main() {
    int len = 0;

    // Test writechar function
    len = 0;
    assert(writechar('A', &len) == 1);  // Test with a character 'A'
    assert(len == 1);                   // Length should be 1 after writing a single character

    len = 0;
    assert(writechar('B', &len) == 1);  // Test with a character 'B'
    assert(len == 1);                   // Length should be 1 after writing a single character

    len = 0;
    assert(writechar('z', &len) == 1);  // Test with a character 'z'
    assert(len == 1);                   // Length should be 1 after writing a single character

    // Test writestring function
    len = 0;
    assert(writestring("Hello", &len) == 1);  // Test with a simple string "Hello"
    assert(len == 5);                        // Length should be 5 after writing "Hello"

    len = 0;
    assert(writestring("World!", &len) == 1); // Test with a string "World!"
    assert(len == 6);                        // Length should be 6 after writing "World!"

    len = 0;
    assert(writestring(NULL, &len) == 1);     // Test with NULL (should write "(null)")
    assert(len == 6);                        // Length should be 6 after writing "(null)"

    len = 0;
    assert(writestring("1234567890", &len) == 1);  // Test with a longer string "1234567890"
    assert(len == 10);                           // Length should be 10 after writing "1234567890"

    printf("All tests passed successfully.\n");
    return 0;
}