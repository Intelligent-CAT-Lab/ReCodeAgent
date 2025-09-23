#ifndef EM_H_HEADER_GUARD
#define EM_H_HEADER_GUARD

#include <stdio.h>   /* fprintf */
#include <assert.h>  /* assert */
#include <string.h>  /* memset */
#include <stdlib.h>  /* malloc, realloc, free */
#include <stdbool.h> /* bool, true, false */

#include "data.h"
#include "utils.h"

typedef enum {
	EM_PUSH = 0,
	EM_POP,

	EM_ADD,
	EM_SUB,
	EM_MUL,
	EM_DIV,

	EM_GRT,
	EM_LESS,
	EM_EQU,
	EM_NEQU,

	EM_PRINT_BEGIN,
	EM_PRINT_END,

	EM_IF_BEGIN,
	EM_IF_END,

	EM_LOOP_BEGIN,
	EM_LOOP_END,

	EM_EXIT,

	EM_DUP,
	EM_SWAP,

#ifdef DEBUG
	EM_DEBUG,
#endif

	EM_TYPES_COUNT,
} em_type_t;

const char *em_type_to_cstr(em_type_t type);

#define DATA_STDOUT 1
#define DATA_STDERR 2

typedef struct {
	data_t    data;
	em_type_t type;

	const char *path;
	size_t      row, col;

	size_t ref;
	bool   ran;
} em_t;

em_t em_new(em_type_t type);
em_t em_new_with_data(em_type_t type, data_t data);

void em_fprintf(em_t *em, FILE *file);

typedef struct {
	em_t  *ems;
	size_t cap, size;
} program_t;

#define DEFAULT_PROGRAM_CAP 256

program_t program_new    (size_t cap);
void      program_destroy(program_t *prog);
void      program_push   (program_t *prog, em_t em);

#endif
