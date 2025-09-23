#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "codegen.h"

#include <lexer/token.h>
#include <testing/tassert.h> // tassert

struct GenState {
    // Each bit corresponds with a registers 0-31 where the LSB is 0
    uint32_t registers_in_use;

    unsigned int rsp_offset;
} GEN_STATE;

void code_gen_init() {
    GEN_STATE.registers_in_use = 0;
    GEN_STATE.rsp_offset = 0;
}

enum Op ttype_to_op(TokenType t) {
    switch (t) {
    case TT_PLUS:
        return OP_ADD;
    case TT_MINUS:
        return OP_SUB;
	default:
		return OP_NOP;
    }
}

char *start_main() {
    static char start[256] = "\
global _start\n\
section .text\n\
\n\
_start:\n";

    return start;
}

char *end_main() {
    static char end[256] = "\
	mov rax, 60\
	mov rdi, 0\
	syscall";

    return end;
}

char *end_main_custom_return(int val) {
    char *end;
    end = (char *)malloc(256 * sizeof(char));
    sprintf(end, "	mov rax, 60\n	mov rdi, %d\n	syscall\n", val);

    return end;
}

static char *op_strs[4] = {"add", "sub", "mov"};

char *op_on_rax_with_rdi(enum Op op) {
    char *end;
    end = (char *)malloc(256 * sizeof(char));

    char *op_str = op_strs[op];

    sprintf(end, "	%s rax, rdi\n", op_str);

    return end;
}

char *start_func() {
    static char start[256] = "\
	sub rsp, 32\
	mov [rsp], r12\
	mov [rsp+8], r13\
	mov [rsp+16], r14\
	mov [rsp+24], r15";

    return start;
}

char *end_func() {
    static char end[256] = "\
	mov r12, [rsp]\
	mov r13, [rsp+8]\
	mov r14, [rsp+16]\
	mov r15, [rsp+24]\
	add rsp, 32";

    return end;
}

char *init_int_literal(int val) {
    GEN_STATE.rsp_offset += 8;

    char *init;
    init = (char *)malloc(256 * sizeof(char));
    sprintf(init, "	mov [rsp+%d], %d", GEN_STATE.rsp_offset, val);

    return init;
}

int test_init_int_literal() {
    testing_func_setup();
    code_gen_init();

    tassert(strcmp(init_int_literal(100), "	mov [rsp+8], 100") == 0);

    return 0;
}

int test_op_on_rax_with_rdi() {
    testing_func_setup();

    char *out = op_on_rax_with_rdi(OP_ADD);
    tassert(strcmp(out, "	add rax, rdi\n") == 0);

    char *out2 = op_on_rax_with_rdi(OP_MOV);
    tassert(strcmp(out2, "	mov rax, rdi\n") == 0);

    return 0;
}
