#include "hashmap.h"
#include <testing/test_utils.h>

int test_util() {
    testing_module_setup();

    test_hash_init();
    test_hash_init_and_store();
    test_hash_set_and_get();
	test_hash_set_and_double_get();

    testing_module_cleanup();
    return 0;
}
