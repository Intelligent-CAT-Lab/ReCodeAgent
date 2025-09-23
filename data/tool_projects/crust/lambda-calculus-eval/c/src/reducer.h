#ifndef REDUCER_H
#define REDUCER_H

#include "common.h"
#include "../hash-table/hash_table.h"
#define SIZE 122
#include "parser.h"
#include "config.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void set_reduction_order(reduction_order_t t);

void print_reduction_order(reduction_order_t t);

AstNode *reduce(HashTable *table, AstNode *n);

void expand_definitions(HashTable *table, AstNode *n);

void replace(AstNode *n, char *old, char *new_name);

AstNode *reduce_ast(HashTable *table, AstNode *n);

AstNode *substitute(AstNode *expression, char *variable, AstNode *replacement);

AstNode *deepcopy(AstNode *n);

AstNode *deepcopy_application(AstNode *function, AstNode *argument);

AstNode *deepcopy_lambda_expr(char *parameter, AstNode *body, char *type);

AstNode *deepcopy_var(char *name, char *type);

#endif
