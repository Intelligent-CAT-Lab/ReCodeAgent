#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "hash.h"

unsigned int hash_function(const void *element)
{
  const char *e_str = (const char *)element;
  return (unsigned int)(e_str[0]);
}

bool compare_equal(const void *e1, const void *e2)
{
  return (strcmp((const char *)e1, (const char *)e2) == 0);
}

int main()
{
  hash_t hash = hash_init(255, hash_function);
  assert_verbose(hash_element_exists(hash, "Hello", compare_equal) == false);
  printf("Inserting element 'Hello'...\n");
  hash_insert(hash, "Hello");
  assert_verbose(hash_element_exists(hash, "Hello", compare_equal) == true);
  printf("Inserting element 'Hi'...\n");
  hash_insert(hash, "Hi");
  assert_verbose(hash_element_exists(hash, "Hi", compare_equal) == true);
  printf("Removing element 'Hello'...\n");
  hash_delete(hash, "Hello", compare_equal);
  assert_verbose(hash_element_exists(hash, "Hello", compare_equal) == false);
  printf("All tests passed successfully.\n");
  hash_free(hash);
}
