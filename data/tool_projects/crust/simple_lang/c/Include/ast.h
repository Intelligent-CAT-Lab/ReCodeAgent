#ifndef SIMPLE_LANG_AST_H
#define SIMPLE_LANG_AST_H

#include "token.h"

typedef struct ASTNode {
    TokenType type;           // 节点类型
    struct ASTNode* left;     // 左子节点
    struct ASTNode* right;    // 右子节点
    char* value;              // 节点的值（可选，根据节点类型）
} ASTNode;

// 创建新AST节点
ASTNode* new_ast_node(TokenType type, const char* value);

// 释放AST节点
void free_ast_node(ASTNode* node);

#endif //SIMPLE_LANG_AST_H
