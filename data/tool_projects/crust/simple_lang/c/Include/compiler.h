#ifndef SIMPLE_LANG_COMPILER_H
#define SIMPLE_LANG_COMPILER_H

#include "vm.h"
#include "lexer.h"
#include "parser.h"

Instruction* compile_asts(ASTNode** asts, int* instr_count);
Instruction* compile(const char* source, int* instr_count);

#endif //SIMPLE_LANG_COMPILER_H
