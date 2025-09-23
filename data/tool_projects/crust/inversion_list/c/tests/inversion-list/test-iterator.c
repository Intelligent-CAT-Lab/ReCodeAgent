#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>

#include "../../src/inversion-list/inversion-list.h"
#include "../../src/inversion-list/inversion-list.inc"

int main(void) {
  inversion_list_init();
  {
    unsigned int a[] = {1, 2, 4, 10};

    InversionList *set =
        inversion_list_create(20, sizeof a / sizeof *a, a);

    InversionListIterator *iterator;

    for (iterator = inversion_list_iterator_create(set);
         inversion_list_iterator_valid(iterator);
         iterator = inversion_list_iterator_next(iterator)) {
      assert(inversion_list_member(set,
                                   inversion_list_iterator_get(iterator)));
    }

    inversion_list_destroy(set);
    inversion_list_iterator_destroy(iterator);
  }

  inversion_list_finish();
  return EXIT_SUCCESS;
}
