#include <assert.h>
#include <errno.h>
#include <math.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../inversion-list/inversion-list.h"  // NOLINT(build/include)
#include "inversion-list.inc"

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

static void *_buffer = NULL;
static size_t _size = 0;

static void *_get_buffer(size_t size) {
  if (size > _size) {
    void *buffer = realloc(_buffer, size);
    if (buffer) {
      _buffer = buffer;
      _size = size;
    } else {
      errno = ENOMEM;
      return NULL;
    }
  }
  return _buffer;
}

static int _counter = 0;

static void _verify(void) { assert(_counter == 0); }

bool inversion_list_init(void) {
  static bool first = true;
  if (first) {
    atexit(_verify);
    first = false;
  }
  if (!_counter++) {
    assert(printf("Inversion list initialised\n"));
  }
  return true;
}

bool inversion_list_finish(void) {
  if (_counter) {
    if (!--_counter) {
      free(_buffer);
      _buffer = NULL;
      _size = 0;

      inversion_list_to_string(NULL);
      assert(printf("Inversion list finished\n"));
    }
    return true;
  } else {
    return false;
  }
}

static int _compare_unsigned_int(const void *value1, const void *value2) {
  return *(unsigned int *)value1 -
         *(unsigned int *)value2;  // NOLINT(readability/casting)
}

InversionList *inversion_list_create(unsigned int capacity, size_t count,
                                     const unsigned int *values) {
  assert(_counter > 0);

  unsigned int *buffer = _get_buffer(count * sizeof(unsigned int));
  if (!buffer) {
    return NULL;
  }

  unsigned int i;
  size_t size;
  memcpy(buffer, values,
         count * sizeof(unsigned int));  // flawfinder: ignore
  unsigned int support = 0;
  if (count) {
    qsort(buffer, count, sizeof *buffer, _compare_unsigned_int);
    if (buffer[count - 1] >= capacity) {
      errno = EINVAL;
      return NULL;
    }
    size = 2;
    support++;
    for (i = 1; i < count; i++) {
      if (buffer[i] != buffer[i - 1]) {
        support++;
        if (buffer[i] > buffer[i - 1] + 1) {
          size += 2;
        }
      }
    }
  } else {
    size = 0;
  }

  InversionList *set =
      malloc(sizeof(InversionList) + size * sizeof(unsigned int));
  if (set) {
    set->size = size;
    set->capacity = capacity;
    set->support = support;
    if (size) {
      i = 0;
      unsigned int current = 0;
      set->couples[current] = buffer[i];
      current++;
      set->couples[current] = buffer[i] + 1;
      for (i = 1; i < count; i++) {
        if (buffer[i] == set->couples[current]) {
          set->couples[current]++;
        } else if (buffer[i] > set->couples[current]) {
          current++;
          set->couples[current] = buffer[i];
          current++;
          set->couples[current] = buffer[i] + 1;
        }
      }
    }
    return set;
  } else {
    errno = ENOMEM;
    return NULL;
  }
}

void inversion_list_destroy(InversionList *set) {
  assert(_counter > 0);

  free(set);
}

unsigned int inversion_list_capacity(const InversionList *set) {
  assert(_counter > 0);

  return set->capacity;
}

unsigned int inversion_list_support(const InversionList *set) {
  assert(_counter > 0);

  return set->support;
}

static int _index_of(const void *element, const void *base, size_t size) {
  return (element - base) / size;
}

static int _sign;
static bool _strict;
static const void *_found;
static int (*_compare_func)(const void *, const void *);
static int _search(const void *key, const void *value) {
  int compare = _compare_func(key, value);
  if (_strict) {
    if (_sign * compare > 0) {
      _found = value;
      compare = _sign;
    } else {
      compare = -_sign;
    }
  } else {
    if (_sign * compare >= 0) {
      _found = value;
      compare = _sign;
    }
  }
  return compare;
}

static const void *_bound(const void *key, const void *base, size_t nmemb,
                          size_t size,
                          int (*compar)(const void *, const void *),
                          int sign, bool strict) {
  _compare_func = compar;
  _found = NULL;
  _sign = sign;
  _strict = strict;
  bsearch(key, base, nmemb, size, _search);
  return _found;
}

static const void *_lower_bound(const void *key, const void *base,
                                size_t nmemb, size_t size,
                                int (*compar)(const void *, const void *),
                                bool strict) {
  return _bound(key, base, nmemb, size, compar, 1, strict);
}

static const void *_upper_bound(const void *key, const void *base,
                                size_t nmemb, size_t size,
                                int (*compar)(const void *, const void *),
                                bool strict) {
  return _bound(key, base, nmemb, size, compar, -1, strict);
}

bool inversion_list_member(const InversionList *set, unsigned int value) {
  assert(_counter > 0);

  const void *bound =
      _lower_bound(&value, set->couples, set->size, sizeof(unsigned int),
                   _compare_unsigned_int, false);
  if (bound) {
    return _index_of(bound, set->couples, sizeof(unsigned int)) % 2 == 0;
  } else {
    return false;
  }
}

InversionList *inversion_list_clone(const InversionList *set) {
  assert(_counter > 0);

  InversionList *clone =
      malloc(sizeof(InversionList) + set->size * sizeof(unsigned int));
  if (clone) {
    memcpy(clone, set,
           sizeof(InversionList) +
               set->size * sizeof(unsigned int));  // flawfinder: ignore
    return clone;
  } else {
    errno = ENOMEM;
    return NULL;
  }
}

InversionList *inversion_list_complement(const InversionList *set) {
  assert(_counter > 0);

  size_t size = set->size;
  if (set->couples[0] == 0 &&
      set->couples[set->size - 1] == set->capacity) {
    size -= 2;
  } else if (set->couples[0] != 0 &&
             set->couples[set->size - 1] != set->capacity) {
    size += 2;
  }

  InversionList *complement =
      malloc(sizeof(InversionList) + size * sizeof(unsigned int));
  if (complement) {
    complement->size = size;
    complement->capacity = set->capacity;
    complement->support = set->capacity - set->support;
    if (set->couples[0] == 0) {
      if (set->couples[set->size - 1] == set->capacity) {
        memcpy(complement->couples, set->couples + 1,
               size * sizeof(unsigned int));  // flawfinder: ignore
      } else {
        memcpy(complement->couples, set->couples + 1,
               (size - 1) * sizeof(unsigned int));  // flawfinder: ignore
        complement->couples[size - 1] = set->capacity;
      }
    } else {
      complement->couples[0] = 0;
      if (set->couples[set->size - 1] == set->capacity) {
        memcpy(complement->couples + 1, set->couples,
               (size - 1) * sizeof(unsigned int));  // flawfinder: ignore
      } else {
        memcpy(complement->couples + 1, set->couples,
               (size - 2) * sizeof(unsigned int));  // flawfinder: ignore
        complement->couples[size - 1] = set->capacity;
      }
    }
    return complement;
  } else {
    errno = ENOMEM;
    return NULL;
  }
}

static bool _add_string(char **pstring, unsigned int *plength,
                        const char *add) {
  if (add) {
    unsigned int inc_length = strlen(add);  // flawfinder: ignore
    char *string = realloc(*pstring, *plength + inc_length + 1);
    if (string) {
      *pstring = string;
      strncpy(*pstring + *plength, add,
              inc_length + 1);  // flawfinder: ignore
      *plength += inc_length;
      return true;
    } else {
      errno = ENOMEM;
      return false;
    }
  } else {
    return false;
  }
}

const char *inversion_list_to_string(const InversionList *set) {
  static char *string = NULL;
  unsigned int length = 0;

  if (!set) {
    free(string);
    string = NULL;
    return NULL;
  }

  assert(_counter > 0);

  if (!_add_string(&string, &length, "[")) {
    return NULL;
  }

  unsigned int i;
  bool first = true;
  for (i = 0; i < set->size; i += 2) {
    unsigned int value;
    for (value = set->couples[i]; value < set->couples[i + 1]; value++) {
      if (first) {
        first = false;
      } else {
        if (!_add_string(&string, &length, ", ")) {
          return NULL;
        }
      }
      static char current[100];  // flawfinder: ignore
      snprintf(current, sizeof current, "%u", value);
      if (!_add_string(&string, &length, current)) {
        return NULL;
      }
    }
  }

  if (!_add_string(&string, &length, "]")) {
    return NULL;
  }
  return string;  // NOLINT(build/include)
}

bool _in_list(const InversionList *set1, const InversionList *set2) {
  unsigned int i;
  for (i = 0; i < set1->size; i++) {
    if (set2->couples[i] != set1->couples[i]) {
      return false;
    }
  }

  return true;
}

bool inversion_list_equal(const InversionList *set1,
                          const InversionList *set2) {
  if (inversion_list_support(set1) != inversion_list_support(set2) ||
      set1->size != set2->size) {
    return false;
  }

  return _in_list(set1, set2);
}

bool inversion_list_not_equal(const InversionList *set1,
                              const InversionList *set2) {
  return !inversion_list_equal(set1, set2);
}

/***************EXERCICE  2*******************/
bool inversion_list_less(const InversionList *set1,
                         const InversionList *set2) {
  if (inversion_list_support(set1) >= inversion_list_support(set2)) {
    return 0;
  }
  unsigned int i = 0;
  unsigned int count = 0;
  // A AMELIORER

  while (i < set1->couples[set1->size - 1] && count < 1) {
    if (inversion_list_member(set2, i)) {
      count++;
    }
    i++;
  }
  return count;
}

bool inversion_list_less_equal(const InversionList *set1,
                               const InversionList *set2) {
  return inversion_list_equal(set1, set2) ||
         inversion_list_less(set1, set2);
}

bool inversion_list_greater(const InversionList *set1,
                            const InversionList *set2) {
  return inversion_list_less(set2, set1);
}
bool inversion_list_greater_equal(const InversionList *set1,
                                  const InversionList *set2) {
  return inversion_list_less_equal(set2, set1);
}

bool inversion_list_disjoint(const InversionList *set1,
                             const InversionList *set2) {
  return inversion_list_not_equal(set1, set2);
}

/***************EXERCICE  3*******************/

static InversionList *_union(const InversionList *set1,
                             const InversionList *set2) {
  if (set2 == NULL) {
    return inversion_list_clone(set2);
  }
  if (set1 == NULL) {
    return inversion_list_clone(set1);
  }

  unsigned int *buff =
      _get_buffer((set1->support + set2->support) * sizeof(unsigned int));

  if (!buff) {
    return NULL;
  }

  unsigned int max;

  unsigned int i = 0;
  unsigned int j = 0;

  unsigned int cap = MAX(set1->capacity, set2->capacity);

  max = MAX(set1->couples[set1->size - 1], set2->couples[set2->size - 1]);
  i = MIN(set1->couples[0], set2->couples[0]);

  while (i <= max) {
    if (inversion_list_member(set1, i) || inversion_list_member(set2, i)) {
      buff[j++] = i;
    }
    i++;
  }

  return inversion_list_create(cap, j, buff);
}

static InversionList *_intersection(const InversionList *set1,
                                    const InversionList *set2) {
  if (set2 == NULL) {
    return inversion_list_clone(set1);
  }
  if (set1 == NULL) {
    return inversion_list_clone(set2);
  }

  unsigned int *buff =
      _get_buffer((set1->support + set2->support) * sizeof(unsigned int));
  if (!buff) {
    return NULL;
  }

  unsigned int j = 0;
  unsigned int i2 = 0;
  unsigned int cap = MAX(set1->capacity, set2->capacity);

  unsigned int i = 0;
  unsigned int set1_size = set1->size;
  unsigned int set2_size = set2->size;

  while (i < set1_size && i2 < set2_size) {
    unsigned int set1_min = set1->couples[i];
    unsigned int set1_max = set1->couples[i + 1];

    unsigned int set2_min = set2->couples[i2];
    unsigned int set2_max = set2->couples[i2 + 1];

    unsigned int min = MAX(set1_min, set2_min);
    unsigned int max = MIN(set1_max, set2_max);

    if (min <= max) {
      for (unsigned int k = min; k <= max; k++) {
        if (inversion_list_member(set1, k) &&
            inversion_list_member(set2, k)) {
          buff[j++] = k;
        }
      }
    }

    if (set1_max < set2_max) {
      i += 2;
    } else if (set1_max > set2_max) {
      i2 += 2;
    } else {
      i += 2;
      i2 += 2;
    }
  }

  return inversion_list_create(cap, j, buff);
}

static InversionList *_difference(const InversionList *set1,
                                  const InversionList *set2) {
  if (set2 == NULL) {
    return inversion_list_clone(set1);
  }
  if (set1 == NULL) {
    return inversion_list_clone(set2);
  }

  unsigned int *buff =
      _get_buffer((set1->support + set2->support) * sizeof(unsigned int));
  if (!buff) {
    return NULL;
  }

  unsigned int j = 0;
  unsigned int i2 = 0;
  unsigned int cap = MAX(set1->capacity, set2->capacity);

  unsigned int i = 0;
  unsigned int set1_size = set1->size;
  unsigned int set2_size = set2->size;

  while (i < set1_size && i2 < set2_size) {
    unsigned int set1_min = set1->couples[i];
    unsigned int set1_max = set1->couples[i + 1];

    unsigned int set2_min = set2->couples[i2];
    unsigned int set2_max = set2->couples[i2 + 1];

    unsigned int min = MAX(set1->couples[0], set1->couples[0]);
    unsigned int max = MIN(set1_max, set2_max);

    if (min <= max) {
      for (unsigned int k = min; k <= max; k++) {
        if (inversion_list_member(set1, k) &&
            !inversion_list_member(set2, k)) {
          buff[j++] = k;
        }
      }
    }

    if (set1_max < set2_max) {
      for (unsigned int k = set1_min; k < set2_max; k++) {
        if (inversion_list_member(set1, k) &&
            !inversion_list_member(set2, k)) {
          buff[j++] = k;
        }
      }
      i += 2;
    } else if (set1_max > set2_max) {
      i2 += 2;
    } else {
      i += 2;
      i2 += 2;
    }
  }

  return inversion_list_create(cap, j, buff);
}


/***************************EXERCICE4***************************/

InversionList *inversion_list_union(const InversionList *set, ...) {
  const InversionList *arg;

  InversionList *res = NULL;
  InversionList *temp = inversion_list_clone(set);
  va_list args;
  unsigned int i = 0;

  va_start(args, set);
  unsigned int count = 0;

  while ((arg = va_arg(args, const InversionList *))) {
    inversion_list_destroy(res);

    res = _union(temp, arg);

    inversion_list_destroy(temp);
    temp = inversion_list_clone(res);
  }

  va_end(args);

  inversion_list_destroy(temp);

  return res;
}

InversionList *inversion_list_intersection(const InversionList *set, ...) {
  const InversionList *arg;
  InversionList *res = NULL;
  InversionList *temp = inversion_list_clone(set);

  va_list args;
  va_start(args, set);

  while ((arg = va_arg(args, const InversionList *))) {
    inversion_list_destroy(res);

    res = _intersection(temp, arg);

    inversion_list_destroy(temp);
    temp = inversion_list_clone(res);
  }

  va_end(args);
  inversion_list_destroy(temp);

  return res;
}

InversionList *inversion_list_difference(const InversionList *set, ...) {
  const InversionList *arg;
  InversionList *res;
  InversionList *temp = NULL;

  va_list args;
  va_start(args, set);

  while ((arg = va_arg(args, const InversionList *))) {
    if (inversion_list_not_equal(set, arg)) {
      if (temp == NULL) {
        inversion_list_destroy(temp);
        temp = inversion_list_clone(arg);
      } else {
        res = _union(temp, arg);

        inversion_list_destroy(temp);
        temp = inversion_list_clone(res);

        inversion_list_destroy(res);
      }
    }
  }
  va_end(args);
  res = _difference(set, temp);

  inversion_list_destroy(temp);
  return res;
}

/***************************EXERCICE3***************************/
InversionList *inversion_list_symmetric_difference(
    const InversionList *set1, const InversionList *set2) {
  InversionList *null = NULL;
  InversionList *i = inversion_list_intersection(set1, set2, null);
  InversionList *u = inversion_list_union(set1, set2, null);
  InversionList *res = inversion_list_difference(u, i, null);

  inversion_list_destroy(i);
  inversion_list_destroy(u);
  inversion_list_destroy(null);

  return res;
}

/***************************EXERCICE 6***************************/

InversionListIterator *inversion_list_iterator_create(
    const InversionList *set) {
  assert(_counter > 0);
  InversionListIterator *iterator = malloc(sizeof(InversionListIterator));
  if (iterator) {
    iterator->set = set;

    iterator->index = 0;
  }
  return iterator;
}

void inversion_list_iterator_destroy(InversionListIterator *iterator) {
  assert(_counter > 0);
  free(iterator);
}

InversionListIterator *inversion_list_iterator_next(
    InversionListIterator *iterator) {
  assert(_counter > 0);
  iterator->index++;
  return iterator;
}

InversionListIterator *inversion_list_iterator_rewind(
    InversionListIterator *iterator) {
  assert(_counter > 0);
  iterator->index = 0;
  return iterator;
}

bool inversion_list_iterator_valid(const InversionListIterator *iterator) {
  assert(_counter > 0);
  return iterator->index < iterator->set->couples[iterator->set->size - 1];
}

unsigned int inversion_list_iterator_get(InversionListIterator *iterator) {
  if (iterator) {
    if (inversion_list_member(iterator->set, iterator->index)) {
      return iterator->index;
    } else {
      return inversion_list_iterator_get(
          inversion_list_iterator_next(iterator));
    }
  }
  return iterator->set->support;
}

/***************************EXERCICE 7***************************/
InversionListCoupleIterator *inversion_list_couple_iterator_create(
    const InversionList *set) {
  assert(_counter > 0);
  InversionListCoupleIterator *iterator =
      malloc(sizeof(InversionListIterator));
  if (iterator) {
    iterator->set = set;
    iterator->index = 0;
  }
  return iterator;
}

void inversion_list_iterator_couple_destroy(
    InversionListIterator *iterator) {
  assert(_counter > 0);
  free(iterator);
}

InversionListCoupleIterator *inversion_list_couple_iterator_next(
    InversionListCoupleIterator *iterator) {
  assert(_counter > 0);
  iterator->index++;
  return iterator;
}

InversionListCoupleIterator *inversion_list_couple_iterator_rewind(
    InversionListCoupleIterator *iterator) {
  assert(_counter > 0);
  iterator->index = 0;
  return iterator;
}

bool inversion_list_iterator_couple_valid(
    const InversionListCoupleIterator *iterator) {
  assert(_counter > 0);
  return iterator->index < iterator->set->size / 2;
}

unsigned int inversion_list_couple_iterator_get_inf(
    const InversionListCoupleIterator *iterator) {
  return iterator->set->couples[iterator->index * 2];
}

unsigned int inversion_list_couple_iterator_get_sup(
    const InversionListCoupleIterator *iterator) {
  return iterator->set->couples[iterator->index * 2 + 1];
}
