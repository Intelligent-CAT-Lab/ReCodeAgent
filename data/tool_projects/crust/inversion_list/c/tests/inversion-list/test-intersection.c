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
    unsigned int i;
    unsigned int a[] = {1, 2, 3, 5, 7, 9};
    unsigned int b[] = {1, 2, 3, 5, 7, 9, 10};
    unsigned int c[] = {23, 12, 1};

    InversionList *set =
        inversion_list_create(20, sizeof a / sizeof *a, a);
    InversionList *set1;
    InversionList *null = NULL;

    InversionList *set2 =
        inversion_list_create(20, sizeof b / sizeof *b, b);
    InversionList *set3 =
        inversion_list_create(20, sizeof c / sizeof *c, c);

    set1 = inversion_list_intersection(set, set2, set3, null);
    assert(strcmp(inversion_list_to_string(set1), "[1, 2, 3, 5, 7, 9]") ==
           0);
    inversion_list_destroy(set1);

    set1 = inversion_list_intersection(set, set2, null);
    assert(strcmp(inversion_list_to_string(set1), "[1, 2, 3, 5, 7, 9]") ==
           0);

    inversion_list_destroy(null);
    inversion_list_destroy(set);
    inversion_list_destroy(set1);
    inversion_list_destroy(set2);
    inversion_list_destroy(set3);
  }

  inversion_list_finish();
  return EXIT_SUCCESS;
}
