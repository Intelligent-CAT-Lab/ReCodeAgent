#include "hashmap.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <testing/tassert.h>
#include <testing/test_utils.h>

unsigned fnva1(char *value) {
    unsigned long h = 16777619;
    long int prime = 2166136261;

    while (*value != '\0') {
        h ^= *value;
        h *= prime;
        ++value;
    }

    return h;
}

unsigned equal_key(char *a, char *b) { return strcmp(a, b) == 0; }

struct Hashmap *create_hashmap(int capacity) {
    struct Hashmap *h = malloc(sizeof(struct Hashmap));

    h->buckets = calloc(capacity, sizeof(struct BucketNode *));

    h->size = 0;
    h->cap = capacity;

    h->hash = fnva1;
    h->equals = equal_key;

    return h;
}

void destroy_hashmap(struct Hashmap *h) {
    free(h->buckets);
    free(h);
}

struct BucketNode *create_bucket(char *key, void *value) {
    struct BucketNode *b = malloc(sizeof(struct BucketNode));

    b->key = key;
    b->value = value;
    b->next = NULL;

    return b;
}

struct BucketNode *hm_get(struct Hashmap *h, char *key) {
    unsigned a = h->hash(key) % h->cap;

    struct BucketNode *b = h->buckets[a];

    if (b == NULL) {
        return NULL;
    }

    if (h->equals(b->key, key)) {
        return b;
    }

    // check for linear probing

    return NULL;
}

int hm_set(struct Hashmap *h, char *key, void *value) {
    unsigned a = h->hash(key) % h->cap;

    struct BucketNode *b = h->buckets[a];

    if (b == NULL) {
        if (h->size == h->cap) {
            double_cap(h);
        }

        h->size++;

        h->buckets[a] = malloc(sizeof(struct BucketNode *));
        h->buckets[a]->key = key;
        h->buckets[a]->value = value;
        h->buckets[a]->next = NULL;

        return 0;
    } else {
        // Handle linear probing
        return -1;
    }
}

void double_cap(struct Hashmap *h) {
    struct BucketNode **new_buckets =
        calloc(h->cap * 2, sizeof(struct BucketNode *));

    for (int i = 0; i < h->cap; i++) {

        if (h->buckets[i] != NULL) {
            struct BucketNode *b = h->buckets[i];
            unsigned a = h->hash(b->key) % h->cap;
            new_buckets[a] = b;
        }
    }

    h->buckets = new_buckets;

    h->cap = h->cap * 2;
}

int test_hash_init() {
    testing_func_setup();
    struct Hashmap *h = create_hashmap(100);

    tassert(h->size == 0);
    tassert(h->cap == 100);
}

int test_hash_init_and_store() {
    testing_func_setup();
    struct Hashmap *h = create_hashmap(100);

    tassert(h->size == 0);
    tassert(h->cap == 100);

    char name[5] = "jake";

    char key[5] = "test";
    int ret = hm_set(h, key, name);
    tassert(ret != -1);

    uint64_t ind = h->hash(key) % h->cap;
    struct BucketNode *b = h->buckets[ind];
    tassert(strcmp(b->key, key) == 0);

    tassert(h->size == 1);
    tassert(h->cap == 100);
}

int test_hash_set_and_get() {
    testing_func_setup();
    struct Hashmap *h = create_hashmap(100);

    char name[100] = "jake";
    char key[10] = "test";

    int ret = hm_set(h, key, name);
    tassert(ret != -1);

    struct BucketNode *got = hm_get(h, "test");
    tassert(strcmp(got->value, "jake") == 0);

    return 0;
}

int test_hash_set_and_double_get() {
    testing_func_setup();
    struct Hashmap *h = create_hashmap(100);

    char name[100] = "jake";
    char key[10] = "test";

    int ret = hm_set(h, key, name);
    tassert(ret != -1);

    double_cap(h);

    struct BucketNode *got = hm_get(h, "test");
    tassert(strcmp(got->value, "jake") == 0);

    return 0;
}
