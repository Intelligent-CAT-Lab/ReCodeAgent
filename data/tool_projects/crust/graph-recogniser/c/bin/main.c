#include <stdio.h>

#include "../src/openhash.h"
#include "../src/cuckoohash.h"

int main() {
	{
		struct open_hash_table * tt = create_open_hash_table(2);

		open_insert("stefan", "manov", tt);
		open_insert("hristo", "tenchev", tt);
		open_insert("dimitar", "kajabachev", tt);
		open_insert("georgi", "popov", tt);
		open_insert("stanislav", "ivanov", tt);
		open_insert("nikola", "yolov", tt);
		open_insert("andrei", "radev", tt);

		printf("The best programmer in the world is %s\n", open_find("nikola", tt));

		free_open_hash_table(tt);
	}
	{
		struct cuckoo_hash_table * tt = create_cuckoo_hash_table(2);

		cuckoo_insert("stefan", "manov", tt);
		cuckoo_insert("hristo", "tenchev", tt);
		cuckoo_insert("dimitar", "kajabachev", tt);
		cuckoo_insert("georgi", "popov", tt);
		cuckoo_insert("stanislav", "ivanov", tt);
		cuckoo_insert("nikola", "yolov", tt);
		cuckoo_insert("andrei", "radev", tt);

		printf("The best programmer in the world is %s\n", cuckoo_find("nikola", tt));

		free_cuckoo_hash_table(tt);
	}

	return 0;
}

