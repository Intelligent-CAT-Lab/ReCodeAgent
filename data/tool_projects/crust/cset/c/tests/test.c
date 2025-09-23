#include <assert.h>
#include <stdio.h>

#include "../src/cset.h"

void test__cset_init() {
  Cset(int) cset_int_t;
  cset_int_t cset_int;
  cset__init(&cset_int);

  size_t size = cset__size(cset__ref(cset_int));
  assert(size == 0);

  size_t cap = cset__cap(cset__ref(cset_int));
  assert(cap == cset__INITIAL_CAP);

  cset__free(&cset_int);
}

void test__cset_add() {
  Cset(int) cset_int_t;
  cset_int_t cset_int;
  cset__init(&cset_int);

  cset__add(&cset_int, 34);
  assert(cset__size(&cset_int) == 1);

  cset__add(&cset_int, 35);
  assert(cset__size(&cset_int) == 2);
  cset__add(&cset_int, 36);
  cset__add(&cset_int, 37);
  cset__add(&cset_int, 38);

  cset__free(&cset_int);
}

void test__cset_contains() {
  Cset(int) cset_int_t;
  cset_int_t cset_int;
  cset__init(&cset_int);
  cset__add(&cset_int, 34);
  cset__add(&cset_int, 36);
  cset__remove(&cset_int, 36);

  bool contains = false;
  cset__contains(&cset_int, 12, &contains);
  assert(contains == false);

  cset__contains(&cset_int, 34, &contains);
  assert(contains == true);

  cset__add(&cset_int, 50);
  cset__contains(&cset_int, 45, &contains);
  assert(contains == false);

  assert(cset__size(&cset_int) == 2);
  cset__free(&cset_int);
}

void test__cset_iteration() {
  Cset(int) cset_int_t;
  cset_int_t cset_int;
  cset__init(&cset_int);

  for (size_t i = 0; i < 3200; i++) {
    cset__add(&cset_int, i);
  }

  Cset_iterator(cset_int_t) iterator_t;
  iterator_t iterator;
  cset_iterator__init(&iterator, &cset_int);

  for (;;) {
    if (cset_iterator__done(&iterator))
      break;
    int *buffer;
    cset_iterator__next(&iterator, buffer);

    bool contains = false;
    cset__contains(&cset_int, *buffer, &contains);
    assert(contains == true);
  }

  cset__free(&cset_int);
}

void test__cset_unique() {

  Cset(int) cset_int_t;
  cset_int_t cset_int;

  cset__init(&cset_int);

  cset__add(&cset_int, 45);
  cset__add(&cset_int, 46);
  cset__add(&cset_int, 57);

  assert(cset__size(&cset_int) == 3);

  cset__add(&cset_int, 45);
  assert(cset__size(&cset_int) == 3);
  cset__free(&cset_int);
}

typedef struct Node_t {
  int x;
  int y;
} Node;

void test__cset_struct() {

  Cset(Node) cset_node_t;
  cset_node_t cset_node;

  cset__init(&cset_node);

  cset__add(&cset_node, ((Node){.x = 4, .y = 4}));
  assert(cset__size(&cset_node) == 1);
  cset__add(&cset_node, ((Node){.x = 5, .y = 4}));
  assert(cset__size(&cset_node) == 2);

  cset__add(&cset_node, ((Node){.x = 5, .y = 4}));
  assert(cset__size(&cset_node) == 2);

  cset__add(&cset_node, ((Node){.x = 5, .y = 8}));
  assert(cset__size(&cset_node) == 3);
  cset__free(&cset_node);
}

void test__cset_remove() {
  Cset(int) cset_int_t;
  cset_int_t cset_int;

  cset__init(&cset_int);

  cset__add(&cset_int, 45);
  cset__add(&cset_int, 34);
  cset__add(&cset_int, 10);

  assert(cset__size(&cset_int) == 3);

  cset__remove(&cset_int, (int)45);
  assert(cset__size(&cset_int) == 2);

  cset__remove(&cset_int, 45);
  assert(cset__size(&cset_int) == 2);

  cset__remove(&cset_int, 34);
  assert(cset__size(&cset_int) == 1);

  Cset_iterator(cset_int_t) iterator_t;
  iterator_t iterator;

  cset_iterator__init(&iterator, &cset_int);

  for (;;) {
    if (cset_iterator__done(&iterator))
      break;

    int *value;
    cset_iterator__next(&iterator, value);
    assert(*value == 10);
  }

  cset__remove(&cset_int, 10);
  assert(cset__size(&cset_int) == 0);

  cset__free(&cset_int);
}

void test__cset_resize() {
  Cset(int) cset_int_t;
  cset_int_t cset_int;
  cset__init(&cset_int);

  for (int i = 0; i < 1500; i++) {
    cset__add(&cset_int, i);
  }
  cset__free(&cset_int);
}

void test__default_bytes_comparator() {
  Cset(int) cset_int_t;

  cset_int_t cset_int;

  cset__init(&cset_int);
  cset__add(&cset_int, 45);
  cset__add(&cset_int, 46);
  cset__add(&cset_int, 67);

  bool contains = false;
  cset__contains(&cset_int, 45, &contains);
  assert(contains == true);

  cset__contains(&cset_int, 68, &contains);
  assert(contains == false);

  cset__contains(&cset_int, 46, &contains);
  assert(contains == true);

  cset__remove(&cset_int, 46);
  cset__contains(&cset_int, 46, &contains);
  assert(contains == false);

  cset__remove(&cset_int, 46);
  cset__contains(&cset_int, 46, &contains);
  assert(contains == false);

  assert(cset__size(&cset_int) == 2);

  cset__remove(&cset_int, 45);
  assert(cset__size(&cset_int) == 1);

  cset__remove(&cset_int, 67);
  assert(cset__size(&cset_int) == 0);

  cset__remove(&cset_int, 67);
  assert(cset__size(&cset_int) == 0);

  for (int i = 0; i < 2000; i++) {
    cset__add(&cset_int, i);
  }

  assert(cset__size(&cset_int) == 2000);

  cset__free(&cset_int);
}

bool node_pointer_comparator(Node *self, Node *other) {
  return (self->x) == (other->x);
}

bool custom_node_compare(Node *self, Node *other) {
  return (self->x) == (other->x);
}

cset__u64 hash_node(Node *self, cset__u64 (*hash)(void *, size_t)) {
  return hash(&(self->x), sizeof(self->x));
}

void test__custom_comparator() {

  Cset(Node) cset_node_t;
  cset_node_t cset_node;
  cset__init(&cset_node);
  cset__set_hash(&cset_node, hash_node);
  cset__set_comparator(&cset_node, custom_node_compare);

  cset__add(&cset_node, ((Node){.x = 4, .y = 4}));
  cset__add(&cset_node, ((Node){.x = 4, .y = 4}));
  assert(cset__size(&cset_node) == 1);

  cset__add(&cset_node, ((Node){.x = 1, .y = 2}));
  assert(cset__size(&cset_node) == 2);

  cset__remove(&cset_node, ((Node){.x = 1, .y = 45}));
  assert(cset__size(&cset_node) == 1);

  cset__free(&cset_node);
}

void test__cset_clear() {
  Cset(int) cset_int_t;
  cset_int_t cset_int;

  cset__init(&cset_int);

  cset__add(&cset_int, 12);
  cset__add(&cset_int, 14);
  cset__add(&cset_int, 15);

  assert(cset__size(&cset_int) == 3);

  cset__clear(&cset_int);

  assert(cset__size(&cset_int) == 0);

  cset__add(&cset_int, 45);
  assert(cset__size(&cset_int) == 1);

  cset__free(&cset_int);
}

void test__cset_intersection() {
  Cset(int) cset_int_t;

  cset_int_t cset_int_a;
  cset_int_t cset_int_b;

  cset__init(&cset_int_a);
  cset__init(&cset_int_b);

  cset__add(&cset_int_a, 12);
  cset__add(&cset_int_a, 13);
  cset__add(&cset_int_a, 14);

  cset__add(&cset_int_b, 12);
  cset__add(&cset_int_b, 13);
  cset__add(&cset_int_b, 16);

  cset_int_t result;

  cset__init(&result);

  cset__intersect(&result, &cset_int_a, &cset_int_b);

  assert(cset__size(&result) == 2);

  cset__add(&cset_int_b, 14);
  cset__intersect(&result, &cset_int_a, &cset_int_b);
  assert(cset__size(&result) == 3);

  cset__free(&cset_int_a);
  cset__free(&cset_int_b);
  cset__free(&result);
}

void test__cset_union() {
  Cset(int) cset_int_t;

  cset_int_t cset_int_a;
  cset_int_t cset_int_b;
  cset_int_t cset_result;

  cset__init(&cset_int_a);
  cset__init(&cset_int_b);
  cset__init(&cset_result);

  cset__add(&cset_int_a, 34);
  cset__add(&cset_int_a, 25);
  cset__add(&cset_int_a, 12);

  cset__add(&cset_int_b, 1);
  cset__add(&cset_int_b, 4);
  cset__add(&cset_int_b, 34);

  cset__union(&cset_result, &cset_int_a, &cset_int_b);

  assert(cset__size(&cset_result) == 5);

  cset__add(&cset_int_b, 100);
  cset__union(&cset_result, &cset_int_a, &cset_int_b);

  assert(cset__size(&cset_result) == 6);

  cset__free(&cset_int_a);
  cset__free(&cset_int_b);
  cset__free(&cset_result);
}

void test__cset_disjoint() {

  Cset(char) cset_char_t;

  cset_char_t cset_a;
  cset_char_t cset_b;

  cset__init(&cset_a);
  cset__init(&cset_b);

  cset__add(&cset_a, 'a');
  cset__add(&cset_a, 'b');

  cset__add(&cset_b, 'c');
  cset__add(&cset_b, 'd');

  bool disjoint;
  cset__is_disjoint(&cset_a, &cset_b, &disjoint);

  assert(disjoint == true);

  cset__add(&cset_b, 'a');
  cset__is_disjoint(&cset_a, &cset_b, &disjoint);

  assert(disjoint == false);

  cset__free(&cset_a);
  cset__free(&cset_b);
}

void test__cset_difference() {
  Cset(int) cset_int_t;

  cset_int_t cset_a;
  cset_int_t cset_b;

  cset__init(&cset_a);
  cset__init(&cset_b);

  cset_int_t result;
  cset__init(&result);

  cset__difference(&result, &cset_a, &cset_b);

  assert(cset__size(&result) == 0);

  cset__add(&cset_a, 45);
  cset__add(&cset_a, 46);
  cset__add(&cset_a, 58);

  cset__add(&cset_b, 12);
  cset__add(&cset_b, 11);
  cset__add(&cset_b, 45);

  cset__difference(&result, &cset_a, &cset_b);

  assert(cset__size(&result) == 2);

  bool contains = false;

  cset__contains(&result, 46, &contains);
  assert(contains == true);

  cset__contains(&result, 58, &contains);
  assert(contains == true);

  cset__contains(&result, 45, &contains);
  assert(contains == false);

  cset__clear(&result);

  cset__add(&cset_b, 46);
  cset__add(&cset_b, 58);

  cset__difference(&result, &cset_a, &cset_b);
  assert(cset__size(&result) == 0);

  cset__difference(&result, &cset_b, &cset_a);
  assert(cset__size(&result) == 2);

  cset__free(&cset_a);
  cset__free(&cset_b);
  cset__free(&result);
}

int main() {
  test__cset_init();
  test__cset_add();
  test__cset_contains();
  test__cset_iteration();
  test__cset_unique();
  test__cset_clear();
  test__cset_remove();
  test__cset_resize();
  test__cset_struct();
  test__custom_comparator();
  test__default_bytes_comparator();
  test__cset_intersection();
  test__cset_union();
  test__cset_disjoint();
  test__cset_difference();
}
