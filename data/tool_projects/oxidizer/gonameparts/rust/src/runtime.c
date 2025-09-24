#define ARENA_IMPLEMENTATION
#include "../include/runtime.h"

static Arena default_arena = { .begin = 0, .end = 0 };
static Arena *context_arena = &default_arena;

void *context_alloc(size_t size)
{
    assert(context_arena);
    return arena_alloc(context_arena, size);
}

void context_reset()
{
    arena_free(&default_arena);
}