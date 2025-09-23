#include "../src/common.h"
#include "tests.h"
#include <string.h>
#include <assert.h>
#include "../src/reducer.h"
#include "../hash-table/hash_table.h"
#include "../src/parser.h"


void test_expand_definitions() {
  HashTable *table = createHashTable();
  AstNode *variable = create_variable("test", NULL);
  variable->type = DEFINITION;
  AstNode *definition = create_variable("testing", "Nat");
  insert(table, "test", definition);
  expand_definitions(table, variable);
  assert(variable->type == VAR);
  assert(strcmp(variable->node.variable->name, "testing") == 0);
  assert(strcmp(variable->node.variable->type, "Nat") == 0);
  destroyHashTable(table);
}

void test_replace() {
  AstNode *variable = create_variable("test", NULL);
  AstNode *lambda_expr = create_lambda("test", variable, "Nat");
  char *new_name = "replaced";

  replace(lambda_expr, "test", new_name);

  assert(strcmp(lambda_expr->node.lambda_expr->parameter, new_name) == 0);
  assert(strcmp(variable->node.variable->name, new_name) == 0);
}

void test_reduce_ast() {
  HashTable *table = createHashTable();
  insert(table, "Nat", NULL);
  AstNode *lambda_body = create_variable("x", NULL);
  AstNode *y_var = create_variable("y", "Nat");
  AstNode *lambda_expr = create_lambda("x", lambda_body, "Nat");

  AstNode *parsed = create_application(lambda_expr, y_var);

  AstNode *reduced = reduce_ast(table, parsed);
  assert(reduced->type == VAR);
  assert(strcmp(reduced->node.variable->name, "y") == 0);
  assert(strcmp(reduced->node.variable->type, "Nat") == 0);
}

void test_deepcopy() {
  AstNode *lambda_body = create_variable("x", NULL);
  AstNode *lambda_expr = create_lambda("x", lambda_body, "Nat");

  AstNode *deepcopied = deepcopy(lambda_expr);
  assert(deepcopied->type == LAMBDA_EXPR);
  assert(strcmp(deepcopied->node.lambda_expr->parameter, "x"));
  assert(strcmp(deepcopied->node.lambda_expr->type, "Nat"));
  assert(deepcopied->node.lambda_expr->body->type == VAR);
}


int main() {
  printf("Test reducer ...\t");
  test_expand_definitions();
  test_replace();
  test_reduce_ast();
  print_green("PASSED\n");
  return 0;
}
