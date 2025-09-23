
#pragma once

#include <stdbool.h>        // for bool
#include "trusted_utils.h"

// A hash table mapping from u64 keys to void* values.
// The key zero is a magic number representing an empty entry.
// The capacity is provided logarithmically such that the
// table's capacity is always a power of two.
// Growing is done in powers of two, if the load factor exceeds 0.5.
// No shrinking is done. Assuming roughly monotonic growth,
// the table's load factor is always between 0.25 and 0.5.
// Collisions are handled via linear probing, which in this mode
// appears to perform reasonably well.

struct hash_table_entry {
    u64 key;
    void* val;
};

struct hash_table {
    u64 size;
    u64 max_size;
    float growth_factor;
    u64 capacity;
    struct hash_table_entry* data;
    u64 last_found_idx;
};

struct hash_table* hash_table_init(int log_init_capacity);
void* hash_table_find(struct hash_table* ht, u64 key);
bool hash_table_insert(struct hash_table* ht, u64 key, void* data);
bool hash_table_delete(struct hash_table* ht, u64 key);
bool hash_table_delete_last_found(struct hash_table* ht);
void hash_table_free(struct hash_table* ht);
