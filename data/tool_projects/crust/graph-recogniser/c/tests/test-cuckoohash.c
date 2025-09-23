#include <string.h>

#include "test-unit.h"
#include "cuckoohash.h"

#include "defs.h"

static const char test_strs[][2][0xff] = {
	{ "stefan", "manov" },
	{ "hristo", "tenchev" },
	{ "dimitar", "kajabachev" },
	{ "georgi", "popov" },
	{ "stanislav", "ivanov" },
	{ "nikola", "yolov" },
	{ "andrei", "radev" },
	{ "iulen", "dobrev" },
	{ "iasen", "bantchev" },
	{ "samuele", "carli" },
	{ "henning", "weiler" },
	{ "javier", "martin" },
};

static const unsigned permut[] = {
	10, 0, 4, 3, 5, 3, 7, 11, 4, 11, 6, 0, 1, 8, 5, 1, 10, 3, 5, 2, 9,
};

int test_cuckoohash() {
	unsigned i;
	struct cuckoo_hash_table * tt = create_cuckoo_hash_table(2);

	for (i = 0; i < COUNTOF(test_strs); ++i)
		cuckoo_insert(test_strs[i][0], test_strs[i][1], tt);

	for (i = 0; i < COUNTOF(permut); ++i)
		if (strcmp(cuckoo_find(test_strs[permut[i]][0], tt), test_strs[permut[i]][1]) != 0)
			return 1;

	free_cuckoo_hash_table(tt);

	return 0;
}

