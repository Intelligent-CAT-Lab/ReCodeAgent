#include "../src/worsp.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TEST_ASSERT(expr)                                                      \
  do {                                                                         \
    if (!(expr)) {                                                             \
      fprintf(stderr, "Assertion failed: %s (%s:%d)\n", #expr, __FILE__,       \
              __LINE__);                                                       \
      exit(1);                                                                 \
    }                                                                          \
  } while (0)

#define RUN_TEST(test_func)                                                    \
  do {                                                                         \
    printf("Running test: %s\n", #test_func);                                  \
    test_func();                                                               \
    printf("Test passed.\n\n");                                                \
  } while (0)

void next_singleCharSymbol() {
  char *source = "a";
  struct ParseState state = (struct ParseState){NULL, 0};
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "a") == 0);
}

void next_multipleCharSymbol() {
  char *source = "aaaa";
  struct ParseState state = (struct ParseState){NULL, 0};
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "aaaa") == 0);
}

void next_parenAndDigit() {
  char *source = "(1)";
  struct ParseState state = (struct ParseState){NULL, 0};
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 1);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
}

void next_ifAndSet() {
  char *source = "(if (set a 1) (set b 2) (set c 3))";
  struct ParseState state = (struct ParseState){NULL, 0};
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "if") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "set") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "a") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 1);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "set") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "b") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 2);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "set") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "c") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 3);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
}

void next_string() {
  char *source = "\"hello\" () 1 \"foo\" \"bar\"";
  struct ParseState state = (struct ParseState){NULL, 0};
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_STRING);
  TEST_ASSERT(strcmp(state.token->str, "hello") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 1);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_STRING);
  TEST_ASSERT(strcmp(state.token->str, "foo") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_STRING);
  TEST_ASSERT(strcmp(state.token->str, "bar") == 0);
}

void next_addOp() {
  char *source = "(+ 1 2)";
  struct ParseState state = (struct ParseState){NULL, 0};
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_SYMBOL);
  TEST_ASSERT(strcmp(state.token->str, "+") == 0);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 1);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 2);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
}

void next_listExpr() {
  char *source = "'(1 2 3)";
  struct ParseState state = (struct ParseState){NULL, 0};
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_QUOTE);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_LPAREN);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 1);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 2);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_DIGIT);
  TEST_ASSERT(state.token->val == 3);
  next(source, &state);
  TEST_ASSERT(state.token->kind == TK_RPAREN);
}

void parse_intLiteral() {
  char *source = "3";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);
  TEST_ASSERT(result.program->expressions->expression->type == EXP_LITERAL);
  TEST_ASSERT(result.program->expressions->expression->data.literal->type ==
              LIT_INTERGER);
  TEST_ASSERT(
      result.program->expressions->expression->data.literal->int_value == 3);
}

void parse_stringLiteral() {
  char *source = "\"foo\"";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);
  TEST_ASSERT(result.program->expressions->expression->type == EXP_LITERAL);
  TEST_ASSERT(result.program->expressions->expression->data.literal->type ==
              LIT_STRING);
  TEST_ASSERT(
      strcmp(
          result.program->expressions->expression->data.literal->string_value,
          "foo") == 0);
}

void parse_multipleLiteralExpressions() {
  char *source = "3 \"foo\"";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);
  TEST_ASSERT(result.program->expressions->expression->type == EXP_LITERAL);
  TEST_ASSERT(result.program->expressions->expression->data.literal->type ==
              LIT_INTERGER);
  TEST_ASSERT(
      result.program->expressions->expression->data.literal->int_value == 3);

  TEST_ASSERT(result.program->expressions->next->expression->type ==
              EXP_LITERAL);
  TEST_ASSERT(
      result.program->expressions->next->expression->data.literal->type ==
      LIT_STRING);
  TEST_ASSERT(strcmp(result.program->expressions->next->expression->data
                         .literal->string_value,
                     "foo") == 0);
}

void parse_integersSymbolicExpr() {
  char *source = "(1 2 3)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  TEST_ASSERT(result.program->expressions->expression->type ==
              EXP_SYMBOLIC_EXP);

  struct ExpressionNode *expr1 = result.program->expressions->expression->data
                                     .list->expressions->expression;
  TEST_ASSERT(expr1->type == EXP_LITERAL);
  TEST_ASSERT(expr1->data.literal->type == LIT_INTERGER);
  TEST_ASSERT(expr1->data.literal->int_value == 1);

  struct ExpressionNode *expr2 = result.program->expressions->expression->data
                                     .list->expressions->next->expression;
  TEST_ASSERT(expr2->type == EXP_LITERAL);
  TEST_ASSERT(expr2->data.literal->type == LIT_INTERGER);
  TEST_ASSERT(expr2->data.literal->int_value == 2);

  struct ExpressionNode *expr3 = result.program->expressions->expression->data
                                     .list->expressions->next->next->expression;
  TEST_ASSERT(expr3->type == EXP_LITERAL);
  TEST_ASSERT(expr3->data.literal->type == LIT_INTERGER);
  TEST_ASSERT(expr3->data.literal->int_value == 3);

  TEST_ASSERT(match(&state, TK_EOF));
}

void parse_integersList() {
  char *source = "'(1 2 3)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  TEST_ASSERT(result.program->expressions->expression->type == EXP_LIST);

  struct ExpressionNode *expr1 = result.program->expressions->expression->data
                                     .list->expressions->expression;
  TEST_ASSERT(expr1->type == EXP_LITERAL);
  TEST_ASSERT(expr1->data.literal->type == LIT_INTERGER);
  TEST_ASSERT(expr1->data.literal->int_value == 1);

  struct ExpressionNode *expr2 = result.program->expressions->expression->data
                                     .list->expressions->next->expression;
  TEST_ASSERT(expr2->type == EXP_LITERAL);
  TEST_ASSERT(expr2->data.literal->type == LIT_INTERGER);
  TEST_ASSERT(expr2->data.literal->int_value == 2);

  struct ExpressionNode *expr3 = result.program->expressions->expression->data
                                     .list->expressions->next->next->expression;
  TEST_ASSERT(expr3->type == EXP_LITERAL);
  TEST_ASSERT(expr3->data.literal->type == LIT_INTERGER);
  TEST_ASSERT(expr3->data.literal->int_value == 3);

  TEST_ASSERT(match(&state, TK_EOF));
}

void evaluate_literalExpressionInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "3";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->data.literal->int_value == 3);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 3);
}

void evaluate_literalExpressionString() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "\"foo\"";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(strcmp(expr->data.literal->string_value, "foo") == 0);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.string_value, "foo") == 0);
}

void evaluate_nil() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "nil";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(strcmp(expr->data.symbol->symbol_name, "nil") == 0);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_listWithInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'(133)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_LIST);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 133);
  TEST_ASSERT(evaluated.list_value->cdr->type == OBJ_NIL);
}

void evaluate_listWithMultipleInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'(133 234 345)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_LIST);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 133);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 234);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->cdr->type == OBJ_LIST);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->int_value ==
      345);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->cdr->type ==
      OBJ_NIL);
}

void evaluate_emptyList() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'()";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_LIST);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_emptySymbolicExp() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "()";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_addOp() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(+ 1222 21)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 1243);
}

void evaluate_subOp() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(- 1222 21)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 1201);
}

void evaluate_mulOp() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(* 1222 21)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 25662);
}

void evaluate_divOp() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(/ 1222 21)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 58);
}

void evaluate_modOp() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(% 1222 21)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 4);
}

void evaluate_orOpTrue() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(|| true false)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_orOpFalse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(|| false false)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_orOpTrueInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(|| 1 false)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_orOpTrueNil() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(|| 1 nil)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluted_nestedOps() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(+ 1 (- 2 (* 3 (/ 4 2))))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == -3);
}

void evaluate_andOpFalse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(&& false false)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_andOpFalseInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(&& 1 false)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_andOpFalseNil() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(&& 1 nil)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_andOpTrue() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(&& true true)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_andOpTrueInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(&& 1 2)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_ltOpTrue() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(< 1 2)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_ltOpFalse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(< 2 1)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_gtOpTrue() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(> 2 1)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_gtOpFalse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(> 1 2)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_notTrue() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(not false)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_notFalse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(not true)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_notFalseEq() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(not (eq 1 1))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_eqTrue() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(eq 1 1)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_eqFalse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(eq 2 1)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 0);
}

void evaluate_eqTrueNil() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(eq nil nil)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_eqTrueNilList() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(eq '() nil)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_eqTrueNilSExp() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(eq () nil)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  parse(source, &state, &result);

  struct ExpressionNode *expr = result.program->expressions->expression;
  TEST_ASSERT(expr->type == EXP_SYMBOLIC_EXP);

  struct Object evaluated = (struct Object){};
  evaluateExpressionWithContext(expr, &evaluated, &env);

  TEST_ASSERT(evaluated.type == OBJ_BOOL);
  TEST_ASSERT(evaluated.bool_value == 1);
}

void evaluate_printString() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(print \"hello\")";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_printInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(print 3)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_printBooleanT() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(print true)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_printBooleanF() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(print false)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_printList() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(print '(1 2 3))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_NIL);
}

void evaluate_ifThen() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(if true 1)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 1);
}

void evaluate_ifThenElse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(if false 1 2)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 2);
}

void evaluate_complexIf() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(if (|| (eq 1 1) false) (if true 1 2) 2)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 1);
}

void evaluate_car() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(car '(1 2 3))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 1);
}

void evaluate_cdr() {
  struct Env env = (struct Env){};
  initEnv(&env);

  char *source = "(cdr '(1 2 3))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 2);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 3);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->cdr->type == OBJ_NIL);
}

void evaluate_consIntList() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(cons 1 '(2 3))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->int_value == 1);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 2);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->int_value ==
      3);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->cdr->type ==
      OBJ_NIL);
}

void evaluate_consInt() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(cons 1 2)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->int_value == 1);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 2);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->cdr->type == OBJ_NIL);
}

void evaluate_consListList() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(cons '(1 2) '(3 4))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->list_value->car->int_value == 1);
  TEST_ASSERT(
      evaluated.list_value->car->list_value->cdr->list_value->car->int_value ==
      2);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 3);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->int_value ==
      4);
}

void evaluate_assignment() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'((= a 1) a)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 1);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 1);
}

void evaluate_assignmentComplex() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'((= a 1) (+ a 2))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 1);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 3);
}

void evaluate_defun() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'((defun fn (a) (+ a 1)) (fn 1))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_FUNCTION);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 2);
}

void evaluate_defunClosure() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'((= a 2) (= b 3) (defun fn (c) (+(+ a b) c)) (fn 1))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 2);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 3);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->type ==
      OBJ_FUNCTION);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->cdr->list_value->cdr
                  ->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->cdr->list_value->cdr
                  ->list_value->car->int_value == 6);
}

void evaluate_fact() {
  struct Env env = (struct Env){};
  initEnv(&env);

  // calc factorial with recursion
  char *source =
      "'((defun fact (n) (if (eq n 0) 1 (* n (fact (- n 1))))) (fact 5))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_FUNCTION);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 120);
}

void evaluate_assignmentAndReuse() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'((= a 1) (= a 2) a)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 1);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->int_value == 2);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->type ==
      OBJ_INTEGER);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->int_value ==
      2);
}

void evaluate_stringConcat() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(+ \"foo\" \"bar\")";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type = OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.string_value, "foobar") == 0);
}

void evaluate_while() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "'((= counter 5) (while (> counter 0) (print counter) (= "
                 "counter (- counter 1))))";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.list_value->car->int_value == 5);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_NIL);
}

void evalute_split() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(split \"1 + 1 + 1 \" \" \")";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->car->string_value, "1") == 0);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->cdr->list_value->car->string_value,
                     "+") == 0);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->type ==
      OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->cdr->list_value->cdr->list_value->car
                         ->string_value,
                     "1") == 0);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->cdr->list_value->cdr
                  ->list_value->car->type == OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->cdr->list_value->cdr->list_value->cdr
                         ->list_value->car->string_value,
                     "+") == 0);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->cdr->list_value->cdr
                  ->list_value->cdr->list_value->car->type == OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->cdr->list_value->cdr->list_value->cdr
                         ->list_value->cdr->list_value->car->string_value,
                     "1") == 0);
}

void evaluate_splitWithEmptryChar() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(split \"foo\" \"\")";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_LIST);
  TEST_ASSERT(evaluated.list_value->car->type == OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->car->string_value, "f") == 0);
  TEST_ASSERT(evaluated.list_value->cdr->list_value->car->type == OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->cdr->list_value->car->string_value,
                     "o") == 0);
  TEST_ASSERT(
      evaluated.list_value->cdr->list_value->cdr->list_value->car->type ==
      OBJ_STRING);
  TEST_ASSERT(strcmp(evaluated.list_value->cdr->list_value->cdr->list_value->car
                         ->string_value,
                     "o") == 0);
}

void evaluate_progn() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(progn 1 2 3)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 3);
}

void evaluate_listRef() {
  struct Env env = (struct Env){};
  initEnv(&env);
  char *source = "(list-ref '(1 2 3) 1)";
  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult result = (struct ParseResult){NULL};
  struct Object evaluated = (struct Object){};
  parse(source, &state, &result);
  evaluateExpressionWithContext(result.program->expressions->expression,
                                &evaluated, &env);
  TEST_ASSERT(evaluated.type == OBJ_INTEGER);
  TEST_ASSERT(evaluated.int_value == 2);
}

int main() {
  RUN_TEST(next_singleCharSymbol);
  RUN_TEST(next_multipleCharSymbol);
  RUN_TEST(next_parenAndDigit);
  RUN_TEST(next_ifAndSet);
  RUN_TEST(next_string);
  RUN_TEST(next_addOp);
  RUN_TEST(next_listExpr);

  RUN_TEST(parse_intLiteral);
  RUN_TEST(parse_stringLiteral);
  RUN_TEST(parse_multipleLiteralExpressions);
  RUN_TEST(parse_integersSymbolicExpr);
  RUN_TEST(parse_integersList);

  RUN_TEST(evaluate_literalExpressionInt);
  RUN_TEST(evaluate_literalExpressionString);
  RUN_TEST(evaluate_nil);
  RUN_TEST(evaluate_listWithInt);
  RUN_TEST(evaluate_listWithMultipleInt);
  RUN_TEST(evaluate_emptyList);
  RUN_TEST(evaluate_emptySymbolicExp);
  RUN_TEST(evaluate_addOp);
  RUN_TEST(evaluate_subOp);
  RUN_TEST(evaluate_mulOp);
  RUN_TEST(evaluate_divOp);
  RUN_TEST(evaluate_modOp);
  RUN_TEST(evaluate_orOpTrue);
  RUN_TEST(evaluate_orOpFalse);
  RUN_TEST(evaluate_orOpTrueInt);
  RUN_TEST(evaluate_orOpTrueNil);
  RUN_TEST(evaluted_nestedOps);
  RUN_TEST(evaluate_andOpFalse);
  RUN_TEST(evaluate_andOpFalseInt);
  RUN_TEST(evaluate_andOpFalseNil);
  RUN_TEST(evaluate_andOpTrue);
  RUN_TEST(evaluate_andOpTrueInt);
  RUN_TEST(evaluate_ltOpTrue);
  RUN_TEST(evaluate_ltOpFalse);
  RUN_TEST(evaluate_gtOpTrue);
  RUN_TEST(evaluate_gtOpFalse);
  RUN_TEST(evaluate_eqTrue);
  RUN_TEST(evaluate_eqFalse);
  RUN_TEST(evaluate_eqTrueNil);
  RUN_TEST(evaluate_eqTrueNilList);
  RUN_TEST(evaluate_eqTrueNilSExp);
  RUN_TEST(evaluate_printString);
  RUN_TEST(evaluate_printInt);
  RUN_TEST(evaluate_printBooleanT);
  RUN_TEST(evaluate_printBooleanF);
  RUN_TEST(evaluate_printList);
  RUN_TEST(evaluate_ifThen);
  RUN_TEST(evaluate_ifThenElse);
  RUN_TEST(evaluate_complexIf);
  RUN_TEST(evaluate_car);
  RUN_TEST(evaluate_cdr);
  RUN_TEST(evaluate_consIntList);
  RUN_TEST(evaluate_consInt);
  RUN_TEST(evaluate_consListList);
  RUN_TEST(evaluate_assignment);
  RUN_TEST(evaluate_assignmentComplex);
  RUN_TEST(evaluate_defun);
  RUN_TEST(evaluate_defunClosure);
  RUN_TEST(evaluate_notTrue);
  RUN_TEST(evaluate_notFalse);
  RUN_TEST(evaluate_notFalseEq);
  RUN_TEST(evaluate_fact);
  RUN_TEST(evaluate_assignmentAndReuse);
  RUN_TEST(evaluate_stringConcat);
  RUN_TEST(evalute_split);
  RUN_TEST(evaluate_splitWithEmptryChar);
  RUN_TEST(evaluate_listRef);
  RUN_TEST(evaluate_progn);

  return 0;
}
