#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "hash.h"

hash_t hash_init(unsigned int max_key, unsigned int (*hash_function)(const void *element))
{
  hash_t hash = (hash_t)malloc(sizeof(struct hash_struct));
  hash->buckets = (bucket_t *)malloc(sizeof(bucket_t) * (max_key + 1));
  memset(hash->buckets, 0, sizeof(bucket_t) * (max_key + 1));
  hash->hash_function = hash_function;
  hash->max_key = max_key;
  return hash;
}

void hash_free(hash_t hash)
{
  for(int i = 0; i <= hash->max_key; i++)
  {
    bucket_element_t curr, painted;
    curr = hash->buckets[i];
    while(curr != NULL)
    {
      painted = curr;
      curr = curr->next;
      free(painted);
    }
  }
  free(hash->buckets);
  free(hash);
}

bool hash_insert(hash_t hash, const void *element)
{
  assert(hash != NULL);
  unsigned int ix = hash->hash_function(element);
  assert(ix <= hash->max_key);
  bucket_element_t new_elem, parent;
  if(hash->buckets[ix] == NULL)
  {
    hash->buckets[ix] = (bucket_t)malloc(sizeof(struct bucket_element_struct));
    new_elem = hash->buckets[ix];
  }
  else
  {
    parent = hash->buckets[ix];
    while(parent->next != NULL)
    {
      parent = parent->next;
    }
    parent->next = (bucket_element_t)malloc(sizeof(struct bucket_element_struct));
    new_elem = parent->next;
  }
  new_elem->element = (void *)element;
  new_elem->next = NULL;
  PDEBUG("An element inserted into hash(%x) at index %d\n", (unsigned int)hash, ix);
  return true;
}
bool hash_element_exists(const hash_t hash, const void *element, bool(*compare_equal)(const void *e1, const void *e2))
{
  assert(hash != NULL);
  int ix = hash->hash_function(element);
  assert(ix <= hash->max_key);
  bucket_element_t curr = hash->buckets[ix];
  PDEBUG("Checking hash_element_exists at index %d\n", ix);
  if((unsigned int)curr == 0x3)
    printf("Target acquired\n");
  while(curr != NULL)
  {
    if(compare_equal(curr->element, element))
    {
      return true;
    }
    curr = curr->next;
  }
  return false;
}
bool hash_delete(hash_t hash, const void *element, bool(*compare_equal)(const void *e1, const void *e2))
{
  assert(hash != NULL);
  int ix = hash->hash_function(element);
  assert(ix <= hash->max_key);
  bucket_element_t curr = hash->buckets[ix];
  if(curr != NULL)
  {
    if(compare_equal(curr->element, element))
    {
      // Remove first element in a bucket
      if(curr->next == NULL)
      {
        PDEBUG("Freeing memory of curr(0x%x)\n", (unsigned int)curr);
        hash->buckets[ix] = NULL;
      }
      else
      {
        hash->buckets[ix] = curr->next;
      }
      free(curr);
      PDEBUG("Element deleted from hash(%x) at index %d (first element of bucket)\n", (unsigned int)hash, ix);
      return true;
    }
  }
  else
  {
    PDEBUG("Failed to delete element from hash(%x) at index %d (first element of bucket). Reason: curr is NULL\n", (unsigned int)hash, ix);
    return false;
  }
  while(curr != NULL && curr->next != NULL)
  {
    if(compare_equal(curr->next->element, element))
    {
      bucket_element_t del = curr->next;
      curr->next = del->next;
      PDEBUG("Freeing memory of del(0x%x)\n", (unsigned int)del);
      free(del);
      PDEBUG("Element deleted from hash(%x) at index %d\n", (unsigned int)hash, ix);
      return true;
    }
    curr = curr->next;
  }
  PDEBUG("Failed to delete element from hash(%x) at index %d. Reason: Element not found.\n", (unsigned int)hash, ix);
  return false;
}

