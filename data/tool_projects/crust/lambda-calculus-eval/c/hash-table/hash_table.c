#include "hash_table.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

unsigned int hash(const char *key) {
  unsigned long hash = 5381;
  int c;
  while ((c = *key++))
    hash = ((hash << 5) + hash) + c;
  return hash % HASH_TABLE_SIZE;
}

void insert(HashTable *hashTable, const char *key, struct AstNode *value) {
  unsigned int index = hash(key);
  HashNode *newNode = (HashNode *)malloc(sizeof(HashNode));
  newNode->key = strdup(key);
  newNode->value = value;
  newNode->next = NULL;

  if (hashTable->table[index] == NULL) {
    hashTable->table[index] = newNode;
  } else {
    HashNode *current = hashTable->table[index];
    while (current->next != NULL)
      current = current->next;
    current->next = newNode;
  }
}

bool table_exists(HashTable *hashTable, const char *key) {
  unsigned int index = hash(key);
  HashNode *node = hashTable->table[index];
  return node != NULL;
}

AstNode *search(HashTable *hashTable, const char *key) {
  if (key == NULL) {
    return NULL;
  }

  unsigned int index = hash(key);
  HashNode *current = hashTable->table[index];

  while (current != NULL) {
    if (strcmp(current->key, key) == 0) {
      return current->value;
    }
    current = current->next;
  }

  return NULL;
}

void delete_c(HashTable *hashTable, const char *key) {
  unsigned int index = hash(key);
  HashNode *current = hashTable->table[index];
  HashNode *prev = NULL;

  while (current != NULL) {
    if (strcmp(current->key, key) == 0) {
      if (prev == NULL)
        hashTable->table[index] = current->next;
      else
        prev->next = current->next;
      free(current->key);
      free(current);
      return;
    }
    prev = current;
    current = current->next;
  }
}

HashTable *createHashTable() {
  HashTable *hashTable = (HashTable *)malloc(sizeof(HashTable));
  memset(hashTable->table, 0, sizeof(hashTable->table));
  return hashTable;
}

void destroyHashTable(HashTable *hashTable) {
  for (int i = 0; i < HASH_TABLE_SIZE; i++) {
    HashNode *current = hashTable->table[i];
    while (current != NULL) {
      HashNode *temp = current;
      current = current->next;
      if (temp->value != NULL) {
        free(temp->value);
      }
      free(temp->key);
      free(temp);
    }
  }
  free(hashTable);
}
