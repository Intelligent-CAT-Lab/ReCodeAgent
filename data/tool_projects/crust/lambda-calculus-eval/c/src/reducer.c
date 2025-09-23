#include "reducer.h"
#include "common.h"
#include "../hash-table/hash_table.h"
#include <stdarg.h>
#include "config.h"

static reduction_order_t reduction_order = APPLICATIVE;

void set_reduction_order(reduction_order_t t) {
  reduction_order = t;
}

void print_reduction_order(reduction_order_t t) {
  switch (t) {
    case APPLICATIVE:
      printf("Applicative");
      break;
    case NORMAL:
      printf("Normal");
      break;
  }
  printf("\n");
}

AstNode *reduce(HashTable *table, AstNode *n) {
  print_verbose("Order of reduction is: ");
  print_reduction_order(reduction_order);
  print_verbose("-------------------------------------------\n");
  expand_definitions(table, n);
  print_verbose("Expanded expression:\n");
  print_ast_verbose(n);
  AstNode *reduced = reduce_ast(table, n);
  print_verbose("Final reduced expression:\n");
  print_ast_verbose(reduced);
  print_verbose("-------------------------------------------\n");
  return reduced;
}

void expand_definitions(HashTable *table, AstNode *n) {
  if (n->type == LAMBDA_EXPR) {
    expand_definitions(table, n->node.lambda_expr->body);
  } else if (n->type == APPLICATION) {
    expand_definitions(table, n->node.application->function);
    expand_definitions(table, n->node.application->argument);
  } else if (n->type == DEFINITION) {
    char *def_name = n->node.variable->name;
    AstNode *expanded_def = search(table, def_name);
    HANDLE_NULL(expanded_def);
    print_verbose("Expanding definition of: %s . Term expanded to:\n", def_name);
    print_ast_verbose(expanded_def);

    n->type = expanded_def->type;
    n->node.variable = expanded_def->node.variable;
    n->node.lambda_expr = expanded_def->node.lambda_expr;
    n->node.application = expanded_def->node.application;
  }
}

void replace(AstNode *n, char *old, char *new_name) {
  if (n->type == LAMBDA_EXPR) {
    if (strcmp(n->node.lambda_expr->parameter, old) == 0) {
      n->node.lambda_expr->parameter = new_name;
    }
    replace(n->node.lambda_expr->body, old, new_name);
  }

  else if (n->type == APPLICATION) {
    replace(n->node.application->function, old, new_name);
    replace(n->node.application->argument, old, new_name);
  }

  else if (n->type == VAR) {
    if (strcmp(n->node.variable->name, old) == 0) {
      n->node.variable->name = new_name;
    }
  }
}

AstNode *reduce_ast(HashTable *table, AstNode *n) {
  if (n->type == LAMBDA_EXPR) {
    AstNode *body = n->node.lambda_expr->body;

    // recursively reduce the body
    // if order is applicative, reduce the body right now
    if (reduction_order == APPLICATIVE) {
      n->node.lambda_expr->body = reduce_ast(table, body);
    }
    return n;
  }

  else if (n->type == APPLICATION) {
    // apply reduce rules
    AstNode *function = n->node.application->function;
    AstNode *argument = n->node.application->argument;

    n->node.application->function = reduce_ast(table, function);

    if (reduction_order == APPLICATIVE) {
      n->node.application->argument = reduce_ast(table, argument);
    }

    if (n->node.application->function->type == LAMBDA_EXPR) {
      char *param = n->node.application->function->node.lambda_expr->parameter;
      AstNode *reduced = substitute(
          n->node.application->function->node.lambda_expr->body,
          param,
          n->node.application->argument);
      print_verbose("Applied substitution to lambda expr of parameter <%s> and resulted in:\n", param);
      print_ast_verbose(reduced);

      if (reduction_order == APPLICATIVE) {
        return reduced;
      }
      return reduce_ast(table, reduced);
    }
    return n;
  }

  // base case leaf node or variable, nothing to reduce
  return n;
}

AstNode *substitute(AstNode *expression, char *variable, AstNode *replacement) {
  if (expression->type == VAR) {
    if (strcmp(expression->node.variable->name, variable) == 0) {
      return deepcopy(replacement);
    }
    return expression;
  }

  else if (expression->type == LAMBDA_EXPR) {
    // ex: @x.(x y) -> if variable is equal to @x (x), we should then replace
    // all x in the body of expression with replacement
    expression->node.lambda_expr->body =
        substitute(expression->node.lambda_expr->body, variable, replacement);
    if (expression->node.lambda_expr->parameter != variable) {
      // if @x.(x y) and we're substituting for y, for example, we do not want
      // to remove the lambda term
      return expression;
    }
    return expression->node.lambda_expr->body;

  }

  else if (expression->type == APPLICATION) {
    expression->node.application->function = substitute(
        expression->node.application->function, variable, replacement);
    expression->node.application->argument = substitute(
        expression->node.application->argument, variable, replacement);
    return expression;
  }
  return expression;
}

AstNode *deepcopy(AstNode *n) {
  if (n->type == VAR) {
    return deepcopy_var(n->node.variable->name, n->node.variable->type);
  }

  else if (n->type == LAMBDA_EXPR) {
    return deepcopy_lambda_expr(n->node.lambda_expr->parameter,
                                n->node.lambda_expr->body, n->node.lambda_expr->type);
  }

  else if (n->type == APPLICATION) {
    return deepcopy_application(n->node.application->function,
                                n->node.application->argument);
  }
  return NULL;
}

AstNode *deepcopy_application(AstNode *function, AstNode *argument) {
  AstNode *application = (AstNode *)malloc(sizeof(AstNode));
  application->type = APPLICATION;
  application->node.application = (Application *)malloc(sizeof(Application));
  application->node.application->function = deepcopy(function);
  application->node.application->argument = deepcopy(argument);
  return application;
}

AstNode *deepcopy_lambda_expr(char *parameter, AstNode *body, char *type) {
  AstNode *lambda = (AstNode *)malloc(sizeof(AstNode));
  lambda->type = LAMBDA_EXPR;
  lambda->node.lambda_expr =
      (LambdaExpression *)malloc(sizeof(LambdaExpression));
  lambda->node.lambda_expr->parameter = parameter;
  lambda->node.lambda_expr->type = type;
  lambda->node.lambda_expr->body = deepcopy(body);
  return lambda;
}

AstNode *deepcopy_var(char *name, char *type) {
  AstNode *variable = (AstNode *)malloc(sizeof(AstNode));
  variable->type = VAR;
  variable->node.variable = (Variable *)malloc(sizeof(Variable));
  variable->node.variable->name = name;
  variable->node.variable->type = type;
  return variable;
}
