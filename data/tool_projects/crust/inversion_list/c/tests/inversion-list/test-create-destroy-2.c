#include <errno.h>
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
    unsigned int a[] = {1, 2, 3, 5, 7, 8, 9, 0};
    InversionList *set = inversion_list_create(20, sizeof a / sizeof *a, a);
    assert(set->capacity == 20);
    assert(set->support == 8);
    assert(set->size == 6);
    assert(set->couples[0] == 0);
    assert(set->couples[1] == 4);
    assert(set->couples[2] == 5);
    assert(set->couples[3] == 6);
    assert(set->couples[4] == 7);
    assert(set->couples[5] == 10);
    inversion_list_destroy(set);
  }
  inversion_list_finish();
  return EXIT_SUCCESS;
}
