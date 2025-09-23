//
// Created by lixiaobai on 24-7-5.
//

#include "ast.h"
#include <stdlib.h>
#include <string.h>

ASTNode *
new_ast_node(TokenType type, const char* value)
{
    ASTNode* node = (ASTNode*)malloc(sizeof(ASTNode));
    node->type = type;
    node->left = NULL;
    node->right = NULL;
    if (value) {
        node->value = strdup(value);
    } else {
        node->value = NULL;
    }
    return node;
}

void
free_ast_node(ASTNode* node)
{
    if (node->left) free_ast_node(node->left);
    if (node->right) free_ast_node(node->right);
    if (node->value) free(node->value);
    free(node);
}

