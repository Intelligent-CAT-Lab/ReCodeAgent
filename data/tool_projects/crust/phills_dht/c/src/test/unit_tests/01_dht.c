
#include <stdbool.h>
#include "../../dht.h"
#include <stdio.h>
#include <assert.h>

int main(int argc, char **argv);
bool uninitialise_test();
bool initialised_test();
bool get_size_test_1();
bool get_size_test_2();
bool get_size_test_3();
bool get_size_test_4();
bool get_size_test_5();
bool check_bound_test_5();
bool check_bound_test_4();
bool check_bound_test_3();
bool check_bound_test_2();
bool check_bound_test_1();
bool read_write_1();
bool read_write_2();
bool read_write_3();
bool read_write_4();
bool read_write_5();
bool read_write_6();
bool read_write_7();
bool read_write_8();
bool read_write_9();
bool read_write_10();
bool read_write_11();
bool read_write_12();
bool write_remap_read_1();
bool write_remap_read_2();
bool write_remap_read_3();
bool write_remap_read_4();
bool write_remap_read_5();

/* all functions in an array for looping */
#define TEST_COUNT 29
bool (*tests[])() = {
	uninitialise_test, /* 1 */
	initialised_test, /* 2 */
	get_size_test_1, /* 3 */
	get_size_test_2, /* 4 */
	get_size_test_3, /* 5 */
	get_size_test_4, /* 6 */
	get_size_test_5, /* 7 */
	check_bound_test_1, /* 8 */
	check_bound_test_2, /* 9 */
	check_bound_test_3, /* 10 */
	check_bound_test_4, /* 11 */
	check_bound_test_5, /* 12 */
	read_write_1, /* 13 */
	read_write_2, /* 14 */
	read_write_3, /* 15 */
	read_write_4, /* 16 */
	read_write_5, /* 17 */
	read_write_6, /* 18 */
	read_write_7, /* 19 */
	read_write_8, /* 20 */
	read_write_9, /* 21 */
	read_write_10, /* 22 */
	read_write_11, /* 23 */
	read_write_12, /* 24 */
	write_remap_read_1, /* 25 */
	write_remap_read_2, /* 26 */
	write_remap_read_3, /* 27 */
	write_remap_read_4, /* 28 */
	write_remap_read_5 /* 29 */
};

int main(int argc, char **argv) {

	int i, passed, failed;
	bool result;

	passed = 0;
	failed = 0;

	for (i = 0; i < TEST_COUNT; i++) {
		/* run test */
		result = tests[i]();
		printf("Test %i of %i.", (i + 1), TEST_COUNT);
		if (result) {
			printf(" PASSED\n");
			passed++;
		}
		else {
			printf(" FAILED!\n");
			failed++;
		}
	}

	printf("finished. %i passed. %i failed.\n", passed, failed);
	
	if (failed == 0)
		return 1;
	else
		return 0;
}

bool uninitialise_test() {

	bool b;

	dht_init();

	/* this should return false. no table initialised yet */
	b = dht_is_initialised();

	return (!b);
}


bool initialised_test() {

	bool b;

	dht_init();

	dht_init_table(0, 10, false);
	
	/* this should return true. no table initialised yet */
	b = dht_is_initialised();

	return b;

}

bool get_size_test_1() {

	int size;
	bool ok;

	dht_init();
	ok = dht_init_table(0, 10, false);

	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	size = dht_get_size();

	return (size == 10);
}

bool get_size_test_2() {

	int size;
	bool ok;

	dht_init();
	ok = dht_init_table(0, 254, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	size = dht_get_size();

	return (size == 254);
}

bool get_size_test_3() {

	int size;
	bool ok;

	dht_init();
	ok = dht_init_table(1, 5, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	size = dht_get_size();

	return (size == 4);
}

bool get_size_test_4() {

	int size;
	bool ok;

	dht_init();
	ok = dht_init_table(6, 11, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	size = dht_get_size();

	return (size == 5);
}

bool get_size_test_5() {

	int size;
	bool ok;

	dht_init();
	ok = dht_init_table(1065, 1109, false);

	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	size = dht_get_size();

	return (size == 44);
}

bool check_bound_test_1() {

	bool ok;
	dht_init();
	ok = dht_init_table(1065, 1109, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	return (dht_get_lower_bound() == 1065);

}

bool check_bound_test_2() {

	bool ok;
	dht_init();
	ok = dht_init_table(62, 1109, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	return (dht_get_lower_bound() == 62);

}

bool check_bound_test_3() {

	bool ok;
	dht_init();
	ok = dht_init_table(0, 1109, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	return (dht_get_lower_bound() == 0);

}

bool check_bound_test_4() {

	bool ok;
	dht_init();
	ok = dht_init_table(1065, 1109, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	return (dht_get_upper_bound() == 1109);

}

bool check_bound_test_5() {

	bool ok;
	dht_init();
	ok = dht_init_table(0, 2, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	return (dht_get_upper_bound() == 2);

}

bool read_write_1() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(14);

	return (j == NULL);

}

bool read_write_2() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(0);

	return (j == NULL);

}

bool read_write_3() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(1);

	return (j == NULL);

}

bool read_write_4() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(2);

	return (j == NULL);

}

bool read_write_5() {
	
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(1, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(3);

	return (j == NULL);

}

bool read_write_6() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(7);

	return (j == NULL);

}

bool read_write_7() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(16);

	return (j == NULL);

}

bool read_write_8() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(17);

	return (j == NULL);

}

bool read_write_9() {
	

	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = dht_read(18);

	return (j == NULL);

}

bool read_write_10() {
	

	int i = 65, b;
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(19, j);
	j = dht_read(19);

	b = *((int*)j);
	return (b == i);

}

bool read_write_11() {
	

	int i = 72, b;
	void *j;
	

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(19, j);
	j = dht_read(19);

	b = *((int*)j);
	return (b == i);

}

bool read_write_12() {
	

	int i = 65, b;
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(19, j);
	j = dht_read(19);

	b = *((int*)j);
	return (b == i);

}

bool write_remap_read_1() {

	int i = 65, b;
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(13, j);

	ok = dht_init_table(9, 14, true);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	j = dht_read(13);

	b = *((int*)j);
	return (b == i);

}

bool write_remap_read_2() {

	int i = 101, b;
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(1, 6, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(5, j);

	ok = dht_init_table(2, 14, true);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	j = dht_read(5);

	b = *((int*)j);
	return (b == i);

}

bool write_remap_read_3() {

	int i = 1, b;
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(101, 1600, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(171, j);

	ok = dht_init_table(100, 172, true);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	j = dht_read(171);

	b = *((int*)j);
	return (b == i);

}

bool write_remap_read_4() {

	int i = 65, b;
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(0, j);

	ok = dht_init_table(0, 4, true);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	j = dht_read(0);

	b = *((int*)j);
	return (b == i);

}

bool write_remap_read_5() {

	int i = 65;
	void *j;

	bool ok;
	dht_init();
	ok = dht_init_table(0, 20, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	/* expect null */
	j = &i;
	dht_write(0, j);

	ok = dht_init_table(0, 4, false);
	
	if (!ok) {
		printf("> malloc fail causes fail not logic fail.\n");
		return false;
	}

	j = dht_read(0);

	return (j == NULL);

}
