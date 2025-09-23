#if !defined OPENHASH_H
#define OPENHASH_H

#include "hash.h"

struct open_hash_table;

struct open_hash_table *create_open_hash_table(unsigned initial_size);

void open_insert(o_key_t key, o_data_t data, struct open_hash_table *table);

o_data_t open_find(o_key_t key, const struct open_hash_table *table);

void free_open_hash_table(struct open_hash_table *table);

#endif /* !OPENHASH_H */

