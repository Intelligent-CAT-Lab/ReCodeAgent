#include <stdlib.h> /* malloc */

#include "cuckoohash.h"

#include "check.h" /* CHECK */
#include "log.h" /* log */

static const float load_factor = 1.;
DEBUGIN(static const float eps = 1e-3;)

struct cuckoo_entry {
	o_key_t key;
	o_data_t data;
	unsigned marker;
};

struct cuckoo_hash_table {
	unsigned cur_size;
	unsigned cur_marker;
	unsigned max_size;

	struct cuckoo_entry *first_arr;
	struct cuckoo_entry *second_arr;
};

static inline struct cuckoo_entry *allocate_arr(unsigned size) {
	insert_log(CUCKOO_HASH_LOG, 1, "Allocating new array with size = %u.", size);
 
	struct cuckoo_entry *arr;
	const unsigned arr_size = sizeof(*arr) * size;
	arr = (struct cuckoo_entry *)malloc(arr_size);
	memset(arr, 0, arr_size);

	return arr;
}

static inline void initialize_table(unsigned size, struct cuckoo_hash_table *table) {
	table->cur_size = 0;
	table->cur_marker = 0;
	table->max_size = size;
	table->first_arr = allocate_arr(size);
	table->second_arr = allocate_arr(size);
}

struct cuckoo_hash_table *create_cuckoo_hash_table(unsigned initial_size) {
	insert_log(CUCKOO_HASH_LOG, 5, "Creating a new table with max size = %u.", initial_size);

	CHECK(initial_size >= 2);
	CHECK(initial_size != power && initial_size != alternative_power);

	struct cuckoo_hash_table *ret = (struct cuckoo_hash_table *)malloc(sizeof(*ret));
	initialize_table(initial_size / 2, ret);

	return ret;
}

static inline void recreate(unsigned new_size, struct cuckoo_hash_table *table) {
	insert_log(CUCKOO_HASH_LOG, 4, "Recreating the table with new max size = %u.", new_size * 2);

	CHECK(new_size > 0);
	CHECK((float)new_size * load_factor + eps > table->cur_size);

	struct cuckoo_entry *old_first = table->first_arr;
	struct cuckoo_entry *old_second = table->second_arr;
	unsigned old_size = table->max_size;

	initialize_table(new_size, table);

	for (unsigned i = 0; i < old_size; ++i) {
		struct cuckoo_entry *cur_entry = &old_first[i];
		if (cur_entry->key != empty_key)
			cuckoo_insert(cur_entry->key, cur_entry->data, table);
	}

	for (unsigned i = 0; i < old_size; ++i) {
		struct cuckoo_entry *cur_entry = &old_second[i];
		if (cur_entry->key != empty_key)
			cuckoo_insert(cur_entry->key, cur_entry->data, table);
	}

	free(old_first);
	free(old_second);
}

static inline void resize(struct cuckoo_hash_table *table) {
	recreate(table->max_size * 2, table);
}

static inline void refill(struct cuckoo_hash_table *table) {
	recreate(table->max_size + 1, table);
}

static inline struct cuckoo_entry *get_first_entry(o_key_t key, struct cuckoo_hash_table *table) {
	hash_t h = hash(key);
	hash_t hh = h % table->max_size;
	insert_log(CUCKOO_HASH_LOG, 1, "Looking for key = %s with hash = %u(%u) in the first array.", key, hh, h);

	struct cuckoo_entry *ret = &table->first_arr[hh];
	insert_log(CUCKOO_HASH_LOG, 1, "Found key = %s, data = %s, marker = %u.", ret->key? ret->key : "empty", ret->data? ret->data : "empty", ret->marker);

	return ret;
}

static inline const struct cuckoo_entry *get_first_entry_c(o_key_t key, const struct cuckoo_hash_table *table) {
	hash_t h = hash(key);
	hash_t hh = h % table->max_size;
	insert_log(CUCKOO_HASH_LOG, 1, "Looking for key = %s with hash = %u(%u) in the first array.", key, hh, h);

	const struct cuckoo_entry *ret = &table->first_arr[hh];
	insert_log(CUCKOO_HASH_LOG, 1, "Found key = %s, data = %s, marker = %u.", ret->key? ret->key : "empty", ret->data? ret->data : "empty", ret->marker);

	return ret;
}

static inline struct cuckoo_entry *get_second_entry(o_key_t key, struct cuckoo_hash_table *table) {
	hash_t h = hash(key);
	hash_t hh = h % table->max_size;
	insert_log(CUCKOO_HASH_LOG, 1, "Looking for key = %s with hash = %u(%u) in the second array.", key, hh, h);

	struct cuckoo_entry *ret = &table->second_arr[hh];
	insert_log(CUCKOO_HASH_LOG, 1, "Found key = %s, data = %s, marker = %u.", ret->key? ret->key : "empty", ret->data? ret->data : "empty", ret->marker);

	return ret;
}

static inline const struct cuckoo_entry *get_second_entry_c(o_key_t key, const struct cuckoo_hash_table *table) {
	hash_t h = hash(key);
	hash_t hh = h % table->max_size;
	insert_log(CUCKOO_HASH_LOG, 1, "Looking for key = %s with hash = %u(%u) in the second array.", key, hh, h);

	const struct cuckoo_entry *ret = &table->second_arr[hh];
	insert_log(CUCKOO_HASH_LOG, 1, "Found key = %s, data = %s, marker = %u.", ret->key? ret->key : "empty", ret->data? ret->data : "empty", ret->marker);

	return ret;
}

static inline void swap_key_data_entry(o_key_t *key, o_data_t *data, struct cuckoo_entry *entry) {
	insert_log(CUCKOO_HASH_LOG, 2, "Swapping key = %s, data = %s with key = %s, data = %s.",
		*key, *data, entry->key, entry->data);

	CHECK(key && data && entry);

	o_key_t swap_key = entry->key;
	o_data_t swap_data = entry->data;
	entry->key = *key;
	entry->data = *data;
	*key = swap_key;
	*data = swap_data;
}

static inline int try_to_store(o_key_t key, o_data_t data, struct cuckoo_entry *entry) {
	insert_log(CUCKOO_HASH_LOG, 0, "Trying to store key = %s, data = %s ...", key, data);

	if (entry->key == empty_key) {
		insert_log(CUCKOO_HASH_LOG, 0, "...success.");

		entry->key = key;
		entry->data = data;
		return 1;
	}
	else {
		insert_log(CUCKOO_HASH_LOG, 0, "...failure.");

		CHECK(compare_keys(entry->key, key) != 0);
		return 0;
	}
}

void cuckoo_insert(o_key_t key, o_data_t data, struct cuckoo_hash_table *table) {
	insert_log(CUCKOO_HASH_LOG, 4, "Inserting key = %s, data = %s.", key, data);

	table->cur_marker++;
	insert_log(CUCKOO_HASH_LOG, 3, "Table size = %u, max size = %u and the new marker = %u.", table->cur_size, table->max_size, table->cur_marker);
	if (1. + table->cur_size > table->max_size * load_factor) {
		insert_log(CUCKOO_HASH_LOG, 4, "Table fillness is above the load factor and it needs to be resized.");
		resize(table);
		insert_log(CUCKOO_HASH_LOG, 2, "Finished with resizing, continuing with key = %s, data = %s.", key, data);
	}

	table->cur_size++;
	struct cuckoo_entry *first_entry = get_first_entry(key, table);
	if (try_to_store(key, data, first_entry))
		return;

	struct cuckoo_entry *second_entry = get_second_entry(key, table);
	if (try_to_store(key, data, second_entry))
		return;

	for ( ; ; ) {
		swap_key_data_entry(&key, &data, first_entry);
		first_entry->marker = table->cur_marker;

		second_entry = get_second_entry(key, table);
		if (second_entry->marker == table->cur_marker) {
			insert_log(CUCKOO_HASH_LOG, 5, "A cycle has been detected and the table needs to be recreated.");
			refill(table);
			cuckoo_insert(key, data, table);
			break;
		}

		if (try_to_store(key, data, second_entry))
			break;

		swap_key_data_entry(&key, &data, second_entry);
		second_entry->marker = table->cur_marker;

		first_entry = get_first_entry(key, table);
		if (first_entry->marker == table->cur_marker) {
			insert_log(CUCKOO_HASH_LOG, 5, "A cycle has been detected and the table needs to be recreated.");
			refill(table);
			cuckoo_insert(key, data, table);
			break;
		}

		if (try_to_store(key, data, first_entry))
			break;
	}
}

o_data_t cuckoo_find(o_key_t key, const struct cuckoo_hash_table *table) {
	insert_log(CUCKOO_HASH_LOG, 3, "Looking for key = %s.", key);

	const struct cuckoo_entry *entry = get_first_entry_c(key, table);
	if (entry->key != empty_key && compare_keys(entry->key, key) == 0) {
		insert_log(CUCKOO_HASH_LOG, 2, "The key is found in the first array. Returning data = %s.", entry->data);
		return entry->data;
	}

	insert_log(CUCKOO_HASH_LOG, 2, "The key is not found in the first array.");
	entry = get_second_entry_c(key, table);
	CHECK(entry->key != empty_key && compare_keys(entry->key, key) == 0);

	insert_log(CUCKOO_HASH_LOG, 2, "The key is found in the second array. Returning data = %s.", entry->data);
	return entry->data;
}

void free_cuckoo_hash_table(struct cuckoo_hash_table *table) {
	insert_log(CUCKOO_HASH_LOG, 5, "Deleting hash table with current size = %u, maximum size = %u.", table->cur_size, table->max_size);

	free(table->first_arr);
	free(table->second_arr);
	free(table);
}

