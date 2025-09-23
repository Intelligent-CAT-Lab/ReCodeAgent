
#include "../src/io.h"
#include "../src/common.h"
#include "tests.h"
#include <stdio.h>
#include <string.h>
#include <assert.h>
#include <unistd.h>

#define TEMP_PATH "temp/test.txt"

void test_create_file() {
  FILE *test = create_file(TEMP_PATH);
  assert(access(TEMP_PATH, F_OK) == 0);
  fclose(test);
  remove(TEMP_PATH);
}

void test_get_file() {
  FILE *c = create_file(TEMP_PATH);
  FILE *d = get_file(TEMP_PATH, "r+");

  const char *test_str = "Hello";
  fprintf(c, "%s", test_str);
  fflush(c);
  fseek(d, 0, SEEK_SET);

  char buffer[100];
  fgets(buffer, sizeof(buffer), d);

  assert(strcmp(buffer, test_str) == 0);

  delete_file(TEMP_PATH);
  fclose(c);
  fclose(d);
}

void test_file_deletion() {
  FILE *test = create_file(TEMP_PATH);
  assert(access(TEMP_PATH, F_OK) == 0);
  delete_file(TEMP_PATH);
  assert(access(TEMP_PATH, F_OK) != 0);
  close_file(test);
}

int main() {
    printf("Test IO ...\t");
    test_create_file();
    test_get_file();
    test_file_deletion();
    print_green("PASSED\n");
    return 0;
}
