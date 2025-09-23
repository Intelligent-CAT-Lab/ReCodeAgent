#include <stdio.h>
#include <stdbool.h>
static bool nand(bool a, bool b) {
    return !(a && b);
}
static bool not(bool a) {
    return nand(a, true);
}
static bool or(bool a, bool b) {
    return 
        nand(
           not(a),
           not(b)
        );
}
static bool and(bool a, bool b) {
    return 
        not(
            nand(a, b)
        );
}
static bool xor(bool a, bool b) {
    return 
        or(
           and(a     , not(b)),
           and(not(a), b     )
        );
}
static bool add_bit(bool a, bool b, bool carry, bool* carry_result) {
    *carry_result = and(
                        or(a, b),
                        or(
                           and(a, b),
                           carry
                        )
                    );
    return xor(xor(a, b), carry);
}
static bool half_sub(bool a, bool b, bool* carry_result) {
    *carry_result = and(not(a), b);
    return xor(a, b);
}
static bool sub_bit(bool a, bool b, bool carry, bool* carry_result) {
    bool b1;
    bool b2;
    bool result = half_sub(half_sub(a, b, &b1), carry, &b2);
    *carry_result = or(b1, b2);
    return result;
}
const char* bll(bool x) {
    return x ? "true" : "false";
}
void print_add_bit(bool a, bool b, bool carry) {
    printf("a      : %s\n", bll(a));
    printf("b      : %s\n", bll(b));
    printf("carry  : %s\n", bll(carry));
    bool res_carry;
    printf("result :\n");
    printf("  bit  : %s\n", bll(add_bit(a, b, carry, &res_carry)));
    printf("  carry: %s\n", bll(res_carry));
}
#include <stdint.h>
typedef uint8_t u4;
#define U4_MAX 0b1111
u4 add_u4(u4 a, u4 b) {
    bool carry = false;
    u4 result=0;
    result |= ((u4)add_bit((a >> 0) & 0b1, (b >> 0) & 0b1, carry, &carry)) << 0;
    result |= ((u4)add_bit((a >> 1) & 0b1, (b >> 1) & 0b1, carry, &carry)) << 1;
    result |= ((u4)add_bit((a >> 2) & 0b1, (b >> 2) & 0b1, carry, &carry)) << 2;
    result |= ((u4)add_bit((a >> 3) & 0b1, (b >> 3) & 0b1, carry, &carry)) << 3;
    return result;
}
u4 sub_u4(u4 a, u4 b) {
    bool carry = false;
    u4 result=0;
    result |= ((u4)sub_bit((a >> 0) & 0b1, (b >> 0) & 0b1, carry, &carry)) << 0;
    result |= ((u4)sub_bit((a >> 1) & 0b1, (b >> 1) & 0b1, carry, &carry)) << 1;
    result |= ((u4)sub_bit((a >> 2) & 0b1, (b >> 2) & 0b1, carry, &carry)) << 2;
    result |= ((u4)sub_bit((a >> 3) & 0b1, (b >> 3) & 0b1, carry, &carry)) << 3;
    return result;
}

    
