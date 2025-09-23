#ifndef DATA_H_HEADER_GUARD
#define DATA_H_HEADER_GUARD

#include <stdint.h> /* int64_t */
#include <stdio.h>  /* fprintf */
#include <assert.h> /* assert */
#include <stdlib.h> /* free */

typedef enum {
	DATA_INT = 0,
	DATA_STR,

	DATA_TYPES_COUNT,
} data_type_t;

const char *data_type_to_cstr(data_type_t type);

typedef struct {
	data_type_t type;
	union {
		int64_t int_;
		char   *str;
	} as;
} data_t;

void data_fprintf(data_t *data, FILE *file);

data_t data_new(data_type_t type);
data_t data_new_int(int64_t val);
data_t data_new_str(char   *val);

#endif
