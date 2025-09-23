#include "env.h"

const char *runtime_err_to_cstr_map[RUNTIME_ERRS_COUNT] = {
	[RUNTIME_OK] = "Ok",

	[RUNTIME_ERR_STACK_UNDERFLOW] = "Stack underflow",
	[RUNTIME_ERR_INVALID_ACCESS]  = "Invalid access",
	[RUNTIME_ERR_DIV_BY_ZERO]     = "Division by zero",

	[RUNTIME_ERR_INCORRECT_TYPE] = "Incorrect type",
};

const char *runtime_err_to_cstr(runtime_err_t err) {
	assert(err < RUNTIME_ERRS_COUNT && err >= 0);
	return runtime_err_to_cstr_map[err];
}

runtime_result_t runtime_result_ok(int64_t ex) {
	return (runtime_result_t){.ex = ex, .err = RUNTIME_OK};
}

runtime_result_t runtime_result_err(runtime_err_t err, em_t *em) {
	return (runtime_result_t){.err = err, .em = em};
}

env_t *env_new(size_t stack_cap, size_t popped_cap) {
	env_t *e = (env_t*)malloc(sizeof(env_t));
	assert(e != NULL);
	ZERO_STRUCT(e);

	e->stack = stack_new(stack_cap, popped_cap);
	return e;
}

void env_destroy(env_t *e) {
	stack_destroy(&e->stack);
	free(e);
}

static void env_gc(env_t *e) {
	for (size_t i = 0; i < e->prog->size; ++ i) {
		if (e->prog->ems[i].ran)
			continue;

		if (e->prog->ems[i].data.type == DATA_STR)
			free(e->prog->ems[i].data.as.str);
	}
}

#define STACK_POP(E, RET) \
	if (stack_pop(&(E)->stack, RET) != 0) \
		return runtime_result_err(RUNTIME_ERR_STACK_UNDERFLOW, &(E)->prog->ems[(E)->ip])

#define STACK_DUP(E, OFF) \
	if (stack_dup(&(E)->stack, OFF) != 0) \
		return runtime_result_err(RUNTIME_ERR_INVALID_ACCESS, &(E)->prog->ems[(E)->ip])

#define STACK_SWAP(E, OFF) \
	if (stack_swap(&(E)->stack, OFF) != 0) \
		return runtime_result_err(RUNTIME_ERR_INVALID_ACCESS, &(E)->prog->ems[(E)->ip])

runtime_result_t env_run(env_t *e, program_t *prog) {
	e->ex    = 0;
	e->prog  = prog;
	e->halt  = false;
	e->print = false;
	e->tick  = 0;
	for (e->ip = 0; e->ip < e->prog->size && !e->halt; ++ e->ip) {
		em_t *em = &e->prog->ems[e->ip];
		em->ran  = true;
		switch (em->type) {
		case EM_PUSH: stack_push(&e->stack, em->data); break;
		case EM_POP:
			STACK_POP(e, NULL);
			if (e->print && e->print_from > e->stack.size)
				e->print_from = e->stack.size;
			break;

#define STACK_POP2_INT(A, B) \
	STACK_POP(e, (B)); \
	STACK_POP(e, (A)); \
	\
	if ((A)->type != DATA_INT || (A)->type != (B)->type) \
		return runtime_result_err(RUNTIME_ERR_INCORRECT_TYPE, em)

#define BIN_OP_INST(OP) { \
		data_t a, b; \
		STACK_POP2_INT(&a, &b); \
		stack_push(&e->stack, data_new_int(a.as.int_ OP b.as.int_)); \
	} break

		case EM_ADD: BIN_OP_INST(+);
		case EM_SUB: BIN_OP_INST(-);
		case EM_MUL: BIN_OP_INST(*);
		case EM_DIV: {
			data_t a, b;
			STACK_POP2_INT(&a, &b);
			if (b.as.int_ == 0)
				return runtime_result_err(RUNTIME_ERR_DIV_BY_ZERO, em);

			stack_push(&e->stack, data_new_int(a.as.int_ / b.as.int_));
		} break;

		case EM_GRT:  BIN_OP_INST(>);
		case EM_LESS: BIN_OP_INST(<);
		case EM_EQU:  BIN_OP_INST(==);
		case EM_NEQU: BIN_OP_INST(!=);

#undef BIN_OP_INST
#undef STACK_POP2_INT

		case EM_PRINT_BEGIN:
			if (e->ip == em->ref - 1) {
				data_t data;
				STACK_POP(e, &data);
				FILE *file = e->prog->ems[em->ref].data.as.int_ == DATA_STDOUT? stdout : stderr;
				data_fprintf(&data, file);
				fputc('\n', file);
				fflush(file);
			} else {
				e->print      = true;
				e->print_from = e->stack.size;
			}
			break;

		case EM_PRINT_END: {
			if (!e->print || e->print_from == e->stack.size)
				break;

			e->print   = false;
			FILE *file = em->data.as.int_ == DATA_STDOUT? stdout : stderr;
			for (size_t i = e->print_from; i < e->stack.size; ++ i) {
				if (i > e->print_from)
					fputc(' ', file);

				data_fprintf(&e->stack.buf[i], file);
			}
			stack_shrink_to(&e->stack, e->print_from);
			fputc('\n', file);
			fflush(file);
		} break;

#define STACK_POP_INT(E, VAR) \
	STACK_POP(e, &VAR); \
	if (VAR.type != DATA_INT) \
		return runtime_result_err(RUNTIME_ERR_INCORRECT_TYPE, &e->prog->ems[e->ip]);

		case EM_IF_BEGIN: {
			data_t cond;
			STACK_POP_INT(e, cond);
			if (!cond.as.int_)
				e->ip = em->ref;
		} break;

		case EM_IF_END: break;

		case EM_LOOP_BEGIN: {
			data_t cond;
			STACK_POP_INT(e, cond);
			if (!cond.as.int_)
				e->ip = em->ref;
		} break;

		case EM_LOOP_END:
			e->ip = em->ref - 1;
			break;

		case EM_EXIT: {
			data_t ex;
			STACK_POP_INT(e, ex);
			e->ex   = ex.as.int_;
			e->halt = true;
		} break;

		case EM_DUP: {
			data_t off;
			STACK_POP_INT(e, off);
			STACK_DUP(e, (size_t)off.as.int_);
		} break;

		case EM_SWAP: {
			data_t off;
			STACK_POP_INT(e, off);
			STACK_SWAP(e, (size_t)off.as.int_);
		} break;

#ifdef DEBUG
		case EM_DEBUG: {
			for (size_t i = 0; i < e->stack.size; ++ i) {
				fprintf(stdout, "stack[%zu]: ", i);
				data_fprintf(&e->stack.buf[i], stdout);
				fputc('\n', stdout);
			}
		} break;
#endif

		default: assert(0);
		}

		++ e->tick;
		if (e->tick % GC_FREQUENCY_IN_TICKS == 0)
			stack_gc(&e->stack);
	}

	stack_clear(&e->stack);
	env_gc(e);
	return runtime_result_ok(e->ex);
}
