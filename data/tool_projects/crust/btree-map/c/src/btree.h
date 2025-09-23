#ifndef __BTREE_H__
#define __BTREE_H__

#include<stdint.h>
#include <stdlib.h>


#define BTREE_KEY_SIZE 10

typedef struct value
{
    uint8_t* value;
    size_t len; 
}value_t;

typedef struct btree_key
{
    uint8_t* key;
    size_t len;
}btree_key_t;

typedef struct entry {
    btree_key_t key;
    value_t value;
}entry_t;

typedef struct entry_list{
    entry_t* entries;
    size_t len;
    size_t cap;
}entry_list_t;

typedef struct node node_t;
typedef struct btree btree_t;

btree_t* new_btree();
node_t* new_node(void* key, size_t key_len, void* value, size_t value_len);
value_t* find_entry(btree_t* self, void* key, size_t key_len);
void add_entry(btree_t* self, void* key, size_t key_len, void* value, size_t value_len);
void free_tree(btree_t** self);
void remove_entry(btree_t* self, void* key, size_t key_len);
entry_list_t* list_entries(btree_t* self);
size_t get_entry_count(btree_t* self);
void free_entry_list(entry_list_t** list);
uint32_t calc_key_hash(void* key, size_t key_len);
#endif
