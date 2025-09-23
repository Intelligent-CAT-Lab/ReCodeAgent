#include <stdlib.h> /* malloc */

#include "openhash.h"

#include "check.h" /* CHECK */
#include "log.h" /* insert_log */

static const float load_factor = .6;
DEBUGIN(static const float eps = 1e-3;)

struct open_entry {
	o_key_t key;
	o_data_t data;
};

struct open_hash_table {
	unsigned cur_size;
	unsigned max_size;

	struct open_entry *arr;
};

static inline struct open_entry *allocate_arr(unsigned size) {
	insert_log(OPEN_HASH_LOG, 1, "Allocating new array for a table with size = %u.", size);

	struct open_entry *arr;
	const unsigned arr_size = sizeof(*arr) * size;
	arr = (struct open_entry *)malloc(arr_size);
	memset(arr, 0, arr_size);

	return arr;
}

struct open_hash_table *create_open_hash_table(unsigned initial_size) {
	insert_log(OPEN_HASH_LOG, 5, "Creating a new table with max size = %u.", initial_size);

	CHECK(initial_size != 0);
	CHECK(initial_size != power && initial_size != rehasher);

	struct open_hash_table *ret = (struct open_hash_table *)malloc(sizeof(*ret));
	ret->cur_size = 0;
	ret->max_size = initial_size;
	ret->arr = allocate_arr(initial_size);

	return ret;
}

static inline struct open_entry *query(o_key_t key, const struct open_hash_table *table) {
	insert_log(OPEN_HASH_LOG, 1, "Query in an with key = %s.", key);

	CHECK(key != empty_key);
	hash_t h;

	for (h = hash(key); ; h = rehash(h)) {
		insert_log(OPEN_HASH_LOG, 0, "Trying a cell with hash = %u(%u).", h % table->max_size, h);
		struct open_entry *entry = &table->arr[h % table->max_size];
		const o_key_t cur_key = entry->key;
		if (cur_key == empty_key || compare_keys(cur_key, key) == 0) {
			insert_log(OPEN_HASH_LOG, 0, "The cell is empty!");
			return entry;
		}
		insert_log(OPEN_HASH_LOG, 0, "Collision! Cell: key = %s, data = %s.", entry->key, entry->data);
	}
	CHECK(0);
	return 0;
}

void open_insert(o_key_t key, o_data_t data, struct open_hash_table *table) {
	insert_log(OPEN_HASH_LOG, 4, "Trying to insert a key = %s and data = %s.", key, data);

	insert_log(OPEN_HASH_LOG, 4, "Current size = %u, maximum size = %u.", table->cur_size, table->max_size);

	CHECK(((float)table->cur_size / table->max_size) < load_factor + eps);

	table->cur_size++;
	
	if (((float)table->cur_size / table->max_size) > load_factor) {
		table->max_size *= 2;
		insert_log(OPEN_HASH_LOG, 4, "Table's density in above the load factor. New max size is %u.", table->max_size);

		struct open_entry *old_arr = table->arr;
		table->arr = allocate_arr(table->max_size);

		for (hash_t h = 0; h < table->max_size / 2; h++) {
			const o_key_t cur_key = old_arr[h].key;
			if (cur_key != empty_key)
				*query(cur_key, table) = old_arr[h];
		}

		free(old_arr);
	}

	struct open_entry *cell = query(key, table);

	CHECK(cell->key == empty_key);
	
	insert_log(OPEN_HASH_LOG, 4, "Inserting key = %s, data = %s. Current size = %u, maximum size = %u.", key, data, table->cur_size, table->max_size);
	cell->key = key;
	cell->data = data;
}

o_data_t open_find(o_key_t key, const struct open_hash_table *table) {
	insert_log(OPEN_HASH_LOG, 2, "Looking for a key = %s.", key);

	struct open_entry *cell = query(key, table);

	CHECK(cell->key != empty_key);

	insert_log(OPEN_HASH_LOG, 2, "Returning data = %s.", cell->data);
	return cell->data;
}

void free_open_hash_table(struct open_hash_table *table) {
	insert_log(OPEN_HASH_LOG, 3, "Deleting a table with current size = %u, maximum size = %u.", table->cur_size, table->max_size);

	free(table->arr);
	free(table);
}

