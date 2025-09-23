
#include <stddef.h>
#include <stdlib.h>
#include <stdbool.h>
#include <syslog.h>
#include <assert.h>

#include <stdio.h>

#include "dht.h"

void **hash_table;
unsigned int lower_bound;
unsigned int upper_bound;

/* called on app start up */
void dht_init() {
	hash_table = NULL;
	lower_bound = 0;
	upper_bound = 0;
}

//TODO: This must logically call back to the dispatcher to shift relocated items to other nodes ?
bool dht_init_table(unsigned int new_lower_bound, unsigned int new_upper_bound, bool migrate) {

	unsigned int i;
	unsigned int old_hash_table_size;
	unsigned int new_hash_table_size;
	unsigned int absolute_position;
	void **new_hash_table;
	void *item;

	assert(new_upper_bound > new_lower_bound);

	/* ensure we can get the space for the new table */
	new_hash_table_size = new_upper_bound - new_lower_bound;

	syslog(LOG_INFO, "dht_init_table called (new_lower_bound=%i, new_upper_bound=%i, new_hash_table_size=%i",
		new_lower_bound,
		new_upper_bound,
		new_hash_table_size
	);

	new_hash_table = (void*) malloc(new_hash_table_size * sizeof(void*));

	if (((size_t)new_hash_table) == -1) {
		/* call to malloc failed. dht_intialise fails */
		syslog(LOG_WARNING, "call to malloc() failed in dht_init_table");
		return false;
	}

	/* intialise new array */
	for (i = 0; i < new_hash_table_size; i++) {
		new_hash_table[i] = NULL;
	}

	if (migrate)
		syslog(LOG_INFO, "Migrating data during reinitialisation");

	/* free the old table. Possible migrate data to new locations */

	if (dht_is_initialised()) {

		old_hash_table_size = dht_get_size();

		for (i = 0; i < old_hash_table_size; i++) {

			/* there is overlap. move the item to the new location. */

			item = hash_table[i];

			if (item != NULL) {
				
				absolute_position = (i + lower_bound);
				
				if (migrate && absolute_position >= new_lower_bound &&
								absolute_position < new_upper_bound) {

					new_hash_table[(absolute_position - new_lower_bound)] = item;
				} else {
					//TODO: Return to higher level for remapping.
					printf("forgotten... todo remap send to peer\n");
				}
			}

		}

		free(hash_table);
	}

	/* install new table */
	hash_table = new_hash_table;
	lower_bound = new_lower_bound;
	upper_bound = new_upper_bound;

	syslog(LOG_INFO, "dht_init_table complete");
	return true;
}

bool dht_is_initialised() {
	return (hash_table != NULL);
}

unsigned int dht_get_lower_bound() {
	assert(dht_is_initialised());
	return lower_bound;
}

unsigned int dht_get_upper_bound() {
	assert(dht_is_initialised());
	return upper_bound;
}

unsigned int dht_get_size() {
	assert(dht_is_initialised());
	return (upper_bound - lower_bound);
}

void *dht_read(unsigned int location) {
	unsigned int absolute_position;

	assert(dht_is_initialised());
	assert(location >= lower_bound);
	assert(location < upper_bound);
	
	absolute_position = location - lower_bound;
	return hash_table[absolute_position];
}

void dht_write(unsigned int location, void* data) {
	unsigned int absolute_position;
	
	assert(dht_is_initialised());

	assert(location >= lower_bound);
	assert(location < upper_bound);

	absolute_position = location - lower_bound;
	syslog(LOG_DEBUG, "dht_write into %i", absolute_position);
	hash_table[absolute_position] = data;
}

