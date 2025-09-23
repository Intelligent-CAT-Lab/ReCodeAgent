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
    unsigned int a[] = {1, 2, 3, 5, 7, 8, 9, 2};
    InversionList *set = inversion_list_create(20, sizeof a / sizeof *a, a);
    unsigned int i;
    printf("%d: %d\n", 0, inversion_list_member(set, 0));
    assert(!inversion_list_member(set, 0));
    printf("%d: %d\n", 4, inversion_list_member(set, 4));
    assert(!inversion_list_member(set, 4));
    printf("%d: %d\n", 10, inversion_list_member(set, 10));
    assert(!inversion_list_member(set, 10));
    printf("%d: %d\n", 11, inversion_list_member(set, 11));
    assert(!inversion_list_member(set, 11));
    for (i = 0; i < sizeof a / sizeof *a; i++) {
      printf("%d: %d\n", a[i], inversion_list_member(set, a[i]));
      assert(inversion_list_member(set, a[i]));
    }
    inversion_list_destroy(set);
  }
  inversion_list_finish();
  return EXIT_SUCCESS;
}
