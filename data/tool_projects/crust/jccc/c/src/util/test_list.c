#include "test_list.h"
#include <testing/tassert.h>
#include <testing/test_utils.h>

// A particular application I want to test out.
int basic_100_test() {
    static int BS = 10, TS = 100;
    List* l = create_list(BS);
    if (!l) return 0;
    for (int i = 0; i < TS; ++i) {
        // Technically the addition needs to be a void pointer, but this works
        // too ... right?
        ladd_element(l, (void*) i);
    }
    // Now make sure that when we iterate, we get all the right answers back in
    // the right order.
    for (int i = 0; i < TS; ++i) {
        tassert((int) lget_element(l, i) == i);
    }
    destroy_list(l);
    return 0;
}

// The actual list test in its entirety.
int test_list() {
    testing_module_setup();

    basic_100_test();

    testing_module_cleanup();
    return 0;
}
