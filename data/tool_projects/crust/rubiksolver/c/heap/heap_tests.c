#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "heap.h"

bool is_smaller(const void *element1, const void *element2)
{
  return (*((int *)element1) < *((int *)element2));
}
bool is_smaller2(const void *element1, const void *element2)
{
  return (strcmp((char *)element1,(char *)element2) < 0);
}
int main()
{
  heap_t heap = heap_init(10, is_smaller2);

  printf("Testing empty heap...\n");
  assert_verbose(heap_is_empty(heap) == true);
  
  printf("Adding an item...\n");
  heap_insert(heap, "charlie");
  
  printf("Adding an item...\n");
  heap_insert(heap, "alpha");
  
  printf("Adding an item...\n");
  heap_insert(heap, "bravo");
  
  printf("Testing non-empty heap...\n");
  assert_verbose(heap_is_empty(heap) == false);
  
  printf("Testing output...\n");
  assert_verbose(strcmp(heap_delete_min(heap), "alpha") == 0);
  assert_verbose(strcmp(heap_delete_min(heap), "bravo") == 0);
  assert_verbose(strcmp(heap_delete_min(heap), "charlie") == 0);
  
  printf("Testing empty heap...\n");
  assert_verbose(heap_is_empty(heap) == true);
  
  heap_free(heap);
  
  heap = heap_init(10, is_smaller);
  for(int i = 30; i > 10; i--)
  {
    heap_insert(heap, &i);
  }
  assert_verbose(*((int *)heap_delete_min(heap)) == 10);
  heap_free(heap);
  printf("Passed all tests\n");
}
