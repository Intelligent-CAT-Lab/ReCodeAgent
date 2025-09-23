#include "worsp.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc != 2) {
    printf("filepath is required.\n");
    return 1;
  }

  char *filepath = argv[1];

  FILE *file =
      fopen(filepath, "rb"); // "rb" はバイナリモードで読み込むことを指定

  if (file == NULL) {
    perror("Cannot open file");
    return 1;
  }

  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  fseek(file, 0, SEEK_SET);

  char *file_contents = (char *)malloc(file_size + 1);
  if (file_contents == NULL) {
    perror("Failed to malloc");
    fclose(file);
    return 1;
  }

  fread(file_contents, 1, file_size, file);
  file_contents[file_size] = '\0';

  fclose(file);

  struct ParseState state = (struct ParseState){NULL, 0};
  struct ParseResult *result = malloc(sizeof(struct ParseResult));

  parse(file_contents, &state, result);
  evaluate(result);

  free(file_contents);
  free(result);

  return 0;
}
