#ifndef PARSER_H
#define PARSER_H

#include "common.h"
#include "../hash-table/hash_table.h"
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

tokens_t parse_token(char token);

void p_print_token(tokens_t token);

void p_print_astNode_type(AstNode *n);

void print_ast(AstNode *node);

bool is_variable(char token);

char peek(FILE *in);

void peek_print(FILE *in, int n);

void consume(tokens_t t, FILE *in, char *expected);

AstNode *create_variable(char *name, char *type);

AstNode *create_application(AstNode *function, AstNode *argument);

AstNode *create_lambda(char *variable, AstNode *body, char *type);

char *alpha_convert(char *old);

bool is_used(HashTable *table, char *variable);

void parse_space_chars(FILE *in);

AstNode *parse_lambda(HashTable *table, FILE *in);

AstNode *parse_expression(HashTable *table, FILE *in);

void parse_import(HashTable *table, FILE *in);

void parse_definition(HashTable *table, FILE *in);

bool is_uppercase(char c);

void parse_type_definition(HashTable *types_table, FILE *in);

char *parse_type(HashTable *types_table, FILE *in);

char *parse_variable(FILE *in);

void expect(char *expected, char received);

void free_ast(AstNode *node);


#endif
