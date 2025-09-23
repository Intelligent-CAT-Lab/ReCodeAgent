//
// Created by lixiaobai on 24-7-5.
//

#include "settings.h"
#include "parser.h"
#include <stdio.h>
#include <stdlib.h>


static Token* tokens;
static int pos;

// look ahead token
Token*
lookahead() {
    return &tokens[pos];
}

// consume a token
Token*
consume() {
    return &tokens[pos++];
}

ASTNode*
parse_primary()
{
    Token* token = consume();
    if (token->type == TOKEN_INT) {
        return new_ast_node(TOKEN_INT, token->value);
    } else if (token->type == TOKEN_IDENTIFIER) {
        return new_ast_node(TOKEN_IDENTIFIER, token->value);
    } else {
        fprintf(stderr, "Unexpected token in primary expression: %s\n", token->value);
        exit(1);
    }
}

ASTNode*
parse_expression()
{
    ASTNode* left = parse_primary();
    Token* token = lookahead();
    while (token->type == TOKEN_PLUS || token->type == TOKEN_MINUS) {
        token = consume();
        ASTNode* right = parse_primary();
        ASTNode* node = new_ast_node(token->type, token->value);
        node->left = left;
        node->right = right;
        left = node;
        token = lookahead();
    }
    return left;
}

ASTNode*
parse_statement()
{
    Token* token = consume();
    if (token->type == TOKEN_LET) {
        Token* identifier = consume();
        if (identifier->type != TOKEN_IDENTIFIER) {
            fprintf(stderr, "Expected identifier after 'let'\n");
            exit(1);
        }
        if (consume()->type != TOKEN_ASSIGN) {
            fprintf(stderr, "Expected '=' after identifier\n");
            exit(1);
        }
        ASTNode* expr = parse_expression();
        if (consume()->type != TOKEN_SEMICOLON) {
            fprintf(stderr, "Expected ';' at end of statement\n");
            exit(1);
        }
        ASTNode* node = new_ast_node(TOKEN_LET, identifier->value);
        node->left = expr;
        return node;
    }
    else if (token->type == TOKEN_IDENTIFIER) {
        Token* identifier = token;
        if (consume()->type != TOKEN_ASSIGN) {
            fprintf(stderr, "Expected '=' after identifier\n");
            exit(1);
        }
        ASTNode* expr = parse_expression();
        if (consume()->type != TOKEN_SEMICOLON) {
            fprintf(stderr, "Expected ';' at end of statement\n");
            exit(1);
        }
        ASTNode* node = new_ast_node(TOKEN_ASSIGN, identifier->value);
        node->left = expr;
        return node;
    }
    else if (token->type == TOKEN_DIS){
        ASTNode * expr = parse_expression();
        ASTNode * node = new_ast_node(TOKEN_DIS, NULL);
        if (consume()->type != TOKEN_SEMICOLON) {
            fprintf(stderr, "Expected ';' at end of statement\n");
            exit(1);
        }
        node -> left = expr;
        return node;
    }
    else {
        fprintf(stderr, "Unexpected token: %s\n", token->value);
        exit(1);
    }
}

ASTNode**
parse(Token* tokens_array)
{
    tokens = tokens_array;
    pos = 0;


    ASTNode** ast_nodes = (ASTNode**)malloc(sizeof(ASTNode*) * MAX_STATEMENT);
    int count = 0;

    while (tokens[pos].type != TOKEN_EOF) {
        ast_nodes[count++] = parse_statement();
    }

    ast_nodes[count] = NULL; // Marking the end of an AST array with NULL

    return ast_nodes;
}

