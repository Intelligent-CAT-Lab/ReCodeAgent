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
    unsigned int a[] = {1, 2, 3, 5, 7, 8, 9, 0, 2};
    InversionList *set = inversion_list_create(5, sizeof a / sizeof *a, a);
    assert(set == NULL);
    assert(errno == EINVAL);
  }
  inversion_list_finish();
  return EXIT_SUCCESS;
}
