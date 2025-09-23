#include "settings.h"
#include "compiler.h"

#include <stdio.h>
#include <string.h>
#include <stdlib.h>




void
emit(Instruction * instructions, int * count, OpCode opcode, const char* operand)
{
    instructions[*count] = *new_instruction(opcode, operand);
    (*count)++;
}


void
compile_expression(ASTNode * node, Instruction * instructions, int * count)
{
    /*
     * This function will compile an expression.
     * */
    if (node -> type == TOKEN_INT) {
        emit(instructions, count, LOAD_CONST, node -> value);
    }
    else if (node -> type == TOKEN_IDENTIFIER) {
        emit(instructions, count, LOAD_NAME, node -> value);
    }
    else if (node -> type == TOKEN_PLUS) {
        compile_expression(node -> left, instructions, count);
        compile_expression(node -> right, instructions, count);
        emit(instructions, count, BINARY_ADD, NULL);
    }
    else if (node -> type == TOKEN_MINUS) {
        compile_expression(node -> left, instructions, count);
        compile_expression(node -> right, instructions, count);
        emit(instructions, count, BINARY_SUB, NULL);
    }
}


Instruction*
compile_statement(ASTNode * ast, int *instr_count)
{
    /*
     * This function will compile a statement (an AST) to bytecode.
     * */
    Instruction * instructions = (Instruction *)malloc(sizeof(Instruction)* MAX_INSTR_IN_STATEMENT);
    *instr_count = 0;

    if (ast -> type == TOKEN_LET || ast -> type == TOKEN_ASSIGN) {
        compile_expression(ast -> left, instructions, instr_count);
        emit(instructions, instr_count, STORE_NAME, ast -> value);
    }
    else if (ast -> type == TOKEN_DIS) {
        compile_expression(ast -> left, instructions, instr_count);
        emit(instructions, instr_count, STK_DIS, ast -> value);
    }

    return instructions;
}

Instruction *
compile_asts(ASTNode** asts, int* total_instr_count)
{
    /*
     * This function will compile a group of AST to bytecode.
     * */
    *total_instr_count = 0;
    Instruction ** instr_of_all = (Instruction **) malloc(sizeof(Instruction)
                                                          * MAX_INSTR_IN_STATEMENT * MAX_STATEMENT);
    int * instr_count_of_all = (int *) malloc(sizeof(int) * MAX_STATEMENT);

    for (int i=0; asts[i] != NULL; i++) {
        instr_of_all[i] = compile_statement(asts[i], &instr_count_of_all[i]);
    }

    for (int i=0; asts[i] != NULL; i++) {
        *total_instr_count += instr_count_of_all[i];
    }

    // concat all instructions
    Instruction * all_instr = (Instruction *) malloc(sizeof(Instruction) * *total_instr_count);
    memcpy(all_instr, instr_of_all[0], sizeof(Instruction) * instr_count_of_all[0]);
    int copied_count = instr_count_of_all[0];
    for (int i=1; asts[i] != NULL ; i++) {
        memcpy(&all_instr[copied_count], instr_of_all[i],
               sizeof(Instruction) * instr_count_of_all[i]);
        copied_count += instr_count_of_all[i];
    }

    return all_instr;
}

Instruction *
compile(const char* source, int* instr_count)
{
    Token* tokens = tokenize(source);
    ASTNode** ast_nodes = parse(tokens);
    Instruction * instructions =  compile_asts(ast_nodes, instr_count);

    return  instructions;
}