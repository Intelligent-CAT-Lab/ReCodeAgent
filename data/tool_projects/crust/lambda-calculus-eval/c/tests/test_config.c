#include "../src/config.h"
#include "tests.h"
#include <string.h>
#include <assert.h>

void test_trim() {
  char str_to_trim[] = "  test  ";
  trim(str_to_trim);
  assert(strcmp(str_to_trim, "test") == 0);
}

void test_get_config_type() {
  option_type_t file_opt = get_config_type("file");
  option_type_t step_opt = get_config_type("step_by_step_reduction");
  option_type_t reduction = get_config_type("reduction_order");

  assert(file_opt == FILENAME);
  assert(step_opt == STEP_REDUCTION);
  assert(reduction == REDUCTION_ORDER);
}

void test_parse_config() {
  char line[100];
  char *key, *value;

  strcpy(line, "file=expr.lambda");
  parse_config(line, &key, &value);
  assert(strcmp(key, "file") == 0);
  assert(strcmp(value, "expr.lambda") == 0);
}

int main() {
    printf("Test config ...\t");
    test_trim();
    test_get_config_type();
    test_parse_config();
    print_green("PASSED\n");
    return 0;
}
