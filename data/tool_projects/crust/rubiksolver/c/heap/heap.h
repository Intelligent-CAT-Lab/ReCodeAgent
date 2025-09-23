#ifndef HEAP_H

#define HEAP_H

//#define DEBUG 1
#include "../helpers.h"

#define GROW_FACTOR   1.5f

struct heap_struct;
typedef struct heap_struct *heap_t;

heap_t heap_init(int init_size, bool (*is_smaller)(const void *e1, const void *e2)); //Clears the heap and all it's contents
bool heap_insert(heap_t heap, const void *element);  //Adds element to heap and percolates up
void *heap_delete_min(heap_t heap);            //Removes top element of heap and maintain heap
void *heap_find_min(const heap_t heap);              //Returns top element without removing it
bool heap_is_empty(const heap_t heap);               //Must be used before calling heap_find_min and heap_delete_min
void heap_free(heap_t heap);

#endif
