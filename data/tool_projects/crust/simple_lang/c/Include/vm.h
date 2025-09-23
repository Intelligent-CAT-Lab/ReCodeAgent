#ifndef SIMPLE_LANG_VM_H
#define SIMPLE_LANG_VM_H

#include "ast.h"
#include "settings.h"

typedef enum {
    LOAD_CONST,
    LOAD_NAME,
    STORE_NAME,
    BINARY_ADD,
    BINARY_SUB,
    STK_DIS,
} OpCode;

typedef struct {
    OpCode opcode;
    char* operand;
} Instruction;

typedef struct {
    int stack[STAKE_LENGTH];    // frame stack
    int sp;                     // stack ptr
    int variables[100];         // variable values
    char* var_names[100];       // variable names
    int var_count;              // variable count
} Frame;

void free_instruction(Instruction * instruction);
Instruction * new_instruction(OpCode opcode, const char* operand);
void eval(Frame * frame, Instruction* instructions, int instr_count);
Frame * init_frame();
void free_frame(Frame * frame);

#endif //SIMPLE_LANG_VM_H