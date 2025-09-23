#include "vm.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>


Instruction *
new_instruction(OpCode opcode, const char* operand)
{
    Instruction * instruction = (Instruction *) malloc(sizeof(Instruction));
    instruction -> opcode = opcode;
    if (operand) {
        instruction -> operand = strdup(operand);
    }
    else {
        instruction -> operand = NULL;
    }
    return instruction;
}


void
free_instruction(Instruction * instruction)
{
//    if (instruction -> operand) {
//        free(instruction -> operand);
//    }
    free(instruction);
}

Frame *
init_frame()
{
    Frame * frame = (Frame *) malloc(sizeof(Frame));
    frame -> sp = 0;
    frame -> var_count = 0;
    return frame;
}

void
free_frame(Frame * frame)
{
    free(frame);
}

void
eval(Frame * frame, Instruction* instructions, int instr_count)
{
//    int stack[100];
//    int sp = -1;
//    int variables[100];
//    char* var_names[100];
//    int var_count = 0;

    for (int pc = 0; pc < instr_count; pc++) {
        Instruction instr = instructions[pc];
        switch (instr.opcode) {
            case LOAD_CONST:
                frame->stack[++frame->sp] = atoi(instr.operand);
                //frame->stack[++frame->sp] = strtoll(instr.operand);
                break;
            case LOAD_NAME:
                for (int i = 0; i < frame->var_count; i++) {
                    if (strcmp(frame->var_names[i], instr.operand) == 0) {
                        frame->stack[++frame->sp] = frame->variables[i];
                        break;
                    }
                }
                break;
            case STORE_NAME:
                for (int i = 0; i < frame->var_count; i++) {
                    if (strcmp(frame->var_names[i], instr.operand) == 0) {
                        frame->variables[i] = frame->stack[frame->sp--];
                        break;
                    }
                }
                if (frame->var_count == 0 || strcmp(frame->var_names[frame->var_count - 1], instr.operand) != 0) {
                    frame->var_names[frame->var_count] = strdup(instr.operand);
                    frame->variables[frame->var_count] = frame->stack[frame->sp--];
                    frame->var_count++;
                }
                break;
            case BINARY_ADD:
                frame->stack[frame->sp - 1] = frame->stack[frame->sp - 1] + frame->stack[frame->sp];
                frame->sp--;
                break;
            case BINARY_SUB:
                frame->stack[frame->sp - 1] = frame->stack[frame->sp - 1] - frame->stack[frame->sp];
                frame->sp--;
                break;
            case STK_DIS:
                printf("%d\n", frame->stack[frame->sp]);
                break;
        }
    }

//    for (int i = 0; i < frame->var_count; i++) {
//        printf("%s = %d\n", frame->var_names[i], frame->variables[i]);
//    }

//    for (int i = 1; i < instr_count; i++) {
//        free_instruction(&instructions[i]);
//    }
    free(instructions);
}