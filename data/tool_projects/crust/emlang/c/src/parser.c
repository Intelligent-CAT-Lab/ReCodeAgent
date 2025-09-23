#include "parser.h"

static const char *parser_err_to_cstr_map[PARSER_ERRS_COUNT] = {
	[PARSER_OK] = "Ok",

	[PARSER_ERR_UNEXPECTED_ESCAPE]   = "Unexpected escape",
	[PARSER_ERR_UNKNOWN_ESCAPE]      = "Unknown escape",
	[PARSER_ERR_UNTERMINATED_QUOTES] = "Unterminated quotes",
	[PARSER_ERR_UNEXPECTED_END]      = "Unexpected end",
	[PARSER_ERR_ILLEGAL_PRINT_NEST]  = "Illegal print nesting",
	[PARSER_ERR_EXPECTED_END]        = "Expected matching end",
};

const char *parser_err_to_cstr(parser_err_t err) {
	assert(err < PARSER_ERRS_COUNT && err >= 0);
	return parser_err_to_cstr_map[err];
}

parser_result_t parser_ok(void) {
	return (parser_result_t){.err = PARSER_OK};
}

parser_result_t parser_err(parser_err_t err, const char *path, size_t row, size_t col) {
	return (parser_result_t){.err = err, .path = path, .row = row, .col = col};
}

parser_t *parser_new(size_t prog_cap) {
	parser_t *p = (parser_t*)malloc(sizeof(parser_t));
	assert(p != NULL);
	ZERO_STRUCT(p);

	p->row  = 1;
	p->prog = program_new(prog_cap);
	return p;
}

void parser_load_mem(parser_t *p, const char *in) {
	p->in = (char*)in;
}

int parser_load_file(parser_t *p, const char *path) {
	assert(path != NULL);

	p->from_file = true;
	p->path      = path;
	FILE *file   = fopen(p->path, "r");
	if (file == NULL)
		return -1;

	fseek(file, 0, SEEK_END);
	size_t size = (size_t)ftell(file);
	rewind(file);

	p->in = (char*)malloc(size + 1);
	assert(p->in != NULL);

	if (size > 0)
		assert(fread(p->in, size, 1, file) > 0);

	p->in[size] = '\0';

	fclose(file);
	return 0;
}

void parser_destroy(parser_t *p) {
	assert(p != NULL);

	if (p->from_file) {
		assert(p->in != NULL);
		free(p->in);
	}

	free(p);
}

#define PARSER_END(P) ((P)->ch == '\0')

#define PARSER_TOK_ADD(P, CH) \
	(assert((P)->tok_len < PARSER_MAX_TOKEN_LENGTH), (P)->tok[(P)->tok_len ++] = CH)

#define PARSER_TOK_CLEAR(P)     ((P)->tok_len = 0)
#define PARSER_TOK_NULL_TERM(P) ((P)->tok[(P)->tok_len] = '\0')

#define EXPAND_LOCATION(STRUCT) (STRUCT)->path, (STRUCT)->row, (STRUCT)->col

static void parser_advance(parser_t *p) {
	if (p->ch == '\n') {
		++ p->row;
		p->col = 0;
	}

	p->ch = p->in[p->pos ++];
	if (PARSER_END(p))
		return;

	++ p->col;
}

static parser_result_t parser_parse_quotes(parser_t *p) {
	PARSER_TOK_CLEAR(p);
	size_t start_row = p->row, start_col = p->col;

	bool escape = false;
	while (true) {
		parser_advance(p);
		if (PARSER_END(p) || p->ch == '\n')
			return parser_err(PARSER_ERR_UNTERMINATED_QUOTES, p->path, start_row, start_col);

		if (escape) {
			switch (p->ch) {
			case 'n':  PARSER_TOK_ADD(p, '\n'); break;
			case 'r':  PARSER_TOK_ADD(p, '\r'); break;
			case 't':  PARSER_TOK_ADD(p, '\t'); break;
			case 'f':  PARSER_TOK_ADD(p, '\f'); break;
			case 'v':  PARSER_TOK_ADD(p, '\v'); break;
			case 'b':  PARSER_TOK_ADD(p, '\b'); break;
			case 'a':  PARSER_TOK_ADD(p, '\a'); break;
			case '"':  PARSER_TOK_ADD(p, '"');  break;
			case 'e':  PARSER_TOK_ADD(p, 27);   break;
			case '\\': PARSER_TOK_ADD(p, '\\'); break;

			default: return parser_err(PARSER_ERR_UNKNOWN_ESCAPE, EXPAND_LOCATION(p));
			}

			escape = false;
		} else if (p->ch == '\\')
			escape = true;
		else if (p->ch == '"')
			break;
		else
			PARSER_TOK_ADD(p, p->ch);
	}
	parser_advance(p);

	PARSER_TOK_NULL_TERM(p);
	char *str = strcpy_to_heap(p->tok);
	assert(str != NULL);

	em_t em = em_new_with_data(EM_PUSH, data_new_str(str));
	em.row  = start_row;
	em.col  = start_col;
	em.path = p->path;

	program_push(&p->prog, em);
	return parser_ok();
}

static const char *em_to_keyword_map[EM_TYPES_COUNT] = {
	[EM_PUSH] = NULL,
	[EM_POP]  = ":P",

	[EM_ADD] = ";)",
	[EM_SUB] = ";(",
	[EM_MUL] = "x)",
	[EM_DIV] = "x(",

	[EM_GRT]  = ":>",
	[EM_LESS] = ":<",
	[EM_EQU]  = ":|",
	[EM_NEQU] = "x|",

	[EM_PRINT_BEGIN] = ":O",
	[EM_PRINT_END]   = NULL,

	[EM_IF_BEGIN] = ":/",
	[EM_IF_END]   = ":\\",

	[EM_LOOP_BEGIN] = ":@",
	[EM_LOOP_END]   = "@:",

	[EM_EXIT] = "X_X",

	[EM_DUP]  = ":D",
	[EM_SWAP] = ":S",

#ifdef DEBUG
	[EM_DEBUG] = "D:",
#endif
};

static parser_result_t parser_parse_plain(parser_t *p) {
	PARSER_TOK_CLEAR(p);
	size_t start_row = p->row, start_col = p->col;

	if (p->ch == '\\') {
		parser_advance(p);
		if (PARSER_END(p) || isspace(p->ch))
			return parser_err(PARSER_ERR_UNEXPECTED_ESCAPE, p->path, start_row, start_col);
		else if (p->ch != '"')
			PARSER_TOK_ADD(p, '\\');
	}

	bool is_int = true;
	do {
		if (is_int && !(p->tok_len == 0 && p->ch == '-')) {
			if (!isdigit(p->ch))
				is_int = false;
		}

		PARSER_TOK_ADD(p, p->ch);

		parser_advance(p);
		if (PARSER_END(p))
			return parser_ok();
	} while (!isspace(p->ch));

	if (p->tok_len == 1 && p->tok[0] == '-')
		is_int = false;

	PARSER_TOK_NULL_TERM(p);

	em_t em;
	for (size_t i = 0; i < EM_TYPES_COUNT; ++ i) {
		if (em_to_keyword_map[i] == NULL)
			continue;

		if (strcmp(p->tok, em_to_keyword_map[i]) == 0) {
			em = em_new((em_type_t)i);
			goto push;
		}
	}

	if (strcmp(p->tok, ":x") == 0) {
		while (!PARSER_END(p) && p->ch != '\n')
			parser_advance(p);

		return parser_ok();
	} else if (strcmp(p->tok, ":)") == 0)
		em = em_new_with_data(EM_PRINT_END, data_new_int(DATA_STDOUT));
	else if (strcmp(p->tok, ":(") == 0)
		em = em_new_with_data(EM_PRINT_END, data_new_int(DATA_STDERR));
	else if (strcmp(p->tok, ":3")  == 0 || strcmp(p->tok, ";3") == 0 ||
	         strcmp(p->tok, "<3")  == 0 || strcmp(p->tok, "x3") == 0 ||
	         strcmp(p->tok, "><>") == 0) {
		const char *text;
		switch (p->tok[0]) {
		case ':': text = "meow";        break;
		case ';': text = "nya";         break;
		case 'x': text = "rawr";        break;
		case '>': text = "le fishe";    break;
		case '<': text = "i <3 emlang"; break;

		default: assert(0);
		}

		char *str = strcpy_to_heap(text);
		assert(str != NULL);

		em = em_new_with_data(EM_PUSH, data_new_str(str));
	} else if (is_int)
		em = em_new_with_data(EM_PUSH, data_new_int((int64_t)atoll(p->tok)));
	else {
		char *str = strcpy_to_heap(p->tok);
		assert(str != NULL);

		em = em_new_with_data(EM_PUSH, data_new_str(str));
	}

push:
	em.row  = start_row;
	em.col  = start_col;
	em.path = p->path;
	program_push(&p->prog, em);
	return parser_ok();
}

static parser_result_t parser_parse_next(parser_t *p) {
	while (isspace(p->ch)) {
		parser_advance(p);
		if (PARSER_END(p))
			return parser_ok();
	}

	if (p->ch == '"')
		return parser_parse_quotes(p);
	else
		return parser_parse_plain(p);
}

#define PARSER_MAX_NESTS 256

static parser_result_t parser_cross_ref(parser_t *p) {
	em_type_t expects[PARSER_MAX_NESTS];
	size_t    begins [PARSER_MAX_NESTS];
	size_t    nest = 0;

	bool print = false;
	for (size_t i = 0; i < p->prog.size; ++ i) {
		em_t *em = &p->prog.ems[i];
		switch (em->type) {
		case EM_PRINT_BEGIN:
			if (print)
				return parser_err(PARSER_ERR_ILLEGAL_PRINT_NEST, EXPAND_LOCATION(em));
			print = true;
			/* Fallthrough */

		case EM_IF_BEGIN: case EM_LOOP_BEGIN:
			expects[nest]   = em->type + 1; /* Assuming the end type is right after the begin type */
			begins[nest ++] = i;
			break;

		case EM_PRINT_END:
			print = false;
			/* Fallthrough */

		case EM_IF_END: case EM_LOOP_END:
			if (nest == 0)
				return parser_err(PARSER_ERR_UNEXPECTED_END, EXPAND_LOCATION(em));
			else if (em->type != expects[nest - 1])
				return parser_err(PARSER_ERR_UNEXPECTED_END, EXPAND_LOCATION(em));

			size_t begin = begins[-- nest];
			p->prog.ems[begin].ref = i;
			em->ref                = begin;
			break;

		default: break;
		}
	}

	if (nest != 0)
		return parser_err(PARSER_ERR_EXPECTED_END, EXPAND_LOCATION(&p->prog.ems[begins[nest - 1]]));

	return parser_ok();
}

parser_result_t parser_parse(parser_t *p) {
	parser_advance(p);
	if (PARSER_END(p)) {
		parser_result_t result = parser_ok();
		result.prog = p->prog;
		return result;
	}

	parser_result_t result;
	do
		result = parser_parse_next(p);
	while (result.err == PARSER_OK && !PARSER_END(p));

	if (result.err != PARSER_OK)
		return result;

	result = parser_cross_ref(p);
	if (result.err != PARSER_OK)
		return result;

	result.prog = p->prog;
	return result;
}
