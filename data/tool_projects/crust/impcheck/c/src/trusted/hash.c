
#include "hash.h"
#include "trusted_utils.h"
#include <assert.h>  // for assert
#include <stdlib.h>  // for free

u64 compute_hash(u64 key) {
    return (0xcbf29ce484222325UL ^ key) * 0x00000100000001B3UL;
}
u64 compute_idx(struct hash_table* ht, u64 key) {
    return compute_hash(key) & (ht->capacity-1);
}

bool cell_empty(struct hash_table_entry* entry) {
    return entry->key == 0;
}

bool find_entry(struct hash_table* ht, u64 key, u64* idx) {
    u64 i = compute_idx(ht, key);
    const u64 orig_idx = i;
    while (i < ht->capacity) {
        struct hash_table_entry* entry = &ht->data[i];
        if (cell_empty(entry)) {
            *idx = i; return false; // key is not present.
        }
        if (entry->key == key) {
            *idx = i; return true; // key found
        }
        i++;
    }
    i = 0;
    while (i < orig_idx) {
        struct hash_table_entry* entry = &ht->data[i];
        if (cell_empty(entry)) {
            *idx = i; return false; // key is not present.
        }
        if (entry->key == key) {
            *idx = i; return true; // key found
        }
        i++;
    }
    return false; // searched the entire table
}

bool realloc_table(struct hash_table* ht) {
    u64 new_capacity = (u64) (ht->growth_factor * ht->capacity);
    //printf("GROW %lu -> %lu\n", ht->capacity, new_capacity);
    struct hash_table_entry* old_data = ht->data;
    u64 old_capacity = ht->capacity;
    ht->data = (struct hash_table_entry*) trusted_utils_calloc(new_capacity, sizeof(struct hash_table_entry));
    if (!ht->data) return false;
    ht->size = 0;
    ht->max_size = (u64) (ht->growth_factor * ht->max_size);
    ht->capacity = new_capacity;
    for (u64 i = 0; i < old_capacity; i++) {
        struct hash_table_entry* cell = &old_data[i];
        if (!cell_empty(cell)) {
            if (!hash_table_insert(ht, cell->key, cell->val))
                return false;
        }
    }
    free(old_data);
    return true;
}

bool handle_gap(struct hash_table* ht, u64 idx_of_gap) {

    u64 i = idx_of_gap;
    u64 j = i;
    while (true) {
        // search forward through the following cells of the table
        // until finding either another empty cell or a key that
        // can be moved to cell i (that is, a key whose hash value
        // is equal to or earlier than i)

        j = (j+1) & (ht->capacity-1);
        if (cell_empty(&ht->data[j])) {
            // empty cell found!
            // When an empty cell is found, then emptying cell i
            // is safe and the deletion process terminates.
            struct hash_table_entry* entry = &ht->data[i];
            entry->key = 0;
            entry->val = 0;
            return true;
        }

        u64 k = compute_idx(ht, ht->data[j].key);
        if ((j > i && (k <= i || k > j)) 
            || (j < i && k <= i && k > j)) {

            // movable cell found!
            // when the search finds a key that can be moved to cell i, it performs this move.
            struct hash_table_entry* entry_at_deletion = &ht->data[i];
            struct hash_table_entry* entry2move = &ht->data[j];
            *entry_at_deletion = *entry2move;
            entry2move->key = 0;
            entry2move->val = 0;

            // This empties out another cell, later in the same block of occupied cells.
            // The search for a movable key continues for the new emptied cell,
            // in the same way, until it terminates by reaching a cell that was already empty.
            i = j;
        }
    }
}



struct hash_table* hash_table_init(int log_init_capacity) {
    struct hash_table* ht = trusted_utils_malloc(sizeof(struct hash_table));
    ht->capacity = 1<<log_init_capacity;
    ht->size = 0;
    ht->max_size = (u64) (ht->capacity >> 1);
    ht->growth_factor = 2;
    ht->data = (struct hash_table_entry*) trusted_utils_calloc(ht->capacity, sizeof(struct hash_table_entry));
    return ht;
}

void* hash_table_find(struct hash_table* ht, u64 key) {
    u64 idx;
    if (!find_entry(ht, key, &idx)) return 0;
    ht->last_found_idx = idx;
    assert(ht->data[idx].key == key);
    assert(ht->data[idx].val);
    return ht->data[idx].val;
}

bool hash_table_insert(struct hash_table* ht, u64 key, void* val) {
    if (key == 0) return false; // key 0 is reserved!

    if (ht->size == ht->max_size) {
        if (!realloc_table(ht)) return false; // no memory left
        if (ht->size >= ht->max_size)
            return false; // sth went very wrong during realloc
    }

    u64 idx;
    if (find_entry(ht, key, &idx))
        return false; // found an element with this key!
    if (!cell_empty(&ht->data[idx]))
        return false; // table completely full - shouldn't happen!
    // idx now points to the empty position

    struct hash_table_entry* entry = &ht->data[idx];
    entry->key = key;
    entry->val = val;
    ht->size++;
    return true;
}

bool hash_table_delete(struct hash_table* ht, u64 key) {
    u64 idx;
    if (!find_entry(ht, key, &idx)) return false;
    if (!handle_gap(ht, idx)) return false;
    ht->size--;
    return true;
}

bool hash_table_delete_last_found(struct hash_table* ht) {
    if (!handle_gap(ht, ht->last_found_idx)) return false;
    ht->size--;
    return true;
}

void hash_table_free(struct hash_table* ht) {
    free(ht);
}
