#ifndef COMMON_H
#define COMMON_H
#include <stdbool.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#define HANDLE_NULL(ptr)                                                       \
  do {                                                                         \
    if ((ptr) == NULL) {                                                       \
      fprintf(stderr, "ERROR: Null pointer encountered at %s:%d in %s\n",      \
              __FILE__, __LINE__, __func__);                                   \
      abort();                                                                 \
    }                                                                          \
  } while (0)

typedef enum {
  L_PAREN,
  R_PAREN,
  LAMBDA,
  DOT,
  VARIABLE,
  ERROR,
  WHITESPACE,
  NEWLINE,
  EQ,
  QUOTE,
  COLON,
} tokens_t;

typedef struct AstNode AstNode;

typedef enum {
  LAMBDA_EXPR,
  APPLICATION,
  VAR,
  DEFINITION,
} AstNodeType;

typedef struct {
  char *parameter;
  char *type;
  AstNode *body;
} LambdaExpression;

typedef struct {
  AstNode *function;
  AstNode *argument;
} Application;

typedef struct {
  char *name;
  char *type;
} Variable;

typedef union {
  LambdaExpression *lambda_expr;
  Application *application;
  Variable *variable;
} AstNodeUnion;

struct AstNode {
  AstNodeType type;
  AstNodeUnion node;
};

void set_verbose(bool verbose);

void print_ast_verbose(AstNode *n);

void print_verbose(const char *format, ...);

void error(const char *msg, const char *file, int line, const char *func);

char* format(const char *fmt, ...);

void append_to_buffer(char **buffer, size_t *buffer_size, size_t *length, const char *str);

void append_ast_to_buffer(char **buffer, size_t *buffer_size, size_t *length, AstNode *node);

char *ast_to_string(AstNode *node);

#endif
