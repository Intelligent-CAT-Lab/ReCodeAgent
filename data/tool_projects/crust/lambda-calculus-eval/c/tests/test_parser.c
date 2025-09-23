#include "../src/common.h"
#include "tests.h"
#include <string.h>
#include <assert.h>
#include "../src/parser.h"
#include "../hash-table/hash_table.h"

void test_parse_token() {
  tokens_t l_paren = parse_token('(');
  tokens_t r_paren = parse_token(')');
  tokens_t lambda = parse_token('@');
  tokens_t dot = parse_token('.');
  tokens_t variable = parse_token('t');
  tokens_t whitespace = parse_token(' ');
  tokens_t new_line = parse_token('\n');
  tokens_t eq = parse_token('=');
  tokens_t quote = parse_token('"');
  tokens_t colon = parse_token(':');
  tokens_t error_token = parse_token('$');

  assert(l_paren == L_PAREN);
  assert(r_paren == R_PAREN);
  assert(lambda == LAMBDA);
  assert(dot == DOT);
  assert(variable == VARIABLE);
  assert(whitespace == WHITESPACE);
  assert(new_line == NEWLINE);
  assert(eq == EQ);
  assert(quote == QUOTE);
  assert(colon == COLON);
  assert(error_token == ERROR);
}

void test_is_variable() {
  char var = 't';
  char not_var = '*';
  assert(is_variable(var) == true);
  assert(is_variable(not_var) == false);
}

void test_create_variable() {
  char *name = "test";
  char *type = "Nat";

  AstNode *var = create_variable(name, type);
  assert(var->type == VAR);
  assert(var->node.variable->name == name);
  assert(var->node.variable->type == type);
  free(var->node.variable);
  free(var);
}

void test_create_application() {
  AstNode *var = create_application(NULL, NULL);
  assert(var->type == APPLICATION);
  assert(var->node.application->function == NULL);
  assert(var->node.application->argument == NULL);
  free(var->node.application);
  free(var);
}

void test_create_lambda() {
  char *param = "test";
  char *type = "nat";
  AstNode *var = create_lambda(param, NULL, type);
  assert(var->type == LAMBDA_EXPR);
  assert(var->node.lambda_expr->parameter == param);
  assert(var->node.lambda_expr->type == type);
  assert(var->node.lambda_expr->body == NULL);
  free(var->node.lambda_expr);
  free(var);
}

void test_alpha_convert() {
  char *old = "x";
  char *old_two = "y";
  char *expected = "x_1";
  char *expected_two = "y_2";

  char *new = alpha_convert(old);
  char *new_two = alpha_convert(old_two);

  assert(strcmp(new, expected) == 0);
  assert(strcmp(new_two, expected_two) == 0);
}

void test_is_used() {
  HashTable *table = createHashTable();
  char *key = "test";
  insert(table, key, NULL);
  bool used = is_used(table, key);
  assert(used == true);
  used = is_used(table, "wrong");
  assert(used == false);
}

void test_is_uppercase() {
  char upper = 'A';
  char lower = 'c';
  assert(is_uppercase(upper) == true);
  assert(is_uppercase(lower) == false);
}

int main() {
    printf("Test parser ...\t");
    test_parse_token();
    test_is_variable();
    test_create_variable();
    test_create_application();
    test_create_lambda();
    test_alpha_convert();
    test_is_used();
    print_green("PASSED\n");
    return 0;
}
