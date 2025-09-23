#include <stdlib.h>
#include <assert.h>
#include "simple_vector.h"

// validate vector is not NULL
static void vector_validated(Vector *v) {
  assert(v != NULL && "vector is NULL");
}

// Initialize a vector
Vector* vector_create(Vector *v) { 
  v = malloc(sizeof(Vector));
  v->_capacity = 8;
  v->_size = 0;
  v->_data = calloc(v->_capacity, sizeof(void*));
  return v;
}

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
void vector_release(Vector *v) { 
  vector_validated(v);

  free(v->_data);
  free(v);
}

// Return the size of vector v
int vector_size(Vector *v) { 
  vector_validated(v);

  return v->_size;
}

// Return element of vector `v` at  index `index`
// index must be in range [0..`size`]
void* vector_get_element_at(Vector *v, int index) { 
  vector_validated(v);

  assert(index >= 0 && index < v->_size  && "index of out range");
  return (void*) v->_data[index];
}

// Add new element `elem` at end of vector `v`
//
// Return 1 if push sucessfull
//        0 if have any errors (like cannot new allocate)
int vector_push_back(Vector *v, void* elem) { 
  return vector_insert_at_index(v, elem, v->_size);
}

// Remove last element of vector `v`
void* vector_pop_back(Vector *v) { 
  return vector_remove_element_at_index(v, v->_size - 1);
}

// Return the last element of vector `v`
void* vector_top_back(Vector *v) {
  if (v->_size == 0)
    return NULL;
  return vector_get_element_at(v, vector_size(v) - 1);
}

// Add new element `elem` at begin of vector `v`
// Return 1 if push sucessfull
//        0 if have any errors (like cannot new allocate)
int vector_push_front(Vector *v, void* elem) { 
  return vector_insert_at_index(v, elem, 0);
}

// Remove first element of vector `v`
void* vector_pop_front(Vector *v) { 
  return vector_remove_element_at_index(v, 0);
}

// Return the first element of vector `v`
void* vector_top_front(Vector *v) {
  if (v->_size == 0)
    return NULL;
  return vector_get_element_at(v, 0);
}

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
// Return 1 if insert sucessfull
//        0 if have any errors (like cannot new allocate)
int vector_insert_at_index(Vector *v, void* elem, int index) { 
  vector_validated(v);
  assert(index >= 0 && index <= v->_size && "index of out range");
  
  int i;
  // re-allocate vector if needed
  if (v->_size == v->_capacity) {
    v->_capacity = v->_capacity * 2;
    void** old_data = v->_data;
    v->_data = calloc(v->_capacity, sizeof(void*));
    for (i = 0; i < v->_size; ++i)
      v->_data[i] = old_data[i]; 
    free(old_data);
  }

  // insert element to vector
  v->_size++;
  for (i = v->_size; i > index; --i) {
    v->_data[i] = v->_data[i - 1];
  }
  v->_data[index] = elem;

  return 1;
}

// Remove element of vector `v` at index `index`
// Return element at `index`
void* vector_remove_element_at_index(Vector *v, int index) { 
  vector_validated(v);
  assert(index >= 0 && index < v->_size && "index of out range");

  int i; 
  void* elem;
  elem = v->_data[index];
  for (i = index; i < v->_size; ++i) {
    v->_data[i] = v->_data[i + 1]; 
  }
  v->_size--;

  return elem;
}

// Return first index of element `elem` in vector `v` 
// If we dont find any element, return -1
//
// Example: 
//    >>> v = [1, 1, 1, 2, 3]
//    >>> vector_index_of(v, 3)
//    => 5
//    >>> vector_index_of(v, 3)
//    => 0
int vector_index_of(Vector *v, void *elem) {
  return vector_index_of_with_start(v, elem, 0);
}

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
int vector_index_of_with_start(Vector *v, void *elem, int start) {
  vector_validated(v);

  int i;
  for (i = start; i < v->_size; ++i) {
    if (v->_data[i] == elem) {
      return i;
    }
  }

  return -1;
}
