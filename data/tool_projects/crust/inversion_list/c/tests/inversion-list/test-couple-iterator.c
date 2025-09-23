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
    unsigned int a[] = {1, 2, 3, 4, 10};

    InversionList *set =
        inversion_list_create(20, sizeof a / sizeof *a, a);

    InversionListCoupleIterator *iterator;

    for (iterator = inversion_list_couple_iterator_create(set);
         inversion_list_iterator_couple_valid(iterator);
         iterator = inversion_list_couple_iterator_next(iterator)) {
      assert(set->couples[iterator->index * 2] ==
             inversion_list_couple_iterator_get_inf(iterator));

      assert(set->couples[iterator->index * 2 + 1] ==
             inversion_list_couple_iterator_get_sup(iterator));
    }

    inversion_list_destroy(set);
    inversion_list_iterator_couple_destroy(iterator);
  }

  inversion_list_finish();
  return EXIT_SUCCESS;
}
