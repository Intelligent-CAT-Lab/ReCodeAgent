/**
 * Utilities for variable-length lists, so we don't always have to use fixed-
 * length arrays.
 * The two extreme approaches are linked lists, which are fast to resize but
 * slow to access (and for that matter, bad for memory management), and fixed-
 * size arrays, which are fast to access but slow to resize. We can go for an
 * intermediate approach with a linked list *of* fixed-size arrays to compromise
 * and get the middlest of both worlds.
 */

#pragma once

struct List;
typedef
    struct List
    List;

// Make and destroy lists.
struct List* create_list(int blocksize);
int destroy_list(List* l);

// Access the n-th element of a list.
void* lget_element(List* l, int index);
// Add new element. Returns success (0) or failure (-1).
int ladd_element(List* l, void* element);
// Set n-th element.
int lset_element(List* l, int index, void* value);
// Iterate over elements. This takes a highly customizable approach. We pass in
// a function which takes a void* pointer and returns an int, and we pass this
// function over every void* element in the list. We return the sum of all
// return values thus accumulated.
int literate(List* l, int (*fn)(void*));
