#include "config.h"
#include "common.h"
#include "io.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

void trim(char *str) {
  int start = 0, end = strlen(str) - 1;

  while (isspace(str[start])) {
    start++;
  }

  while (end > start && isspace(str[end])) {
    end--;
  }

  if (start > 0 || end < (strlen(str) - 1)) {
    memmove(str, str + start, end - start + 1);
    str[end - start + 1] = '\0';
  }
}

option_type_t get_config_type(char *key) {
  if (strcmp(key, "file") == 0) {
    return FILENAME;
  } else if (strcmp(key, "step_by_step_reduction") == 0) {
    return STEP_REDUCTION;
  } else if (strcmp(key, "reduction_order") == 0) {
    return REDUCTION_ORDER;
  } else {
    const char *error_msg = format("ERROR: Invalid key '%s' at config file.", key);
    error(error_msg, __FILE__, __LINE__, __func__);
  }
  return CONFIG_ERROR;
}

void parse_config(char *line, char **key, char **value) {
  char *eq_pos = strchr(line, '=');
  if (eq_pos == NULL) {
    const char *error_msg = format("Malformed config file at line: %s . Expected = sign.\n",
                line);
    error(error_msg, __FILE__, __LINE__, __func__);
  }
  *key = strtok(line, "=");
  trim(*key);
  *value = strtok(NULL, "=");
  trim(*value);
}

Options get_config_from_file() {
  // config file format is like env: key=value
  char *line = NULL;
  size_t len = 0;
  FILE *config_file = get_file(CONFIG_PATH, "r");

  Options options;
  options.reduction_order = APPLICATIVE;
  options.step_by_step_reduction = false;
  options.file = NULL;

  while (getline(&line, &len, config_file) != -1) {
    char *key, *value;
    parse_config(line, &key, &value);
    option_type_t cfg = get_config_type(key);

    switch (cfg) {
    case FILENAME: {
      options.file = get_file(value, "r");
      break;
    }
    case STEP_REDUCTION: {
      if (strcmp(value, "true") == 0) {
        options.step_by_step_reduction = true;
        break;
      }
      options.step_by_step_reduction = false;
      break;
    }
    case REDUCTION_ORDER: {
      if (strcmp(value, "applicative") == 0) {
        options.reduction_order = APPLICATIVE;
        break;
      } else if (strcmp(value, "normal") == 0) {
        options.reduction_order = NORMAL;
        break;
      } else {
        const char *error_msg = "ERROR: reduction order in cfg file should be 'normal' "
                                "or 'applicative'.";
        error(error_msg, __FILE__, __LINE__, __func__);
      }
    }
    case CONFIG_ERROR: {
      const char *error_msg = format("Unrecognized key: %s", key);
      error(error_msg, __FILE__, __LINE__, __func__);
    }
    }
  }
  if (options.file == NULL) {
    const char *error_msg = "ERROR: File cannot be null in cfg file.\n";
    error(error_msg, __FILE__, __LINE__, __func__);
  }
  return options;
}
