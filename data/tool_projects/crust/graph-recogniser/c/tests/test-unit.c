#include "test-unit.h"

#include "log.h"

int main() {
        change_log_priority(OPEN_HASH_LOG, 11);
        change_log_priority(CUCKOO_HASH_LOG, 11);

	if (test_openhash())
		insert_log(UNIT_TEST_LOG, 10, "Testing open hashing ... FAIL");
	else
		insert_log(UNIT_TEST_LOG, 10, "Testing open hashing ... OK");

	if (test_cuckoohash())
		insert_log(UNIT_TEST_LOG, 10, "Testing cuckoo hashing ... FAIL");
	else
		insert_log(UNIT_TEST_LOG, 10, "Testing cuckoo hashing ... OK");

	return 0;
}

