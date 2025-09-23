#include "io.h"
#include "common.h"
#include <stdio.h>
#include <stdlib.h>

FILE *create_file(char *path) {
  FILE *file = fopen(path, "w");
  if (file == NULL) {
    const char *error_msg = format("ERROR: Could not create file %s\n", path);
    error(error_msg, __FILE__, __LINE__, __func__);
  }
  return file;
}

FILE *get_file(char *path, char *mode) {
  FILE *file = fopen(path, mode);
  if (file == NULL) {
    const char *error_msg = format("ERROR: Could not open file %s\n", path);
    error(error_msg, __FILE__, __LINE__, __func__);
  }
  return file;
}

void write_to_file(FILE *file, char *content) {
  HANDLE_NULL(file);
  if (fputs(content, file) == EOF) {
    const char *error_msg = format("ERROR: could not write to file at %s:%d\n", __FILE__,
                __LINE__);
    error(error_msg, __FILE__, __LINE__, __func__);
  }
  rewind(file);
}

void delete_file(char *path) {
  int status = remove(path);
  if (status != 0) {
    const char *error_msg = format("ERROR: could not delete to file at %s:%d\n", __FILE__,
                __LINE__);
    error(error_msg, __FILE__, __LINE__, __func__);
  }
}

void close_file(FILE *file) {
  if (fclose(file) != 0) {
    const char *error_msg = format("ERROR: Could not close file at %s:%d\n", __FILE__,
                __LINE__);
    error(error_msg, __FILE__, __LINE__, __func__);
  }
}

char next(FILE *file) {
  int x = fgetc(file);
  return (char)x;
}
