#include "../nand.c"
#include <assert.h>
#include <stdio.h>

bool check_add() {
    for(u4 a = 0; a < U4_MAX; ++a) {
        for(u4 b = 0; b < U4_MAX; ++b) {
            u4 my = add_u4(a, b);
            u4 h  = (a+b) & 0b1111;
            if(my != h) {
                fprintf(stderr, "ERROR: Add failed with %d+%d:\n", a, b);
                fprintf(stderr, "  My  : %d\n", my);
                fprintf(stderr, "  Real: %d\n", my);
                return false;
            }
        }
    }
    fprintf(stderr, "Add works correctly\n");
    return true;
}
bool check_sub() {
    for(u4 a = 0; a < U4_MAX; ++a) {
        for(u4 b = 0; b < U4_MAX; ++b) {
            u4 my = sub_u4(a, b);
            u4 h  = (a-b) & 0b1111;
            if(my != h) {
                fprintf(stderr, "ERROR: Sub failed with %d-%d:\n", a, b);
                fprintf(stderr, "  My  : %d\n", my);
                fprintf(stderr, "  Real: %d\n", h);
                return false;
            }
        }
    }
    fprintf(stderr, "Sub works correctly\n");
    return true;
}

int main(void) {
    assert(check_add());
    assert(check_sub());
    return 0;
}