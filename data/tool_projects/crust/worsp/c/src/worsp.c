#include "worsp.h"
#include <ctype.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// =================================================
//   tokenizer
// =================================================

int isop(int ch) {
  return ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%' ||
         ch == '|' || ch == '&' || ch == '=' || ch == '<' || ch == '>';
}

int match(struct ParseState *state, TokenKind kind) {
  return state->token->kind == kind ? 1 : 0;
}

void next(char *source, struct ParseState *state) {
  // Skip whitespaces
  while (isspace(source[state->pos]) || source[state->pos] == '\n') {
    state->pos++;
  }

  struct Token *current = state->token;
  struct Token *new = malloc(sizeof(struct Token));

  if (source[state->pos] == '(') {
    new->kind = TK_LPAREN;
    new->str = "(";
    state->pos++;
  } else if (source[state->pos] == ')') {
    new->kind = TK_RPAREN;
    new->str = ")";
    state->pos++;
  } else if (source[state->pos] == '\'') {
    new->kind = TK_QUOTE;
    new->str = "'";
    state->pos++;
  } else if (source[state->pos] == '\0') {
    new->kind = TK_EOF;
    new->str = "\0";
  } else if (isalpha(source[state->pos]) || isop(source[state->pos])) {
    // tokenize symbol
    int start = state->pos;
    while (isalnum(source[state->pos]) || isop(source[state->pos])) {
      state->pos++;
    }
    int length = state->pos - start;

    char *str = malloc(length + 1);
    strncpy(str, &source[start], length);
    str[length] = '\0';

    if (strcmp(str, "true") == 0) {
      new->kind = TK_TRUE;
      free(str);
    } else if (strcmp(str, "false") == 0) {
      new->kind = TK_FALSE;
      free(str);
    } else {
      new->kind = TK_SYMBOL;
      new->str = str;
    }
  } else if (isdigit(source[state->pos])) {
    // tokenize digit
    int start = state->pos;
    while (isdigit(source[state->pos])) {
      state->pos++;
    }
    int length = state->pos - start;

    char *str = malloc(length + 1);
    strncpy(str, &source[start], length);
    str[length] = '\0';
    int val = atoi(str);
    free(str);

    new->kind = TK_DIGIT;
    new->val = val;
  } else if (source[state->pos] == '"') {
    // tokenize string
    state->pos++; // skip '"'
    int start = state->pos;
    while (source[state->pos] != '"' && source[state->pos] != '\0') {
      state->pos++;
    }
    int length = state->pos - start;
    new->kind = TK_STRING;
    new->str = malloc(length + 1);
    strncpy(new->str, &source[start], length);
    new->str[length] = '\0';
    if (source[state->pos] == '"') {
      state->pos++; // Skip quote
    }
  } else if (source[state->pos] == ';') {
    // tokenize comment
    while (source[state->pos] != '\n' && source[state->pos] != '\0') {
      state->pos++;
    }
    next(source, state);
    return;
  } else {
    printf("Unexpected token: %c\n", source[state->pos]);
    exit(1);
  }

  if (current == NULL) {
    state->token = new;
    return;
  }
  state->token = current->next = new;
}

// =================================================
//   parser
//     <program>          ::= <expression>*
//     <expression>       ::=
//                        | <s_expression>
//                        | <list>
//                        | <symbol>
//                        | <literal>
//     <s_expression>     ::= "(" <expression>* ")"
//     <list>             ::= "'(" <expression>* ")"
//     <symbol>           ::= <symbol_name>
//     <literal>          ::=
//                         | <interger_literal>
//                         | <string_literal>
//                         | <boolean_literal>
//     <interger_literal> ::= '1' | '2' | '3' | ...
//     <string_literal>   ::= '"' <letter>* '"'
//     <letter>           ::= 'a' | 'b' | 'c' | ...
//     <boolean_literal>  ::= 'true' | 'false'
// =================================================

void appendExpressionToListExpression(struct ListNode *listNode,
                                      struct ExpressionNode *expression) {
  struct ExpressionList *expressions = malloc(sizeof(struct ExpressionList));
  expressions->expression = expression;
  expressions->next = NULL;
  if (listNode->expressions == NULL) {
    listNode->expressions = expressions;
    return;
  }
  struct ExpressionList *current = listNode->expressions;
  while (current->next != NULL) {
    current = current->next;
  }
  current->next = expressions;
}

void appendExpressionToSymbolicExpression(
    struct SymbolicExpNode *symbolicExpNode,
    struct ExpressionNode *expression) {
  struct ExpressionList *expressions = malloc(sizeof(struct ExpressionList));
  expressions->expression = expression;
  expressions->next = NULL;
  if (symbolicExpNode->expressions == NULL) {
    symbolicExpNode->expressions = expressions;
    return;
  }
  struct ExpressionList *current = symbolicExpNode->expressions;
  while (current->next != NULL) {
    current = current->next;
  }
  current->next = expressions;
}

void parseExpression(char *source, struct ParseState *state,
                     struct ExpressionNode *expression);

void parseSymbolicExpression(char *source, struct ParseState *state,
                             struct ExpressionNode *expression) {
  struct SymbolicExpNode *symbolicExp = malloc(sizeof(struct SymbolicExpNode));

  expression->type = EXP_SYMBOLIC_EXP;
  expression->data.symbolic_exp = symbolicExp;
  expression->data.symbolic_exp->expressions = NULL;
  next(source, state); // eat '('
  while (!match(state, TK_RPAREN)) {
    struct ExpressionNode *expressionItem =
        malloc(sizeof(struct ExpressionNode));
    parseExpression(source, state, expressionItem);
    appendExpressionToSymbolicExpression(symbolicExp, expressionItem);
  }
  next(source, state); // eat ')'
}

void parseListExpression(char *source, struct ParseState *state,
                         struct ExpressionNode *expression) {
  struct ListNode *list = malloc(sizeof(struct ListNode));

  expression->type = EXP_LIST;
  expression->data.list = list;
  expression->data.list->expressions = NULL;
  next(source, state); // eat quote
  next(source, state); // eat '('
  while (!match(state, TK_RPAREN)) {
    struct ExpressionNode *expressionItem =
        malloc(sizeof(struct ExpressionNode));
    parseExpression(source, state, expressionItem);
    appendExpressionToListExpression(list, expressionItem);
  }
  next(source, state); // eat ')'
}

void parseSymbolExpression(char *source, struct ParseState *state,
                           struct ExpressionNode *expression) {
  expression->type = EXP_SYMBOL;
  expression->data.symbol = malloc(sizeof(struct SymbolNode));
  expression->data.symbol->symbol_name = state->token->str;
  next(source, state);
}

void parseLiteralExpression(char *source, struct ParseState *state,
                            struct ExpressionNode *expression) {
  if (match(state, TK_DIGIT)) {
    expression->type = EXP_LITERAL;
    expression->data.literal = malloc(sizeof(struct LiteralNode));
    expression->data.literal->type = LIT_INTERGER;
    expression->data.literal->int_value = state->token->val;
    next(source, state);
  } else if (match(state, TK_STRING)) {
    expression->type = EXP_LITERAL;
    expression->data.literal = malloc(sizeof(struct LiteralNode));
    expression->data.literal->type = LIT_STRING;
    expression->data.literal->string_value = state->token->str;
    next(source, state);
  } else if (match(state, TK_TRUE)) {
    expression->type = EXP_LITERAL;
    expression->data.literal = malloc(sizeof(struct LiteralNode));
    expression->data.literal->type = LIT_BOOLEAN;
    expression->data.literal->boolean_value = true;
    next(source, state);
  } else if (match(state, TK_FALSE)) {
    expression->type = EXP_LITERAL;
    expression->data.literal = malloc(sizeof(struct LiteralNode));
    expression->data.literal->type = LIT_BOOLEAN;
    expression->data.literal->boolean_value = false;
    next(source, state);
  } else {
    printf("Unexpected token: %s\n", state->token->str);
    exit(1);
  }
}

void parseExpression(char *source, struct ParseState *state,
                     struct ExpressionNode *expression) {
  if (match(state, TK_LPAREN)) {
    parseSymbolicExpression(source, state, expression);
  } else if (match(state, TK_QUOTE)) {
    parseListExpression(source, state, expression);
  } else if (match(state, TK_SYMBOL)) {
    parseSymbolExpression(source, state, expression);
  } else if (match(state, TK_DIGIT) || match(state, TK_STRING) ||
             match(state, TK_TRUE) || match(state, TK_FALSE)) {
    parseLiteralExpression(source, state, expression);
  } else {
    printf("Unexpected token: %s\n", state->token->str);
    exit(1);
  }
}

void appendExpressionToProgram(struct ProgramNode *program,
                               struct ExpressionNode *expression) {
  struct ExpressionList *expressions = malloc(sizeof(struct ExpressionList));
  expressions->expression = expression;
  expressions->next = NULL;
  if (program->expressions == NULL) {
    program->expressions = expressions;
    return;
  }
  struct ExpressionList *current = program->expressions;
  while (current->next != NULL) {
    current = current->next;
  }
  current->next = expressions;
}

void parseProgram(char *source, struct ParseState *state,
                  struct ParseResult *result) {
  // Set first token
  next(source, state);
  struct ProgramNode *program = malloc(sizeof(struct ProgramNode));
  program->expressions = NULL;

  result->program = program;

  while (!match(state, TK_EOF)) {
    struct ExpressionNode *expression = malloc(sizeof(struct ExpressionNode));
    parseExpression(source, state, expression);
    appendExpressionToProgram(program, expression);
  }
}

void parse(char *source, struct ParseState *state, struct ParseResult *result) {
  parseProgram(source, state, result);
}

// =================================================
//   garbage collector
// =================================================

void initializeObjectStack(struct ObjectStack *stack) { stack->top = -1; }

int isFullObjectStack(struct ObjectStack *stack) {
  return stack->top == OBJECT_NUMBER;
}

int isEmptyObjectStack(struct ObjectStack *stack) { return stack->top == -1; }

void pushObjectStack(struct ObjectStack *stack, struct Object *obj) {
  if (isFullObjectStack(stack)) {
    printf("Object stack is full.\n");
    exit(1);
  }
  stack->top++;
  stack->objects[stack->top] = obj;
}

struct Object *popObjectStack(struct ObjectStack *stack) {
  if (isEmptyObjectStack(stack)) {
    printf("Object stack is empty.\n");
    exit(1);
  }
  struct Object *obj = stack->objects[stack->top];
  stack->top--;
  return obj;
}

struct AllocatorContext *initAllocator() {
  struct AllocatorContext *context = malloc(sizeof(struct AllocatorContext));

  context->memory_pool = (struct Object *)malloc(MEMORY_SIZE);
  for (unsigned long i = 0; i < FREE_BITMAP_SIZE; ++i) {
    context->free_bitmap[i] = 0;
  }

  context->gc_less_mode = 0;

  struct ObjectStack *stack = malloc(sizeof(struct ObjectStack));
  initializeObjectStack(stack);
  context->stack = stack;

  return context;
}

void freeObject(struct AllocatorContext *context, struct Object *object) {
  int index = object - context->memory_pool;
  context->free_bitmap[index] = 0;
}

void sweep(struct AllocatorContext *context) {
  for (unsigned long i = 0; i < FREE_BITMAP_SIZE; ++i) {
    if (context->free_bitmap[i] == 1) {
      struct Object *obj = &context->memory_pool[i];
      if (obj->marked) {
        obj->marked = false;
      } else {
        freeObject(context, obj);
      }
    }
  }
}

int isLastConsCell(struct ConsCell *conscell) {
  return conscell->cdr->type == OBJ_NIL;
}

void mark(struct Object *obj) {
  if (obj->marked) {
    return;
  }
  obj->marked = true;
  if (obj->type == OBJ_LIST) {
    struct Object *current = obj;
    while (1) {
      mark(current);
      // evaluating list object can be nil
      if (current->list_value == NULL) {
        break;
      }
      mark(current->list_value->car);
      if (isLastConsCell(current->list_value)) {
        break;
      }
      current = current->list_value->cdr;
    }
  }
}

void markAll(struct Env *env, struct AllocatorContext *context) {
  // mark objects in stack
  for (int i = 0; i <= context->stack->top; i++) {
    struct Object *obj = context->stack->objects[i];
    mark(obj);
  }

  int i = 0;
  while (env->bindings[i].symbol_name != NULL) {
    struct Object *obj = env->bindings[i].value;
    mark(obj);
    i++;
  }
  if (env->parent != NULL) {
    markAll(env->parent, context);
  }
}

void gc(struct AllocatorContext *context, struct Env *env) {
  markAll(env, context);
  sweep(context);
}

struct Object *allocate(struct AllocatorContext *context, struct Env *env) {
  size_t object_size = sizeof(struct Object);
  if (context->gc_less_mode) {
    return malloc(object_size);
  }

  for (unsigned long i = 0; i < FREE_BITMAP_SIZE; ++i) {
    if (context->free_bitmap[i] == 0) {
      context->free_bitmap[i] = 1; // Mark the block as used
      return &context->memory_pool[i];
    }
  }

  gc(context, env);

  for (unsigned long i = 0; i < FREE_BITMAP_SIZE; ++i) {
    if (context->free_bitmap[i] == 0) {
      context->free_bitmap[i] = 1; // Mark the block as used
      return &context->memory_pool[i];
    }
  }

  fprintf(stderr, "Out of Memory\n");
  exit(1);
}

// =================================================
//   defined functions
// =================================================

bool boolVal(struct Object *obj) {
  if (obj->type == OBJ_BOOL) {
    return obj->bool_value;
  } else if (obj->type == OBJ_NIL) {
    return false;
  } else {
    return true;
  }
}

int eq(struct Object *op1, struct Object *op2) {
  if (op1->type != op2->type) {
    return 0;
  } else {
    if (op1->type == OBJ_INTEGER) {
      return op1->int_value == op2->int_value;
    } else if (op1->type == OBJ_STRING) {
      return strcmp(op1->string_value, op2->string_value) == 0;
    } else if (op1->type == OBJ_BOOL) {
      return op1->bool_value == op2->bool_value;
    } else if (op1->type == OBJ_LIST) {
      return op1->list_value == op2->list_value;
    } else if (op1->type == OBJ_NIL) {
      return 1;
    }
  }
  return 0;
}

char *stringifyObject(struct Object *obj) {
  if (obj->type == OBJ_INTEGER) {
    if (obj->int_value == 0) {
      char *str = (char *)malloc(2 * sizeof(char));
      strncpy(str, "0", 2);
      return str;
    }
    int digits = (int)log10(obj->int_value) + 1;
    char *str = (char *)malloc((digits + 1) * sizeof(char));
    sprintf(str, "%d", obj->int_value);
    return str;
  } else if (obj->type == OBJ_STRING) {
    char *str = (char *)malloc((strlen(obj->string_value) + 1) * sizeof(char));
    strncpy(str, obj->string_value, strlen(obj->string_value) + 1);
    return str;
  } else if (obj->type == OBJ_BOOL) {
    char *str = (char *)malloc(1 * sizeof(char));
    if (obj->bool_value) {
      strncpy(str, "T", 2);
    } else {
      strncpy(str, "F", 2);
    }
    return str;
  } else if (obj->type == OBJ_LIST) {
    int length = 2; // '(' and ')'
    char *str = (char *)malloc(length + 1 * sizeof(char));
    memset(str, 0, length + 1);
    str[0] = '(';
    struct ConsCell *current = obj->list_value;
    while (1) {
      char *serialized = stringifyObject(current->car);
      length += strlen(serialized);
      str = realloc(str, length + 1);
      strncat(str, serialized, strlen(serialized));
      if (isLastConsCell(current)) {
        break;
      } else {
        length += 1; // " "
        str = realloc(str, length + 1);
        strncat(str, " ", 1);
      }
      current = current->cdr->list_value;
    }
    str[length - 1] = ')';
    str[length] = '\0';
    return str;
  } else if (obj->type == OBJ_NIL) {
    char *str = (char *)malloc(4 * sizeof(char));
    strncpy(str, "nil", 4);
    return str;
  } else if (obj->type == OBJ_FUNCTION) {
    char *str = (char *)malloc(9 * sizeof(char));
    strncpy(str, "<function>", 10);
    return str;
  } else {
    printf("Unexpected object type: %d\n", obj->type);
    exit(1);
  }
}

void definedFunctionAdd(struct Object *op1, struct Object *op2,
                        struct Object *evaluated) {
  if (op1->type == OBJ_INTEGER && op2->type == OBJ_INTEGER) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = op1->int_value + op2->int_value;
  } else if (op1->type == OBJ_STRING && op2->type == OBJ_STRING) {
    evaluated->type = OBJ_STRING;
    evaluated->string_value =
        malloc((strlen(op1->string_value) + strlen(op2->string_value) + 1) *
               sizeof(char));
    strncpy(evaluated->string_value, op1->string_value,
            strlen(op1->string_value) + 1);
    strcat(evaluated->string_value, op2->string_value);
  } else {
    printf("Type error: operands for + must be integers or strings.\n");
    exit(1);
  }
}

void definedFunctionSub(struct Object *op1, struct Object *op2,
                        struct Object *evaluated) {
  if (op1->type == OBJ_INTEGER && op2->type == OBJ_INTEGER) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = op1->int_value - op2->int_value;
  } else {
    printf("Type error: operands for - must be integers.\n");
    exit(1);
  }
}

void definedFunctionMul(struct Object *op1, struct Object *op2,
                        struct Object *evaluated) {
  if (op1->type == OBJ_INTEGER && op2->type == OBJ_INTEGER) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = op1->int_value * op2->int_value;
  } else {
    printf("Type error: operands for * must be integers.\n");
    exit(1);
  }
}

void definedFunctionDiv(struct Object *op1, struct Object *op2,
                        struct Object *evaluated) {
  if (op1->type == OBJ_INTEGER && op2->type == OBJ_INTEGER) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = op1->int_value / op2->int_value;
  } else {
    printf("Type error: operands for / must be integers.\n");
    exit(1);
  }
}

void definedFunctionMod(struct Object *op1, struct Object *op2,
                        struct Object *evaluated) {
  if (op1->type == OBJ_INTEGER && op2->type == OBJ_INTEGER) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = op1->int_value % op2->int_value;
  } else {
    printf("Type error: operands for % must be integers.\n");
    exit(1);
  }
}

void definedFunctionOr(struct Object *op1, struct Object *op2,
                       struct Object *evaluated) {
  if (boolVal(op1) || boolVal(op2)) {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = 1;
  } else {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = 0;
  }
}

void definedFunctionAnd(struct Object *op1, struct Object *op2,
                        struct Object *evaluated) {
  if (boolVal(op1) && boolVal(op2)) {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = 1;
  } else {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = 0;
  }
}

void definedFunctionLt(struct Object *op1, struct Object *op2,
                       struct Object *evaluated) {
  if (op1->type == OBJ_INTEGER && op2->type == OBJ_INTEGER) {
    if (op1->int_value < op2->int_value) {
      evaluated->type = OBJ_BOOL;
      evaluated->bool_value = 1;
    } else {
      evaluated->type = OBJ_BOOL;
      evaluated->bool_value = 0;
    }
  } else {
    printf("Type error: operands for < must be integers.\n");
    exit(1);
  }
}

void definedFunctionGt(struct Object *op1, struct Object *op2,
                       struct Object *evaluated) {
  if (op1->type == OBJ_INTEGER && op2->type == OBJ_INTEGER) {
    if (op1->int_value > op2->int_value) {
      evaluated->type = OBJ_BOOL;
      evaluated->bool_value = 1;
    } else {
      evaluated->type = OBJ_BOOL;
      evaluated->bool_value = 0;
    }
  } else {
    printf("Type error: operands for < must be integers.\n");
    exit(1);
  }
}

void definedFunctionEq(struct Object *op1, struct Object *op2,
                       struct Object *evaluated) {
  if (eq(op1, op2)) {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = 1;
  } else {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = 0;
  }
}

void definedFunctionCar(struct Object *op, struct Object *evaluated) {
  if (op->type != OBJ_LIST) {
    printf("Type error: car operand must be list.\n");
    exit(1);
  }
  *evaluated = *(op->list_value->car);
}

void definedFunctionCdr(struct Object *op, struct Object *evaluated) {
  if (op->type != OBJ_LIST) {
    printf("Type error: cdr operand must be list.\n");
    exit(1);
  }
  *evaluated = *op->list_value->cdr;
}

void definedFunctionCons(struct Object *op1, struct Object *op2,
                         struct Object *evaluated, struct Env *env,
                         struct AllocatorContext *context) {
  evaluated->type = OBJ_LIST;
  evaluated->list_value = malloc(sizeof(struct ConsCell));
  evaluated->list_value->car = op1;

  if (op2->type == OBJ_LIST) {
    evaluated->list_value->type = CONSCELL_TYPE_CELL;
    evaluated->list_value->cdr = op2;
  } else if (op2->type == OBJ_NIL) {
    evaluated->list_value->cdr = op2;
  } else {
    struct Object *cdr_obj = allocate(context, env);
    cdr_obj->type = OBJ_LIST;
    struct ConsCell *new_conscell = malloc(sizeof(struct ConsCell));
    new_conscell->car = op2;
    new_conscell->type = CONSCELL_TYPE_CELL;
    new_conscell->cdr = allocate(context, env);
    new_conscell->cdr->type = OBJ_NIL;
    cdr_obj->list_value = new_conscell;
    evaluated->list_value->cdr = cdr_obj;
  }
}

void definedFunctionNot(struct Object *op, struct Object *evaluated) {
  if (op->type != OBJ_BOOL) {
    printf("Type error: not operand must be boolean.\n");
    exit(1);
  }
  evaluated->type = OBJ_BOOL;
  if (op->bool_value) {
    evaluated->bool_value = 0;
  } else {
    evaluated->bool_value = 1;
  }
}

void definedFunctionSplit(struct Object *op1, struct Object *op2,
                          struct Object *evaluated, struct Env *env,
                          struct AllocatorContext *context) {
  if (op1->type != OBJ_STRING) {
    printf("Type error: split first operand must be string.\n");
    exit(1);
  }
  if (op2->type != OBJ_STRING) {
    printf("Type error: split second operand must be string.\n");
    exit(1);
  }

  // when op2 is "", return list of characters
  if (strcmp(op2->string_value, "") == 0) {
    evaluated->type = OBJ_LIST;
    evaluated->list_value = malloc(sizeof(struct ConsCell));
    evaluated->list_value->type = CONSCELL_TYPE_CELL;
    evaluated->list_value->car = allocate(context, env);
    evaluated->list_value->car->type = OBJ_STRING;
    evaluated->list_value->car->string_value = malloc(sizeof(char));
    evaluated->list_value->car->string_value[0] = op1->string_value[0];
    evaluated->list_value->cdr = allocate(context, env);
    evaluated->list_value->cdr->type = OBJ_NIL;
    struct ConsCell *current = evaluated->list_value;
    for (unsigned long i = 1; i < strlen(op1->string_value); i++) {
      struct ConsCell *new_conscell = malloc(sizeof(struct ConsCell));
      new_conscell->type = CONSCELL_TYPE_CELL;
      new_conscell->car = allocate(context, env);
      new_conscell->car->type = OBJ_STRING;
      new_conscell->car->string_value = malloc(sizeof(char));
      new_conscell->car->string_value[0] = op1->string_value[i];
      new_conscell->cdr = allocate(context, env);
      new_conscell->cdr->type = OBJ_NIL;
      current->cdr = allocate(context, env);
      current->cdr->type = OBJ_LIST;
      current->cdr->list_value = new_conscell;
      current = current->cdr->list_value;
    }
    return;
  }

  // split op1 string by op2 string and return list of strings
  evaluated->type = OBJ_LIST;
  evaluated->list_value = malloc(sizeof(struct ConsCell));
  evaluated->list_value->type = CONSCELL_TYPE_CELL;
  evaluated->list_value->car = allocate(context, env);
  evaluated->list_value->car->type = OBJ_STRING;
  evaluated->list_value->car->string_value =
      strtok(op1->string_value, op2->string_value);
  evaluated->list_value->cdr = allocate(context, env);
  evaluated->list_value->cdr->type = OBJ_NIL;
  struct ConsCell *current = evaluated->list_value;
  while (1) {
    char *token = strtok(NULL, op2->string_value);
    if (token == NULL) {
      break;
    }
    struct ConsCell *new_conscell = malloc(sizeof(struct ConsCell));
    new_conscell->type = CONSCELL_TYPE_CELL;
    new_conscell->car = allocate(context, env);
    new_conscell->car->type = OBJ_STRING;
    new_conscell->car->string_value = token;
    new_conscell->cdr = allocate(context, env);
    new_conscell->cdr->type = OBJ_NIL;
    current->cdr = allocate(context, env);
    current->cdr->type = OBJ_LIST;
    current->cdr->list_value = new_conscell;
    current = current->cdr->list_value;
  }
}

void definedFunctionListRef(struct Object *op1, struct Object *op2,
                            struct Object *evaluated) {
  if (op1->type != OBJ_LIST) {
    printf("Type error: list-ref first operand must be list.\n");
    exit(1);
  }
  if (op2->type != OBJ_INTEGER) {
    printf("Type error: list-ref second operand must be integer.\n");
    exit(1);
  }
  int index = op2->int_value;
  struct ConsCell *current = op1->list_value;
  for (int i = 0; i < index; i++) {
    if (current->cdr->type == OBJ_NIL) {
      printf("Index out of range.\n");
      exit(1);
    }
    current = current->cdr->list_value;
  }
  *evaluated = *current->car;
}

void definedFunctionRemoveWhitespaces(struct Object *op1,
                                      struct Object *evaluated) {
  if (op1->type != OBJ_STRING) {
    printf("Type error: remove-whitespaces operand must be string.\n");
    exit(1);
  }
  char *str = op1->string_value;
  char *new_str = malloc(sizeof(char) * (strlen(str) + 1));
  int i = 0;
  int j = 0;
  while (str[i]) {
    if (!isspace(str[i])) {
      new_str[j++] = str[i];
    }
    i++;
  }
  new_str[j] = '\0';
  evaluated->type = OBJ_STRING;
  evaluated->string_value = new_str;
}

void definedFunctionPop(struct Object *op, struct Object *evaluated) {
  // empty list is evaluted as nil
  if (op->type == OBJ_NIL) {
    evaluated->type = OBJ_NIL;
    return;
  }
  if (op->type != OBJ_LIST) {
    printf("Type error: pop operand must be list.\n");
    exit(1);
  }
  struct ConsCell *current = op->list_value;
  struct ConsCell *prev = NULL;
  while (1) {
    if (isLastConsCell(current)) {
      if (prev == NULL) {
        *evaluated = *current->car;
      } else {
        prev->cdr->type = OBJ_NIL;
        prev->cdr->list_value = NULL;
        *evaluated = *current->car;
      }
      break;
    }
    prev = current;
    current = current->cdr->list_value;
  }
}

void definedFunctionPush(struct Object *op1, struct Object *op2,
                         struct Object *evaluated, struct Env *env,
                         struct AllocatorContext *context) {
  if (op1->type == OBJ_NIL) {
    *evaluated = *op2;

    int i = 0;
    while (env->bindings[i].symbol_name != NULL) {
      if (env->bindings[i].value == op1) {
        struct Object *new_list = allocate(context, env);
        new_list->type = OBJ_LIST;
        new_list->list_value = malloc(sizeof(struct ConsCell));
        new_list->list_value->type = CONSCELL_TYPE_CELL;
        new_list->list_value->car = op2;
        new_list->list_value->cdr = allocate(context, env);
        new_list->list_value->cdr->type = OBJ_NIL;
        env->bindings[i].value = new_list;
        break;
      }
      i++;
    }

    return;
  }

  if (op1->type != OBJ_LIST) {
    printf("Type error: push second operand must be list.\n");
    exit(1);
  }

  struct ConsCell *current = op1->list_value;
  while (1) {
    if (isLastConsCell(current)) {
      struct ConsCell *new_conscell = malloc(sizeof(struct ConsCell));
      new_conscell->type = CONSCELL_TYPE_CELL;
      new_conscell->car = op2;
      new_conscell->cdr = allocate(context, env);
      new_conscell->cdr->type = OBJ_NIL;
      current->cdr->type = OBJ_LIST;
      current->cdr->list_value = new_conscell;
      break;
    }
    current = current->cdr->list_value;
  }

  *evaluated = *op2;
}

void definedFunctionLength(struct Object *op, struct Object *evaluated) {
  if (op->type == OBJ_NIL) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = 0;
    return;
  }
  if (op->type == OBJ_LIST) {
    int length = 1;
    struct ConsCell *current = op->list_value;
    while (1) {
      if (isLastConsCell(current)) {
        break;
      }
      length++;
      current = current->cdr->list_value;
    }
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = length;
  } else if (op->type == OBJ_STRING) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = strlen(op->string_value);
  } else {
    printf("Type error: length operand must be list or string.\n");
    exit(1);
  }
}

void definedFunctionIsIntString(struct Object *op, struct Object *evaluated) {
  if (op->type == OBJ_STRING) {
    char *str = op->string_value;
    int i = 0;
    while (str[i]) {
      if (!isdigit(str[i])) {
        evaluated->type = OBJ_BOOL;
        evaluated->bool_value = false;
        return;
      }
      i++;
    }
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = true;
  } else {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = false;
  }
}

void definedFunctionParseInt(struct Object *op, struct Object *evaluated) {
  if (op->type != OBJ_STRING) {
    printf("Type error: parse-int operand must be string.\n");
    exit(1);
  }
  char *str = op->string_value;
  int i = 0;
  while (str[i]) {
    if (!isdigit(str[i])) {
      printf("Type error: parse-int operand must be string of digits.\n");
      exit(1);
    }
    i++;
  }
  evaluated->type = OBJ_INTEGER;
  evaluated->int_value = atoi(str);
}

void definedFunctionStringRef(struct Object *op1, struct Object *op2, struct Object *evaluated) {
  if (op1->type != OBJ_STRING) {
    printf("Type error: string-ref first operand must be string.\n");
    exit(1);
  }
  if (op2->type != OBJ_INTEGER) {
    printf("Type error: string-ref second operand must be integer.\n");
    exit(1);
  }
  int index = op2->int_value;

  if (index < 0 || index >= (int)strlen(op1->string_value)) {
    printf("Index out of range.\n");
    exit(1);
  }
  evaluated->type = OBJ_STRING;
  evaluated->string_value = malloc(sizeof(char) * 2);
  evaluated->string_value[0] = op1->string_value[index];
  evaluated->string_value[1] = '\0';
}

// =================================================
//   evaluator
// =================================================

void evaluateListExpression(struct ExpressionNode *expression,
                            struct Object *evaluated, struct Env *env,
                            struct AllocatorContext *context) {
  struct ExpressionList *expressions = expression->data.list->expressions;

  // empty data list is evaluated as nil
  if (expressions == NULL) {
    evaluated->type = OBJ_NIL;
    return;
  }

  evaluated->type = OBJ_LIST;

  struct ConsCell *car_conscell = NULL;
  struct ConsCell *prev_conscell = NULL;

  while (expressions != NULL) {
    struct ConsCell *new_conscell = malloc(sizeof(struct ConsCell));
    if (car_conscell == NULL) {
      car_conscell = new_conscell;
    }

    struct Object *evaluatedItem = allocate(context, env);
    new_conscell->car = evaluatedItem;
    evaluateExpression(expressions->expression, evaluatedItem, env, context);

    expressions = expressions->next;

    if (prev_conscell != NULL) {
      struct Object *new_cdr = allocate(context, env);
      new_cdr->type = OBJ_LIST;
      new_cdr->list_value = new_conscell;
      prev_conscell->cdr = new_cdr;
      prev_conscell->type = CONSCELL_TYPE_CELL;
    }
    prev_conscell = new_conscell;

    if (expressions == NULL) {
      struct Object *nilObj = allocate(context, env);
      nilObj->type = OBJ_NIL;
      prev_conscell->type = CONSCELL_TYPE_NIL;
      prev_conscell->cdr = nilObj;
    }
  }

  evaluated->list_value = car_conscell;
}

void setObjectToEnv(struct Env *env, char *symbolName, struct Object *obj) {
  // search binding that has the symbol name
  int i = 0;
  while (env->bindings[i].symbol_name != NULL) {
    if (strcmp(env->bindings[i].symbol_name, symbolName) == 0) {
      env->bindings[i].value = obj;
      return;
    }
    i++;
  }

  // if not found, set to parent env
  struct Binding *binding = malloc(sizeof(struct Binding));
  binding->symbol_name = symbolName;
  binding->value = obj;
  i = 0;
  while (env->bindings[i].symbol_name != NULL) {
    i++;
  }
  env->bindings[i] = *binding;
}

void evaluateSymbolicExpression(struct ExpressionNode *expression,
                                struct Object *evaluated, struct Env *env,
                                struct AllocatorContext *context) {
  struct ExpressionList *expressions = expression->data.list->expressions;
  if (expressions != NULL) {
    struct ExpressionNode *expr = expressions->expression;
    if (expr != NULL && expr->type == EXP_SYMBOL) {
      if (strcmp(expr->data.symbol->symbol_name, "if") == 0) {
        // if
        struct ExpressionNode *cond = expressions->next->expression;
        if (cond == NULL) {
          printf("if must have condition.\n");
          exit(1);
        }
        struct ExpressionNode *then = expressions->next->next->expression;
        if (then == NULL) {
          printf("if must have then clause.\n");
          exit(1);
        }
        struct Object *condObj = allocate(context, env);
        evaluateExpression(cond, condObj, env, context);
        if (boolVal(condObj)) {
          evaluateExpression(then, evaluated, env, context);
        } else {
          if (expressions->next->next->next != NULL) {
            struct ExpressionNode *els =
                expressions->next->next->next->expression;
            evaluateExpression(els, evaluated, env, context);
          } else {
            evaluated->type = OBJ_NIL;
          }
        }
      } else if (strcmp(expr->data.symbol->symbol_name, "while") == 0) {
        // while
        struct ExpressionNode *cond = expressions->next->expression;
        if (cond == NULL) {
          printf("if must have condition.\n");
          exit(1);
        }
        struct ExpressionNode *then = expressions->next->next->expression;
        if (then == NULL) {
          printf("if must have then clause.\n");
          exit(1);
        }
        while (1) {
          struct Object *condObj = allocate(context, env);
          evaluateExpression(cond, condObj, env, context);
          if (boolVal(condObj)) {
            evaluateExpression(then, evaluated, env, context);
          } else {
            evaluated->type = OBJ_NIL;
            break;
          }
        }
      } else if (strcmp(expr->data.symbol->symbol_name, "=") == 0) {
        // assignment
        struct ExpressionNode symbolExpr = *expressions->next->expression;
        if (symbolExpr.type != EXP_SYMBOL) {
          printf("Variable name must be symbol.\n");
          exit(1);
        }
        char *symbol_name = symbolExpr.data.symbol->symbol_name;
        struct ExpressionNode *expr = expressions->next->next->expression;

        if (expr == NULL) {
          printf("assignment must have expression.\n");
          exit(1);
        }
        struct Object *evaluatedExpr = allocate(context, env);
        evaluateExpression(expr, evaluatedExpr, env, context);
        *evaluated = *evaluatedExpr;

        // set value to current env
        setObjectToEnv(env, symbol_name, evaluatedExpr);
      } else if ((strcmp(expr->data.symbol->symbol_name, "defun") == 0)) {
        // define function
        // (defun fn (n) (+ n 1))
        struct ExpressionNode symbolExpr = *expressions->next->expression;
        if (symbolExpr.type != EXP_SYMBOL) {
          printf("Function name must be symbol.\n");
          exit(1);
        }
        char *symbol_name = symbolExpr.data.symbol->symbol_name;

        struct ExpressionNode *paramsExpr = expressions->next->next->expression;
        if (paramsExpr == NULL) {
          printf("Function must have parameter.\n");
          exit(1);
        }
        if (paramsExpr->type != EXP_SYMBOLIC_EXP) {
          printf("Function parameter must be list.\n");
          exit(1);
        }

        struct ExpressionList *params =
            paramsExpr->data.symbolic_exp->expressions;
        // check all elements are symbol and get symbol names

        char **param_symbol_names = malloc(sizeof(char *) * MAX_BINDINGS);

        int i = 0;
        while (params != NULL) {
          if (params->expression->type != EXP_SYMBOL) {
            printf("Function parameter must be symbol.\n");
            exit(1);
          }
          param_symbol_names[i] = params->expression->data.symbol->symbol_name;
          i++;
          params = params->next;
        }

        struct ExpressionNode *bodyExpr =
            expressions->next->next->next->expression;
        if (bodyExpr == NULL) {
          printf("Function must have body.\n");
          exit(1);
        }

        struct Function *function = malloc(sizeof(struct Function));
        function->param_symbol_names = param_symbol_names;
        function->body = bodyExpr;

        evaluated->type = OBJ_FUNCTION;
        evaluated->function_value = function;

        setObjectToEnv(env, symbol_name, evaluated);
      } else {
        // function call
        if (strcmp(expr->data.symbol->symbol_name, "+") == 0) {
          // +
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionAdd(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "-") == 0) {
          // -
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionSub(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "*") == 0) {
          // *
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionMul(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "/") == 0) {
          // /
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionDiv(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "%") == 0) {
          // %
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionMod(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "||") == 0) {
          // ||
          struct ExpressionList *exprs = expressions->next;
          struct Object *operand = NULL;
          while (exprs != NULL) {
            operand = allocate(context, env);
            evaluateExpression(exprs->expression, operand, env, context);
            if (boolVal(operand)) {
              evaluated->type = OBJ_BOOL;
              evaluated->bool_value = 1;
              return;
            }
            exprs = exprs->next;
          }
          evaluated->type = OBJ_BOOL;
          evaluated->bool_value = 0;
        } else if (strcmp(expr->data.symbol->symbol_name, "&&") == 0) {
          // &&
          struct ExpressionList *exprs = expressions->next;
          struct Object *operand = NULL;
          while (exprs != NULL) {
            operand = allocate(context, env);
            evaluateExpression(exprs->expression, operand, env, context);
            if (!boolVal(operand)) {
              evaluated->type = OBJ_BOOL;
              evaluated->bool_value = 0;
              return;
            }
            exprs = exprs->next;
          }
          evaluated->type = OBJ_BOOL;
          evaluated->bool_value = 1;
        } else if (strcmp(expr->data.symbol->symbol_name, "<") == 0) {
          // <
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionLt(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, ">") == 0) {
          // >
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionGt(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "eq") == 0) {
          // eq
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionEq(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "not") == 0) {
          // not
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionNot(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "print") == 0) {
          // print
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          char *str = stringifyObject(operand);
          printf("%s\n", str);
          free(str);
          evaluated->type = OBJ_NIL;
        } else if (strcmp(expr->data.symbol->symbol_name, "car") == 0) {
          // car
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionCar(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "cdr") == 0) {
          // cdr
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionCdr(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "cons") == 0) {
          // cons
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionCons(operand1, operand2, evaluated, env, context);
        } else if (strcmp(expr->data.symbol->symbol_name, "readline") == 0) {
          // readline
          char *line = NULL;
          size_t len = 0;
          ssize_t read;
          if ((read = getline(&line, &len, stdin)) != -1) {
            evaluated->type = OBJ_STRING;
            // trim newline
            line[read - 1] = '\0';
            evaluated->string_value = line;
          } else {
            evaluated->type = OBJ_NIL;
          }
        } else if (strcmp(expr->data.symbol->symbol_name, "split") == 0) {
          // split
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionSplit(operand1, operand2, evaluated, env, context);
        } else if (strcmp(expr->data.symbol->symbol_name, "list-ref") == 0) {
          // list-ref
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                             context);
          definedFunctionListRef(operand1, operand2, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "progn") == 0) {
          // progn
          struct ExpressionList *exprs = expressions->next;
          struct Object *operand = NULL;
          while (exprs != NULL) {
            operand = allocate(context, env);
            evaluateExpression(exprs->expression, operand, env, context);
            exprs = exprs->next;
          }
          if (operand != NULL) {
            *evaluated = *operand;
          } else {
            evaluated->type = OBJ_NIL;
          }
        } else if (strcmp(expr->data.symbol->symbol_name,
                          "remove-whitespaces") == 0) {
          // remove-whitespaces
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionRemoveWhitespaces(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "pop") == 0) {
          // pop
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionPop(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "push") == 0) {
          // push
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->expression, operand2, env,
                             context);
          definedFunctionPush(operand2, operand1, evaluated, env, context);
        } else if (strcmp(expr->data.symbol->symbol_name, "length") == 0) {
          // length
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionLength(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "is-int-string") ==
                   0) {
          // is-int-string
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionIsIntString(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "parse-int") == 0) {
          // parse-int
          struct Object *operand = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand, env,
                             context);
          definedFunctionParseInt(operand, evaluated);
        } else if (strcmp(expr->data.symbol->symbol_name, "string-ref") == 0) {
          // string-ref
          struct Object *operand1 = allocate(context, env);
          struct Object *operand2 = allocate(context, env);
          evaluateExpression(expressions->next->expression, operand1, env,
                             context);
          evaluateExpression(expressions->next->next->expression, operand2, env,
                              context);
          definedFunctionStringRef(operand1, operand2, evaluated);
        } else {
          // function call
          int i = 0;
          struct Env *current_env = env;
          struct Env *looked_up_env = current_env;
          while (looked_up_env->bindings[i].symbol_name != NULL) {
            if (strcmp(looked_up_env->bindings[i].symbol_name,
                       expr->data.symbol->symbol_name) == 0) {
              struct Function *function =
                  looked_up_env->bindings[i].value->function_value;
              struct Env *new_env = malloc(sizeof(struct Env));
              new_env->parent = current_env;
              int j = 0;
              struct ExpressionList *param_expr = expressions->next;
              while (function->param_symbol_names[j] != NULL) {
                struct Object *param = allocate(context, env);
                evaluateExpression(param_expr->expression, param, current_env,
                                   context);
                param_expr = param_expr->next;
                setObjectToEnv(new_env, function->param_symbol_names[j], param);
                j++;
              }
              evaluateExpression(function->body, evaluated, new_env, context);
              return;
            }
            i++;
            // look up the parent env if not found
            if (env->bindings[i].symbol_name == NULL) {
              looked_up_env = looked_up_env->parent;
              i = 0;
            }
          }
          printf("Undefined function: %s\n", expr->data.symbol->symbol_name);
          exit(1);
        }
      }
    } else {
      printf("S-exp must be started with symbol.\n");
      exit(1);
    }
  } else {
    evaluated->type = OBJ_NIL;
  }
}

void evaluateLiteralExpression(struct ExpressionNode *expression,
                               struct Object *evaluated) {
  if (expression->data.literal->type == LIT_INTERGER) {
    evaluated->type = OBJ_INTEGER;
    evaluated->int_value = expression->data.literal->int_value;
  } else if (expression->data.literal->type == LIT_STRING) {
    evaluated->type = OBJ_STRING;
    evaluated->string_value = expression->data.literal->string_value;
  } else if (expression->data.literal->type == LIT_BOOLEAN) {
    evaluated->type = OBJ_BOOL;
    evaluated->bool_value = expression->data.literal->boolean_value;
  }
}

void evaluateSymbolExpression(struct ExpressionNode *expression,
                              struct Object *evaluated, struct Env *env,
                              struct AllocatorContext *context) {
  if (strcmp(expression->data.symbol->symbol_name, "nil") == 0) {
    evaluated->type = OBJ_NIL;
  } else {
    // get symbol value from env
    int i = 0;
    while (env->bindings[i].symbol_name != NULL) {
      if (strcmp(env->bindings[i].symbol_name,
                 expression->data.symbol->symbol_name) == 0) {
        *evaluated = *env->bindings[i].value;
        env->bindings[i].value = evaluated;
        return;
      }
      i++;
    }
    if (env->parent != NULL) {
      evaluateSymbolExpression(expression, evaluated, env->parent, context);
    } else {
      printf("Undefined symbol: %s\n", expression->data.symbol->symbol_name);
      exit(1);
    }
  }
}

void evaluateExpression(struct ExpressionNode *expression,
                        struct Object *evaluated, struct Env *env,
                        struct AllocatorContext *context) {
  pushObjectStack(context->stack, evaluated);
  if (expression->type == EXP_LIST) {
    evaluateListExpression(expression, evaluated, env, context);
  } else if (expression->type == EXP_SYMBOLIC_EXP) {
    evaluateSymbolicExpression(expression, evaluated, env, context);
  } else if (expression->type == EXP_LITERAL) {
    evaluateLiteralExpression(expression, evaluated);
  } else if (expression->type == EXP_SYMBOL) {
    evaluateSymbolExpression(expression, evaluated, env, context);
  }
  popObjectStack(context->stack);
}

void evaluateExpressionWithContext(struct ExpressionNode *expression,
                                   struct Object *evaluated, struct Env *env) {
  struct AllocatorContext *context = initAllocator();
  evaluateExpression(expression, evaluated, env, context);
}

void initEnv(struct Env *env) {
  env->parent = NULL;
  env->bindings[0].symbol_name = NULL;
  env->bindings[0].value = NULL;
}

void evaluateProgram(struct ProgramNode *program) {
  struct ExpressionList *expressions = program->expressions;

  struct Env *env = malloc(sizeof(struct Env));
  initEnv(env);

  struct AllocatorContext *context = initAllocator();

  while (expressions != NULL) {
    struct Object *evaluated = allocate(context, env);
    evaluateExpression(expressions->expression, evaluated, env, context);
    expressions = expressions->next;
  }
}

void evaluate(struct ParseResult *result) { evaluateProgram(result->program); }
