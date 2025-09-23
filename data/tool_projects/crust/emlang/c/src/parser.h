#ifndef PARSER_H_HEADER_GUARD
#define PARSER_H_HEADER_GUARD

#include <stdio.h>   /* fopen, fclose, fseek, rewind, ftell, SEEK_END */
#include <string.h>  /* memset, strlen, strcpy, strcmp */
#include <assert.h>  /* assert */
#include <ctype.h>   /* isspace, isdigit */
#include <stdbool.h> /* bool, true, false */

#include "em.h"
#include "utils.h"

typedef enum {
	PARSER_OK = 0,

	PARSER_ERR_UNEXPECTED_ESCAPE,
	PARSER_ERR_UNKNOWN_ESCAPE,
	PARSER_ERR_UNTERMINATED_QUOTES,
	PARSER_ERR_UNEXPECTED_END,
	PARSER_ERR_ILLEGAL_PRINT_NEST,
	PARSER_ERR_EXPECTED_END,

	PARSER_ERRS_COUNT,
} parser_err_t;

const char *parser_err_to_cstr(parser_err_t err);

typedef struct {
	parser_err_t err;
	bool         syntax_err;

	const char *path;
	size_t      row, col;

	program_t prog;
} parser_result_t;

parser_result_t parser_ok(void);
parser_result_t parser_err(parser_err_t err, const char *path, size_t row, size_t col);

#define PARSER_MAX_TOKEN_LENGTH 1024

typedef struct {
	const char *path;
	size_t      row, col;

	bool   from_file;
	char  *in;
	int    ch;
	size_t pos;

	char   tok[PARSER_MAX_TOKEN_LENGTH];
	size_t tok_len;

	program_t prog;
} parser_t;

parser_t *parser_new    (size_t prog_cap);
void      parser_destroy(parser_t *p);

void parser_load_mem (parser_t *p, const char *in);
int  parser_load_file(parser_t *p, const char *path);

parser_result_t parser_parse(parser_t *p);

#endif
