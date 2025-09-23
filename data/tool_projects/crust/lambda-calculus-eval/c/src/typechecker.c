#include "common.h"
#include <stdbool.h>
#include "typechecker.h"

// at this point, when the typcheck occurs, we have a parsed AST
// containing definitions, types and a huge lambda expression to typecheck
// first of all we need to expand the definitions inside of the main AST, calling the reducer.
// This will be done on main
// Then, we call the typecheck entry point here that recursively typechecks the expresssions. We need to follow the rules:
// - If x has a type A in the context, then x has a type A
// - term constants have base types
// - If, in a certain context with x having type A, e having type B, then, in the same context without x, @x:A.(e) has type A -> B
// - If in a certain context, e1 has type A -> B and e2 has type A, then e1 e2 has type B
// So, first of all we create the main environment. This can be done by checking all type definitions (inside lambdas or variables), get their
// values on the table and add to our custom env type.
// we also need a custom type for Types, because we need to account for function types
// what if we have an expression like:    (@x : Nat (x) b : Bool) . This should be a type error, since e1 has type A -> A, not A->B
// But this error is triggered only if b is already normalized, isnt it?
// ((@x : Nat(x) (@b : Bool(b) a : Bool)) -> how do we know this is wrong?
// CALL typecheck on APP. e1: Nat->Nat e2 : APP (Bool->Bool) (e_1.1 Bool->Bool e_2.1 Bool)
// Typechecks: func type should be the same as arg type, so checks arg.
// arg is an APP: Typechecks -> func type should coincident with arg type, so checks arg:
// arg is a VARIABLE. Compare variable type with func arg. If they match, return the return type of the function (in this case, Bool because
// we have Bool->Bool). So we return the type Bool. (simply type.next on the func type)
// Recurse back to the first call to typecheck App. Func type is Nat but returned type of the arg is bool! ERROR! Typechecked failed.
// If succeeded, we would have a return type func.next which would be Nat
// how to typecheck a lambda expr? They should specify their param type, so what happens after in the body is not lambdas problem. We got nothing to do
// so @x : Nat. (x b : Nat)

void assert_(bool expr, char *error_msg) {
  if (expr) return;
  error(error_msg, __FILE__, __LINE__, __func__);
}

Type *typecheck(AstNode *expr, TypeEnv *env) {
  if(expr->type == VAR) {
    char *type = get_type_from_expr(expr);
    Type *t = create_type(type, NULL, expr);
    add_to_env(&env, t);
    return t;
  } else if (expr->type == APPLICATION) {
    // need to typecheck the func and the arg, we also have to "expect" some types eq to others
    Type *func_type = typecheck(expr->node.application->function, env);
    Type *arg_type = typecheck(expr->node.application->argument, env);

    assert_(type_equal(func_type, arg_type), "Type mismatch.");
    // TODO: This should return the return_type of func_type, for now we assume functions are always pure and return values w simple types
    return func_type;
  } else if (expr->type == LAMBDA_EXPR) {
    char *type = get_type_from_expr(expr);
    // TODO: Fix me for function types
    Type *t = create_type(type, NULL, expr);
    add_to_env(&env, t);
    return t;
  }
  return NULL;
}

bool type_equal(Type *a, Type *b) {
  if (a == NULL || b == NULL) {
    return false;
  }
  bool type_eql = strcmp(a->type, b->type) == 0;

  bool return_eql = false;
  if (a->return_type == NULL && b->return_type == NULL) {
    return_eql = true;
  } else if (a->return_type == NULL || b->return_type == NULL) {
    return_eql = false;
  } else {
    return_eql = strcmp(a->return_type, b->return_type) == 0;
  }
  return type_eql && return_eql;
}

char *get_type_from_expr(AstNode *expr) {
  if (expr->type == VAR) {
    return expr->node.variable->type;
  } else if (expr->type == LAMBDA_EXPR) {
    return expr->node.lambda_expr->type;
  }
  return NULL;
}

void p_print_type(Type *t) {
  if (t == NULL) {
    printf("(type null)\n");
    return;
  }
  if (t->type != NULL) {
    printf("Type: %s\n", t->type);
  }
  if (t->return_type != NULL) {
    printf("Return type: %s\n", t->return_type);
  }
}

Type *create_type(char *type, char *return_type, AstNode *expr) {
  Type *t = malloc(sizeof(Type));
  HANDLE_NULL(t);
  t->return_type = return_type;
  t->type = type;
  t->expr = expr;
  return t;
}

Type *parse_function_type(char *type) {
  Type *t = malloc(sizeof(Type));
  HANDLE_NULL(t);
  t->return_type = NULL;
  t->type = type;
  t->expr = NULL;
  return t;
}

bool expr_type_equal(Type *t, AstNode *expr) {
  if (t->expr != expr) {
    return false;
  }

  char *type = get_type_from_expr(expr);
  HANDLE_NULL(type);

  Type *parsed_type = parse_function_type(type);

  if (strcmp(t->type, parsed_type->type) != 0) {
    return false;
  }

  if (parsed_type->return_type == NULL) {
    if (t->return_type == NULL) return true;
    return false;
  }

  if (strcmp(t->return_type, parsed_type->return_type) != 0) {
    return false;
  }

  return true;
}

void add_to_env(TypeEnv **env, Type *type) {
  TypeEnv *new_env = malloc(sizeof(TypeEnv));
  HANDLE_NULL(new_env);
  new_env->type = type;
  new_env->next = *env;
  *env = new_env;
}

Type *lookup_type(TypeEnv *env, AstNode *expr) {
  while (env != NULL) {
    if (expr_type_equal(env->type, expr)) {
      return env->type;
    }
    env = env->next;
  }
  return NULL;
}
