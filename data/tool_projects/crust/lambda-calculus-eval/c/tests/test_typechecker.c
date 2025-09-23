#include "../src/common.h"
#include "tests.h"
#include <string.h>
#include <assert.h>
#include "../src/typechecker.h"
#include "../hash-table/hash_table.h"
#include "../src/io.h"
#include "../src/parser.h"

void test_type_equal() {
  Type *a = create_type("Nat", NULL, NULL);
  Type *b = create_type("Nat", NULL, NULL);
  Type *c = create_type("Bool", NULL, NULL);

  assert(type_equal(a, b) == true);
  assert(type_equal(a, c) == false);
}

void test_get_type_from_expr() {
  AstNode *n = create_variable("test", "Nat");
  AstNode *n1 = create_lambda("test", NULL, "Bool");

  char *n_type = get_type_from_expr(n);
  char *n1_type = get_type_from_expr(n1);

  assert(strcmp(n_type, "Nat") == 0);
  assert(strcmp(n1_type, "Bool") == 0);
}

void test_expr_type_equal() {
  AstNode *n = create_variable("test", "Nat");
  Type *a = create_type("Nat", NULL, n);
  assert(expr_type_equal(a, n) == true);
}

void test_typecheck() {
  AstNode *lambda_body = create_variable("x", NULL);
  AstNode *y_var = create_variable("y", "Nat");
  AstNode *lambda_expr = create_lambda("x", lambda_body, "Nat");
  AstNode *parsed = create_application(lambda_expr, y_var);
  typecheck(parsed, NULL);
  // if it gets here, it typechecked
}

int main() {
  printf("Test typechecker ...\t");
  test_type_equal();
  test_get_type_from_expr();
  test_expr_type_equal();
  test_typecheck();
  print_green("PASSED\n");
  return 0;
}
