#ifndef SIMPLE_LANG_MISC_H
#define SIMPLE_LANG_MISC_H

#include "stdio.h"
#include "vm.h"
#include "ast.h"
#include "settings.h"
#include <string.h>

void show_opcode(Instruction * instruction, int instr_count);
void show_tokens(Token * tokens);
void print_asts(ASTNode ** asts);
char * read_file(const char * source_path);

#endif //SIMPLE_LANG_MISC_H
