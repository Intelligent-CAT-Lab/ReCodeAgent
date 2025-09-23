//
// Created by lixiaobai on 24-7-5.
//

#include "lexer.h"
#include "parser.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void test_parse() {
    const char* source = "let x = 5 + 3 - 2; let y = x + 1; dis x + 1;";
    Token* tokens = tokenize(source);
    ASTNode** ast_nodes = parse(tokens);

    assert(ast_nodes[0]->type == TOKEN_LET);
    assert(ast_nodes[0]->left->type == TOKEN_MINUS);
    assert(ast_nodes[0]->left->left->type == TOKEN_PLUS);
    assert(ast_nodes[0]->left->left->left->type == TOKEN_INT);
    assert(ast_nodes[0]->left->left->right->type == TOKEN_INT);
    assert(ast_nodes[0]->left->right->type == TOKEN_INT);

    assert(ast_nodes[1]->type == TOKEN_LET);
    assert(ast_nodes[1]->left->type == TOKEN_PLUS);
    assert(ast_nodes[1]->left->left->type == TOKEN_IDENTIFIER);
    assert(ast_nodes[1]->left->right->type == TOKEN_INT);

    assert(ast_nodes[2]->type == TOKEN_DIS);
    assert(ast_nodes[2]->left->type == TOKEN_PLUS);
    assert(ast_nodes[2]->left->left->type == TOKEN_IDENTIFIER);
    assert(ast_nodes[2]->left->right->type == TOKEN_INT);

    for (int i = 0; ast_nodes[i] != NULL; i++) {
        free_ast_node(ast_nodes[i]);
    }
    free(ast_nodes);
    free(tokens);
}

int main() {
    test_parse();
    printf("All Tests passed!\n");
    return 0;
}
