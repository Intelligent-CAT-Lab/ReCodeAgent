/* Array implementation of binary heap
 * Author: J Kishore Kumar
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "heap.h"

struct heap_struct
{
	void **head;
	int capacity;
	int last;
	bool (*is_smaller)(const void *e1, const void *e2);
};

heap_t heap_init(int init_size, bool (*is_smaller)(const void *e1, const void *e2))
{
	heap_t heap = (heap_t)malloc(sizeof(struct heap_struct));
	heap->head = (void **)malloc(init_size * sizeof(void *));
	memset(heap->head, 0, init_size * sizeof(void *));
	heap->capacity = init_size;
	heap->last = -1;
	heap->is_smaller = is_smaller;
	return heap;
}

void heap_free(heap_t heap)
{
    free(heap->head);
    free(heap);
}
bool heap_grow(heap_t heap)
{
	assert(heap != NULL);
	void **tmp = (void **)malloc(heap->capacity * GROW_FACTOR * sizeof(void *));
	memset(tmp, 0, heap->capacity * GROW_FACTOR * sizeof(void *));
	memcpy(tmp, heap->head, heap->capacity * sizeof(void *));
	if (tmp != NULL)
	{
		free(heap->head);
		heap->head = tmp;
		heap->capacity = heap->capacity * GROW_FACTOR;
		PDEBUG("Heap expanded to %d items..\n", heap->capacity);
		return true;
	}
	else
	{
		PDEBUG("Error expanding heap to %d items..\n", heap->capacity);
		return false;
	}
}

bool heap_insert(heap_t heap, const void *element)
{
	assert(heap != NULL);
	int parent;
	int son = heap->last + 1;
	if(heap->capacity <= son)
	{
		if(!heap_grow(heap))
		{
			return false;
		}
	}
	heap->head[son] = (void *)element;
	parent = (son - 1) / 2;
	while(parent >= 0 && heap->is_smaller(heap->head[son], heap->head[parent]))
	{
		SWAP_ARRAY_ELEMENTS(heap->head, parent, son);
		son = parent;
		parent = (son - 1) / 2;
	}
	heap->last += 1;
	PDEBUG("An element is inserted in heap at position %d. Heap size is %d\n", son, heap->last + 1);
	return true;
}

void *heap_delete_min(heap_t heap)
{
	assert(heap->head != NULL);
	void *min = NULL;
	int parent = 0, son1, son2, son;
	min = heap->head[parent];
	heap->head[parent] = heap->head[heap->last];
	heap->head[heap->last] = NULL;
	heap->last -= 1;
	do
	{
		son1 = 2 * parent + 1;
		son2 = 2 * parent + 2;
		if(son1 <= heap->last)
			if(son2 <= heap->last)
				son = (heap->is_smaller(heap->head[son1],heap->head[son2])?son1:son2);
			else
				son = son1;
		else
			break;
		if(heap->is_smaller(heap->head[son],heap->head[parent]))
		{
			SWAP_ARRAY_ELEMENTS(heap->head, parent, son);
		}
		parent = son;
	}while(1);
	PDEBUG("An element is removed from heap. Heap size is %d\n", heap->last + 1);
	return min;
}

bool heap_is_empty(const heap_t heap)
{
	assert(heap != NULL);
	return (heap->last == -1);
}

void *heap_find_min(const heap_t heap)
{
	assert(heap->head != NULL);
	return heap->head[0];
}
