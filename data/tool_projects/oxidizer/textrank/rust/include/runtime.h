#include "arena.h"

static Arena default_arena;
static Arena *context_arena;

void *context_alloc(size_t size);

void context_reset();
