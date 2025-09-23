#ifndef STACK_H_HEADER_GUARD
#define STACK_H_HEADER_GUARD

#include <stdbool.h> /* bool, true, false */
#include <stdlib.h>  /* malloc, realloc, free, size_t */
#include <assert.h>  /* assert */

#include "utils.h"
#include "data.h"

#define DEFAULT_POPPED_CAP 32

typedef struct {
	char *str;
	bool  marked;
} popped_t;

typedef struct {
	data_t *buf;
	size_t  cap, size;

	popped_t *popped;
	size_t    popped_cap, popped_size;
} stack_t;

#define DEFAULT_STACK_CAP 1024

stack_t stack_new(size_t cap, size_t popped_cap);
void    stack_destroy(stack_t *stack);

void stack_push     (stack_t *stack, data_t  data);
int  stack_pop      (stack_t *stack, data_t *ret);
int  stack_dup      (stack_t *stack, size_t off);
int  stack_swap     (stack_t *stack, size_t off);
void stack_shrink_to(stack_t *stack, size_t size);
void stack_clear    (stack_t *stack);

void stack_gc(stack_t *stack);

#endif
