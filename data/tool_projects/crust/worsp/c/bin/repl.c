#include "worsp.h"
#include <stdio.h>
#include <stdlib.h>

void handleCtrlC() {
  printf("%s", "\nCtrl-C is pressed.");
  exit(0);
}

int main() {
  signal(SIGINT, handleCtrlC);

  struct Env env = (struct Env){};
  initEnv(&env);

  struct AllocatorContext *context = initAllocator();
  // context->gc_less_mode = 1;

  while (1) {
    char input[1024];
    printf("%s", "> ");

    if (scanf("%[^\n]%*c", input) == EOF) {
      printf("\nAbort .\n");
      break;
    }

    struct ParseState state = (struct ParseState){NULL, 0};
    struct ParseResult *result = malloc(sizeof(struct ParseResult));
    struct Object *evaluated = allocate(context, &env);
    parse(input, &state, result);
    evaluateExpression(result->program->expressions->expression, evaluated,
                       &env, context);

    char *stringified = stringifyObject(evaluated);

    printf("%s\n", stringified);

    free(stringified);
  }

  return 0;
}
