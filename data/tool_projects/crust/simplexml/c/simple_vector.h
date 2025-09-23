#ifndef SIMPLE_VECTOR_H_
#define SIMPLE_VECTOR_H_

typedef struct Vector {
  int _capacity;
  int _size; 
  void** _data;
} Vector;

// Initialize a vector
Vector* vector_create();

// Release a vector
// This function only release memory of vector `v`
// You must write code to release all element of `v`
//
// Example
//    Vector *v;
//    // allocate and add some element in v
//    for (int i = 0; i < vector_size(v); ++i) {
//      void* elem = vector_get_element_at(v, i);
//      free(elem);
//    }
//    vector_release(v);
void vector_release(Vector *v);

// Return the size of vector v
int vector_size(Vector *v);

// Return element of vector `v` at  index `index`
// index must be in range [0..`size`]
void* vector_get_element_at(Vector *v, int index);

// Add new element `elem` at end of vector `v`
//
// Return 1 if push sucessfull
//        0 if have any errors (like cannot new allocate)
int vector_push_back(Vector *v, void* elem);

// Remove and return last element of vector `v`
void* vector_pop_back(Vector *v);

// Return last element of vector `v`
void* vector_top_back(Vector *v);

// Add new element `elem` at begin of vector `v`
// Return 1 if push sucessfull
//        0 if have any errors (like cannot new allocate)
int vector_push_front(Vector *v, void* elem);

// Remove and return first element of vector `v`
void* vector_pop_front(Vector *v);

// Return first element of vector `v`
void* vector_top_front(Vector *v);

// Add new element `elem` before `index`
// Example:
//    >>> v = [1, 2, 3]
//    >>> vector_insert_at_index(v, 5, 0)
//    => v = [5, 1, 2, 3]
//    >>> vector_insert_at_index(v, 10, 2)
//    => v = [5, 1, 10, 2, 3]
//    >>> vector_insert_at_index(v, 11, 5)
//    => v = [5, 1, 10, 2, 3, 11]
//
// Return 1 if push sucessfull
//        0 if have any errors (like cannot new allocate)
int vector_insert_at_index(Vector *v, void* elem, int index);

// Remove element of vector `v` at index `index`
// Return element at `index`
void* vector_remove_element_at_index(Vector *v, int index);

// Return first index of element `elem` in vector `v` 
// If we dont find any element, return -1
//
// Example: 
//    >>> v = [1, 1, 1, 2, 3]
//    >>> vector_index_of(v, 3)
//    => 5
//    >>> vector_index_of(v, 3)
//    => 0
int vector_index_of(Vector *v, void *elem);

// Return first index which greater than equal `start_index`, 
// of element `elem` in vector `v` 
// If we dont find any element, return -1
// 
// Example: 
//    >>> v = [1, 1, 1, 2, 3]
//    >>> vector_index_of(v, 3)
//    => 5
//    >>> vector_index_of(v, 1)
//    => 0
//    >>> vector_index_of(v, 1, 1)
//    => 1
int vector_index_of_with_start(Vector *v, void *elem, int start);

#endif
