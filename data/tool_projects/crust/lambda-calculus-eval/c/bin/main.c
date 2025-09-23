#include "../hash-table/hash_table.h"
#include "../src/common.h"
#include "../src/config.h"
#include "../src/io.h"
#include "../src/parser.h"
#include "../src/reducer.h"
#include "../src/typechecker.h"
#include <stdio.h>

#define MAX_LAMBDA_LENGTH 1000
#define TMP_PATH "tmp/expr.lambda"

char *get_str_input(char *input_msg, char *error_msg) {
  char *input = malloc(MAX_LAMBDA_LENGTH + 1 * sizeof(char));
  HANDLE_NULL(input);

  printf("%s", input_msg);

  if (fgets(input, MAX_LAMBDA_LENGTH + 1, stdin) == NULL) {
    free(input);
    error(error_msg, __FILE__, __LINE__, __func__);
  }

  size_t len = strlen(input);
  if (len > 0 && input[len - 1] == '\n') {
    input[len - 1] = '\0';
  }

  return input;
}

char *get_lambda_expr_input() {
  char *lambda_expr =
      get_str_input("Enter the lambda expression: ",
                    "failed to read the lambda expression.");

  FILE *file = create_file(TMP_PATH);
  write_to_file(file, lambda_expr);
  close_file(file);
  return TMP_PATH;
}

char *get_file_path_input() {
  char *file_path = get_str_input("Enter the file path for your expression: ",
                                  "failed to read file path.");
  return file_path;
}

bool get_reduction_step_option() {
  int option;
  while (true) {
    printf("Choose: \n[1] Normal reduction\n[2] Step by step reduction\n");
    scanf("%d", &option);
    while (getchar() != '\n')
      ;
    switch (option) {
    case 1: {
      return false;
    }
    case 2: {
      return true;
    }
    default:
      return false;
    }
  }
}

reduction_order_t get_reduction_order() {
  int option;
  while (true) {
    printf("Choose: \n[1] Normal order\n[2] Applicative Order\n");
    scanf("%d", &option);
    while (getchar() != '\n')
      ;
    switch (option) {
    case 1: {
      return NORMAL;
    }
    case 2: {
      return APPLICATIVE;
    }
    }
  }
}

bool exists(char *path) {
  FILE *file;
  if ((file = fopen(path, "r")) != NULL) {
    fclose(file);
    return true;
  }
  return false;
}


Options cli() {
  FILE *f;
  int option;
  bool valid_option = false;
  bool step_by_step_reduction = get_reduction_step_option();
  reduction_order_t reduction_order = get_reduction_order();

  while (!valid_option) {

    printf("Choose your option:\n[1] Write input\n[2] File input\n");

    scanf("%d", &option);

    while (getchar() != '\n')
      ;
    switch (option) {
    case 1: {
      char *path = get_lambda_expr_input();
      f = get_file(path, "r");
      valid_option = true;
      break;
    }
    case 2: {
      char *path = get_file_path_input();
      f = get_file(path, "r");
      free(path);
      valid_option = true;
      break;
    }
    default:
      printf("Invalid input. Try again\n");
      break;
    }
  }
  Options opt;
  opt.file = f;
  opt.step_by_step_reduction = step_by_step_reduction;
  opt.reduction_order = reduction_order;
  return opt;
}

int main(void) {
  Options opt;
  if (exists(CONFIG_PATH)) {
    opt = get_config_from_file();
  } else {
    opt = cli();
  }
  set_verbose(opt.step_by_step_reduction);
  set_reduction_order(opt.reduction_order);
  FILE *in = opt.file;
  HANDLE_NULL(in);
  HashTable *table = createHashTable();

  AstNode *parsed = parse_expression(table, in);
  printf("\nPARSED AST: \n");
  print_ast(parsed);
  print_verbose("\nTYPECHECKING --------------------------------------------------------------\n");
  typecheck(parsed, NULL);
  print_verbose("TYPECHECKING DONE! PROCEEDING TO REDUCE.");

  AstNode *reduced = reduce(table, parsed);
  printf("\nReduced ast:\n");
  char *reduced_ast_str = ast_to_string(reduced);
  printf("%s\n", reduced_ast_str);

  destroyHashTable(table);
  close_file(in);
  return 1;
}

// grammar
// Expression ::= LambdaExpression | Application | Variable
// LambdaExpression ::= "@" Identifier "." Expression
// Application ::= "(" Expression Expression ")"
// Variable ::= Identifier
