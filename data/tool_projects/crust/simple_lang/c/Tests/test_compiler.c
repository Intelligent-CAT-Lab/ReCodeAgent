#include "compiler.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void test_compiler() {
    const char* source = "let x = 5 + 3 - 2; dis x;";
    int instr_num = 0;
    Instruction * instr = compile(source, &instr_num);

    assert(instr[0].opcode == LOAD_CONST);
    assert(strcmp(instr[0].operand,"5") == 0);

    assert(instr[1].opcode == LOAD_CONST);
    assert(strcmp(instr[1].operand,"3") == 0);

    assert(instr[2].opcode == BINARY_ADD);

    assert(instr[3].opcode == LOAD_CONST);
    assert(strcmp(instr[3].operand, "2") == 0);

    assert(instr[4].opcode == BINARY_SUB);

    assert(instr[5].opcode == STORE_NAME);
    assert(strcmp(instr[5].operand,"x") == 0);

    assert(instr[6].opcode == LOAD_NAME);
    assert(strcmp(instr[6].operand,"x") == 0);

    assert(instr[7].opcode == STK_DIS);

    free_instruction(instr);
}

int main() {
    test_compiler();
    printf("All Tests passed!\n");
    return 0;
}
