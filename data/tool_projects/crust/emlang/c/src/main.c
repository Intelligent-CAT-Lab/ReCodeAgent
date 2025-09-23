#include <stdio.h>  /* stderr, fprintf, printf */
#include <stdlib.h> /* exit, EXIT_FAILURE, EXIT_SUCCESS */
#include <string.h> /* strcmp */

#include "parser.h"
#include "env.h"

program_t parse(const char *path) {
	parser_t       *p = parser_new(DEFAULT_PROGRAM_CAP);
	parser_result_t result;

	if (parser_load_file(p, path) != 0) {
		fprintf(stderr, "Error: Failed to open file '%s'\n", path);
		exit(EXIT_FAILURE);
	}

	result = parser_parse(p);
	if (result.err != PARSER_OK) {
		fprintf(stderr, "Error at %s:%zu:%zu: %s\n",
		        result.path, result.row, result.col, parser_err_to_cstr(result.err));
		exit(EXIT_FAILURE);
	}

	parser_destroy(p);
	return result.prog;
}

void usage(const char *path) {
	printf(":O emlang :)\n"
	       "https://github.com/lordoftrident/emlang\n\n"
	       "Usage: %s FILE | OPTIONS\n"
	       "Options:\n"
	       "  -h, --help    Show the usage\n", path);
}

int main(int argc, const char **argv) {
	if (argc != 2) {
		fprintf(stderr, "Error: No file provided\n");
		fprintf(stderr, "Try '%s -h'\n", argv[0]);
		return EXIT_FAILURE;
	} else if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0) {
		usage(argv[0]);
		return EXIT_SUCCESS;
	}

	program_t prog = parse(argv[1]);

#ifdef DEBUG
	for (size_t i = 0; i < prog.size; ++ i)
		em_fprintf(&prog.ems[i], stdout);
#endif

	env_t *e = env_new(DEFAULT_STACK_CAP, DEFAULT_POPPED_CAP);

	runtime_result_t result = env_run(e, &prog);
	if (result.err != RUNTIME_OK) {
		fprintf(stderr, "Error at %s:%zu:%zu: %s\n",
		        result.em->path, result.em->row, result.em->col,
		        runtime_err_to_cstr(result.err));
		exit(EXIT_FAILURE);
	}

	env_destroy(e);
	program_destroy(&prog);
	return result.ex;
}
