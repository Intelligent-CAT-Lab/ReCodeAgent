#include "em.h"

static const char *em_type_to_cstr_map[EM_TYPES_COUNT] = {
	[EM_PUSH] = "push",
	[EM_POP]  = "pop",

	[EM_ADD] = "add",
	[EM_SUB] = "sub",
	[EM_MUL] = "mul",
	[EM_DIV] = "div",

	[EM_GRT]  = "grt",
	[EM_LESS] = "less",
	[EM_EQU]  = "equ",
	[EM_NEQU] = "nequ",

	[EM_PRINT_BEGIN] = "print_begin",
	[EM_PRINT_END]   = "print_end",

	[EM_IF_BEGIN] = "if_begin",
	[EM_IF_END]   = "if_end",

	[EM_LOOP_BEGIN] = "loop_begin",
	[EM_LOOP_END]   = "loop_end",

	[EM_EXIT] = "exit",

	[EM_DUP]  = "dup",
	[EM_SWAP] = "swap",

#ifdef DEBUG
	[EM_DEBUG] = "debug",
#endif
};

const char *em_type_to_cstr(em_type_t type) {
	assert(type < EM_TYPES_COUNT && type >= 0);
	return em_type_to_cstr_map[type];
}

em_t em_new(em_type_t type) {
	return (em_t){.type = type};
}

em_t em_new_with_data(em_type_t type, data_t data) {
	return (em_t){.data = data, .type = type};
}

void em_fprintf(em_t *em, FILE *file) {
	assert(em   != NULL);
	assert(file != NULL);

	fprintf(file, "<%s", em_type_to_cstr(em->type));

	switch (em->type) {
	case EM_PUSH:
		fprintf(file, " ");
		data_fprintf(&em->data, file);
		break;

	case EM_PRINT_END:
		fprintf(file, " %s", em->data.as.int_ == DATA_STDOUT? "stdout" : "stderr");
		break;

	case EM_PRINT_BEGIN: case EM_IF_BEGIN:
		fprintf(file, " ref: %zu", em->ref);
		break;

	default: break;
	}

	fprintf(file, " %s:%zu:%zu>\n", em->path, em->row, em->col);
}

program_t program_new(size_t cap) {
	assert(cap > 0);

	program_t prog = {0};
	prog.cap = cap;
	prog.ems = (em_t*)malloc(prog.cap * sizeof(em_t));
	assert(prog.ems != NULL);
	return prog;
}

void program_destroy(program_t *prog) {
	assert(prog != NULL);
	assert(prog->ems != NULL);

	free(prog->ems);
}

void program_push(program_t *prog, em_t em) {
	assert(prog != NULL);

	if (prog->size >= prog->cap) {
		prog->cap *= 2;
		prog->ems  = (em_t*)realloc(prog->ems, prog->cap * sizeof(em_t));
		assert(prog->ems != NULL);
	}

	prog->ems[prog->size ++] = em;
}
