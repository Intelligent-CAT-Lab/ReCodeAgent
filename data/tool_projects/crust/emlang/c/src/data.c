#include "data.h"

static const char *data_type_to_cstr_map[DATA_TYPES_COUNT] = {
	[DATA_INT] = "int",
	[DATA_STR] = "str",
};

const char *data_type_to_cstr(data_type_t type) {
	assert(type < DATA_TYPES_COUNT && type >= 0);
	return data_type_to_cstr_map[type];
}

#ifdef DEBUG
void data_fprintf(data_t *data, FILE *file) {
	assert(data != NULL);
	assert(file != NULL);

	fprintf(file ,"[%s ", data_type_to_cstr(data->type));
	switch (data->type) {
	case DATA_INT: fprintf(file, "%i",   (int)data->as.int_); break;
	case DATA_STR: fprintf(file, "'%s'", data->as.str);       break;

	default: assert(0);
	}
	fprintf(file ,"]");
}
#else
void data_fprintf(data_t *data, FILE *file) {
	assert(data != NULL);
	assert(file != NULL);

	switch (data->type) {
	case DATA_INT: fprintf(file, "%i", (int)data->as.int_); break;
	case DATA_STR: fprintf(file, "%s", data->as.str);       break;

	default: assert(0);
	}
}
#endif

data_t data_new(data_type_t type) {
	return (data_t){.type = type};
}

data_t data_new_int(int64_t val) {
	return (data_t){.as = {.int_ = val}, .type = DATA_INT};
}

data_t data_new_str(char *val) {
	assert(val != NULL);
	return (data_t){.as = {.str = val}, .type = DATA_STR};
}
