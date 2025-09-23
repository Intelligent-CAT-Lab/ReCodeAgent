#ifndef ENV_H_HEADER_GUARD
#define ENV_H_HEADER_GUARD

#include <string.h>  /* memset */
#include <assert.h>  /* assert */
#include <stdint.h>  /* int64_t */
#include <stdio.h>   /* fputc, fflush */
#include <stdbool.h> /* bool, true, false */

#include "em.h"
#include "utils.h"
#include "stack.h"

#ifndef GC_FREQUENCY_IN_TICKS
#	define GC_FREQUENCY_IN_TICKS 64
#endif

typedef enum {
	RUNTIME_OK = 0,

	RUNTIME_ERR_STACK_UNDERFLOW,
	RUNTIME_ERR_INVALID_ACCESS,
	RUNTIME_ERR_DIV_BY_ZERO,
	RUNTIME_ERR_INCORRECT_TYPE,

	RUNTIME_ERRS_COUNT,
} runtime_err_t;

const char *runtime_err_to_cstr(runtime_err_t err);

typedef struct {
	runtime_err_t err;
	int64_t       ex;
	em_t         *em;
} runtime_result_t;

runtime_result_t runtime_result_ok (int64_t ex);
runtime_result_t runtime_result_err(runtime_err_t err, em_t *em);

typedef struct {
	program_t *prog;
	stack_t    stack;

	size_t ip, ex, tick;
	bool   halt;

	bool   print;
	size_t print_from;
} env_t;

env_t *env_new    (size_t stack_cap, size_t popped_cap);
void   env_destroy(env_t *e);

runtime_result_t env_run(env_t *e, program_t *prog);

#endif
