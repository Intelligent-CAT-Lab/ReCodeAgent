#include "common.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdarg.h>
#include <string.h>
#include <stdlib.h>

static bool verbose_mode = false;

void set_verbose(bool verbose) {
  verbose_mode = verbose;
}

void print_ast_verbose(AstNode *n) {
  if (verbose_mode == false) {
    return;
  }
  char *lambda_ast = ast_to_string(n);
  printf("%s\n", lambda_ast);
}

void print_verbose(const char *format, ...) {
  if (verbose_mode == false) {
    return;
  }
  printf("\n");
  va_list args;
  va_start(args, format);
  vprintf(format, args);
  va_end(args);
}

void error(const char *msg, const char *file, int line, const char *func) {
  fprintf(stderr, "ERROR: %s at %s:%d in %s()\n", msg, file, line, func);
  exit(EXIT_FAILURE);
}

char* format(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);

    int size = vsnprintf(NULL, 0, fmt, args);
    va_end(args);

    if (size < 0) {
        return NULL;
    }

    char *str = (char *)malloc(size + 1);
    if (!str) {
        return NULL;
    }

    va_start(args, fmt);
    vsnprintf(str, size + 1, fmt, args);
    va_end(args);

    return str;
}

void append_to_buffer(char **buffer, size_t *buffer_size, size_t *length, const char *str) {
    size_t str_len = strlen(str);
    while (*length + str_len + 1 >= *buffer_size) {
        *buffer_size *= 2;
        *buffer = realloc(*buffer, *buffer_size);
    }
    strcpy(*buffer + *length, str);
    *length += str_len;
}

void append_ast_to_buffer(char **buffer, size_t *buffer_size, size_t *length, AstNode *node) {
    if (node == NULL) {
        return;
    }

    switch (node->type) {
        case LAMBDA_EXPR:
            append_to_buffer(buffer, buffer_size, length, "(@");
            append_to_buffer(buffer, buffer_size, length, node->node.lambda_expr->parameter);
            append_to_buffer(buffer, buffer_size, length, " : ");
            append_to_buffer(buffer, buffer_size, length, node->node.lambda_expr->type);
            append_to_buffer(buffer, buffer_size, length, " .");
            append_ast_to_buffer(buffer, buffer_size, length, node->node.lambda_expr->body);
            append_to_buffer(buffer, buffer_size, length, ") ");
            break;

        case APPLICATION:
            append_to_buffer(buffer, buffer_size, length, "(");
            append_ast_to_buffer(buffer, buffer_size, length, node->node.application->function);
            append_ast_to_buffer(buffer, buffer_size, length, node->node.application->argument);
            append_to_buffer(buffer, buffer_size, length, ") ");
            break;

        case VAR:
            append_to_buffer(buffer, buffer_size, length, "(");
            append_to_buffer(buffer, buffer_size, length, node->node.variable->name);
            if (node->node.variable->type != NULL) {
              append_to_buffer(buffer, buffer_size, length, " : ");
              append_to_buffer(buffer, buffer_size, length, node->node.variable->type);
            }
            append_to_buffer(buffer, buffer_size, length, ") ");
            break;

        case DEFINITION:
            append_to_buffer(buffer, buffer_size, length, "(");
            append_to_buffer(buffer, buffer_size, length, node->node.variable->name);
            append_to_buffer(buffer, buffer_size, length, ") ");
            break;

        default:
            append_to_buffer(buffer, buffer_size, length, "(UNKNOWN) ");
    }
}

char *ast_to_string(AstNode *node) {
    size_t buffer_size = 1024;
    char *buffer = malloc(buffer_size);
    buffer[0] = '\0';
    size_t length = 0;
    append_ast_to_buffer(&buffer, &buffer_size, &length, node);
    return buffer;
}
