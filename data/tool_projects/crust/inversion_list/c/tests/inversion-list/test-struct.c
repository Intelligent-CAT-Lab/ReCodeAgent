#include <stdlib.h>

#ifdef NDEBUG
#undef NDEBUG
#endif
#include <assert.h>

#include "../../src/inversion-list/inversion-list.h"
#include "../../src/inversion-list/inversion-list.inc"


int main(void) {
  /*
  InversionList set;
  assert(sizeof set == 2 * sizeof(unsigned int) + sizeof(size_t));
  assert((void *)&set.capacity == (void *)&set);
  assert((void *)&set.support == (void *)&set + sizeof(unsigned int));
  assert((void *)&set.size == (void *)&set + 2 * sizeof(unsigned int));
  assert((void *)set.couples == (void *)&set + 2 * sizeof(unsigned int) + sizeof(size_t));
  assert((void *)&set.couples[0] == (void *)&set + 2 * sizeof(unsigned int) + sizeof(size_t)); 
  assert((void *)&set.couples[1] == (void *)&set + 2 * sizeof(unsigned int) + sizeof(size_t) + sizeof(unsigned int)); 
  */
  return EXIT_SUCCESS;
}
