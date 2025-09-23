#ifndef CONFIG_H
#define CONFIG_H

#include <stdbool.h>
#include <stdio.h>

#define CONFIG_PATH "config"

typedef enum {
  APPLICATIVE, // always fully evaluate the arguments of a function before
               // evaluating the function itself
  NORMAL,      // reduction happens from the outside in
} reduction_order_t;

typedef enum {
  FILENAME,
  STEP_REDUCTION,
  REDUCTION_ORDER,
  CONFIG_ERROR,
} option_type_t;

typedef struct {
  FILE *file;
  bool step_by_step_reduction;
  reduction_order_t reduction_order;
} Options;

void trim(char *str);

option_type_t get_config_type(char *key);

void parse_config(char *line, char **key, char **value);

Options get_config_from_file();


#endif
