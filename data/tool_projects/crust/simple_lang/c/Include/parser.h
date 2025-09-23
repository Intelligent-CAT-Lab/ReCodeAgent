#ifndef SIMPLE_LANG_PARSER_H
#define SIMPLE_LANG_PARSER_H


#include "token.h"
#include "ast.h"

// 解析函数，返回AST
ASTNode** parse(Token* tokens);


#endif //SIMPLE_LANG_PARSER_H
