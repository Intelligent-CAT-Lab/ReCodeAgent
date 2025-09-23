#ifndef HASH_H

#define HASH_H 1

//#define DEBUG 1
#include "../helpers.h"

typedef struct bucket_element_struct
{
  void *element;
  struct bucket_element_struct *next;
}*bucket_element_t;

typedef bucket_element_t bucket_t;

typedef struct hash_struct
{
  bucket_t *buckets;
  unsigned int (*hash_function)(const void *element);
  unsigned int max_key;
}*hash_t;

hash_t hash_init(unsigned int max_key, unsigned int (*hash_function)(const void *element));
bool hash_insert(hash_t hash, const void *element);
bool hash_element_exists(const hash_t hash, const void *element, bool(*compare_equal)(const void *e1, const void *e2));
bool hash_delete(hash_t hash, const void *element, bool(*compare_equal)(const void *e1, const void *e2));
void hash_free(hash_t hash);

#endif

