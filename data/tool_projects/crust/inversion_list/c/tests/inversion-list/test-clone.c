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
    unsigned int a[] = {1, 2, 3, 5, 7, 8, 9, 0, 2};
    InversionList *set = inversion_list_create(20, sizeof a / sizeof *a, a);
    InversionList *clone = inversion_list_clone(set);
    inversion_list_destroy(set);
    assert(clone->capacity == 20);
    assert(clone->support == 8);
    assert(clone->size == 6);
    assert(clone->couples[0] == 0);
    assert(clone->couples[1] == 4);
    assert(clone->couples[2] == 5);
    assert(clone->couples[3] == 6);
    assert(clone->couples[4] == 7);
    assert(clone->couples[5] == 10);
    inversion_list_destroy(clone);
  }
  inversion_list_finish();
  return EXIT_SUCCESS;
}
