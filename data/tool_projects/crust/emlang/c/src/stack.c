#include "stack.h"

stack_t stack_new(size_t cap, size_t popped_cap) {
	stack_t stack = {.cap = cap, .size = 0};
	stack.buf = (data_t*)malloc(stack.cap * sizeof(*stack.buf));
	assert(stack.buf != NULL);

	stack.popped_cap = popped_cap;
	stack.popped     = (popped_t*)malloc(stack.popped_cap * sizeof(popped_t));
	assert(stack.popped != NULL);

	return stack;
}

static void stack_add_popped(stack_t *stack, char *str) {
	assert(stack != NULL);

	if (stack->popped_size >= stack->popped_cap) {
		stack->popped_cap *= 2;
		stack->popped      = (popped_t*)realloc(stack->popped,
		                                        stack->popped_cap * sizeof(popped_t));
		assert(stack->popped != NULL);
	}

	stack->popped[stack->popped_size ++] = (popped_t){.str = str, .marked = false};
}

void stack_destroy(stack_t *stack) {
	assert(stack != NULL);

	for (size_t i = 0; i < stack->size; ++ i) {
		if (stack->buf[i].type != DATA_STR)
			continue;

		assert(stack->buf[i].as.str != NULL);
		free(stack->buf[i].as.str);
	}

	free(stack->buf);
	free(stack->popped);
}

void stack_push(stack_t *stack, data_t data) {
	assert(stack != NULL);

	if (stack->size >= stack->cap) {
		stack->cap *= 2;
		stack->buf  = (data_t*)realloc(stack->buf, stack->cap * sizeof(*stack->buf));
		assert(stack->buf != NULL);
	}

	stack->buf[stack->size ++] = data;
}

int stack_pop(stack_t *stack, data_t *ret) {
	assert(stack != NULL);
	if (stack->size == 0)
		return -1;

	data_t data = stack->buf[-- stack->size];
	if (data.type == DATA_STR)
		stack_add_popped(stack, data.as.str);

	if (ret != NULL)
		*ret = data;
	return 0;
}

int stack_dup(stack_t *stack, size_t off) {
	assert(stack != NULL);
	if (off + 1 > stack->size)
		return -1;

	stack_push(stack, stack->buf[stack->size - off - 1]);
	return 0;
}

int stack_swap(stack_t *stack, size_t off) {
	assert(stack != NULL);
	if (off + 1 > stack->size)
		return -1;

	data_t tmp = stack->buf[stack->size - off - 1];
	stack->buf[stack->size - off - 1] = stack->buf[stack->size - 1];
	stack->buf[stack->size - 1]       = tmp;
	return 0;
}

void stack_shrink_to(stack_t *stack, size_t size) {
	assert(stack != NULL);
	if (size == stack->size)
		return;
	else
		assert(size < stack->size);

	for (size_t i = size; i < stack->size; ++ i) {
		if (stack->buf[i].type == DATA_STR) {
			stack_add_popped(stack, stack->buf[i].as.str);
		}
	}

	stack->size = size;
}

void stack_clear(stack_t *stack) {
	stack_shrink_to(stack, 0);
	stack_gc(stack);
}

void stack_gc(stack_t *stack) {
#ifdef DEBUG
	printf("\n[GARBAGE COLLECTING]\n");
#endif

	assert(stack != NULL);
	if (stack->popped_size == 0)
		return;

	for (size_t i = 0; i < stack->size; ++ i) {
		if (stack->buf[i].type != DATA_STR)
			continue;

		for (size_t i = 0; i < stack->popped_size; ++ i) {
			if (stack->buf[i].as.str == stack->popped[i].str)
				stack->popped[i].marked = true;
		}
	}

	for (size_t i = 0; i < stack->popped_size; ++ i) {
		if (!stack->popped[i].marked)
			free(stack->popped[i].str);
	}

	stack->popped_size = 0;
}
