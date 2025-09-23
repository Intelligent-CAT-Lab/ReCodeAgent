#include <stdio.h>
#include <stdlib.h>

#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>

#include "../../src/inversion-list/inversion-list.h"
#include "../../src/inversion-list/inversion-list.inc"

int main(void) {
  inversion_list_init();
  {
    unsigned int i;
    unsigned int a[] = {1, 2, 3, 5, 7, 8, 9};
    unsigned int b[] = {1, 2, 3, 9, 8};
    unsigned int c[] = {1, 2, 3, 5, 7, 9, 10};

    InversionList *set =
        inversion_list_create(20, sizeof a / sizeof *a, a);
    InversionList *set1 = inversion_list_clone(set);

    InversionList *set2 =
        inversion_list_create(20, sizeof b / sizeof *b, b);
    InversionList *set3 =
        inversion_list_create(20, sizeof c / sizeof *c, c);

    // test equal
    assert(inversion_list_equal(set, set1));
    assert(inversion_list_equal(set, set));
    assert(!inversion_list_equal(set, set2));
    assert(!inversion_list_equal(set, set3));
    assert(!inversion_list_equal(set2, set3));

    // test not equal
    assert(!inversion_list_not_equal(set, set1));
    assert(!inversion_list_not_equal(set, set));
    assert(inversion_list_not_equal(set, set2));
    assert(inversion_list_not_equal(set, set3));
    assert(inversion_list_not_equal(set2, set3));

    // test less
    assert(!inversion_list_less(set, set1));
    assert(!inversion_list_less(set, set));
    assert(!inversion_list_less(set, set2));
    assert(!inversion_list_less(set, set3));
    assert(inversion_list_less(set2, set3));

    // test less-equal
    assert(inversion_list_less_equal(set, set1));
    assert(inversion_list_less_equal(set, set));
    assert(!inversion_list_less_equal(set, set2));
    assert(!inversion_list_less_equal(set, set3));
    assert(inversion_list_less_equal(set2, set3));

    // test greater
    assert(!inversion_list_greater(set, set1));
    assert(!inversion_list_greater(set, set));
    assert(inversion_list_greater(set, set2));
    assert(!inversion_list_greater(set3, set));
    assert(!inversion_list_greater(set2, set3));

    // test greater-equal
    assert(inversion_list_greater_equal(set, set1));
    assert(inversion_list_greater_equal(set, set));
    assert(inversion_list_greater_equal(set, set2));
    assert(!inversion_list_greater_equal(set3, set));
    assert(!inversion_list_greater_equal(set2, set3));

    inversion_list_destroy(set);
    inversion_list_destroy(set1);
    inversion_list_destroy(set2);
    inversion_list_destroy(set3);
  }

  inversion_list_finish();
  return EXIT_SUCCESS;
}
