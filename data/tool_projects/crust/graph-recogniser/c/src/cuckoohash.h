#if !defined CUCKOOHASH_H
#define CUCKOOHASH_H

#include "hash.h"

struct cuckoo_hash_table;

struct cuckoo_hash_table *create_cuckoo_hash_table(unsigned initial_size);

void cuckoo_insert(o_key_t key, o_data_t data, struct cuckoo_hash_table *table);

o_data_t cuckoo_find(o_key_t key, const struct cuckoo_hash_table *table);

void free_cuckoo_hash_table(struct cuckoo_hash_table *table);

#endif /* !CUCKOOHASH_H */

