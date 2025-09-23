#ifndef HASH_TABLE_H
#define HASH_TABLE_H

#include "../src/common.h"
#include <stdbool.h>

#define HASH_TABLE_SIZE 2000

typedef struct HashNode {
  char *key;
  struct AstNode *value;
  struct HashNode *next;
} HashNode;

typedef struct HashTable {
  HashNode *table[HASH_TABLE_SIZE];
} HashTable;

unsigned int hash(const char *key);

void insert(HashTable *hashTable, const char *key, struct AstNode *value);

bool table_exists(HashTable *hashTable, const char *key);

AstNode *search(HashTable *hashTable, const char *key);

void delete_c(HashTable *hashTable, const char *key);

HashTable *createHashTable();

void destroyHashTable(HashTable *hashTable);

#endif
