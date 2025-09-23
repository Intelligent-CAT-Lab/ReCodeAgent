#include<stdlib.h>
#include<stdio.h>
#include "btree.h"
#include <assert.h>
#include <string.h>


void test_add_entry() {
    btree_t* btree = new_btree();
    add_entry(btree, "entry_1", sizeof("entry_1"), "value_1", sizeof("value_1"));
    value_t* value = find_entry(btree, "entry_1", sizeof("entry_1"));
    assert(value != NULL);
    assert(memcmp(value->value, "value_1", sizeof("value_1")) == 0);
    free_tree(&btree);
}

void test_entry_list() {
    btree_t* btree = new_btree();
    add_entry(btree, "entry_1", sizeof("entry_1"), "value_1", sizeof("value_1"));
    add_entry(btree, "entry_2", sizeof("entry_2"), "value_2", sizeof("value_2"));
    add_entry(btree, "entry_3", sizeof("entry_3"), "value_3", sizeof("value_3"));
    add_entry(btree, "entry_4", sizeof("entry_4"), "value_4", sizeof("value_4"));
    add_entry(btree, "entry_5", sizeof("entry_5"), "value_5", sizeof("value_5"));
    entry_list_t* list = list_entries(btree);
    assert(list->len == 5);
    assert(memcmp(list->entries[0].key.key, "entry_1", sizeof("entry_1")) == 0);
    assert(memcmp(list->entries[0].value.value, "value_1", sizeof("value_1")) == 0);
    assert(memcmp(list->entries[1].key.key, "entry_2", sizeof("entry_2")) == 0);
    assert(memcmp(list->entries[1].value.value, "value_2", sizeof("value_2")) == 0);
    assert(memcmp(list->entries[2].key.key, "entry_3", sizeof("entry_3")) == 0);
    assert(memcmp(list->entries[2].value.value, "value_3", sizeof("value_3")) == 0);
    assert(memcmp(list->entries[3].key.key, "entry_4", sizeof("entry_4")) == 0);
    assert(memcmp(list->entries[3].value.value, "value_4", sizeof("value_4")) == 0);
    assert(memcmp(list->entries[4].key.key, "entry_5", sizeof("entry_5")) == 0);
    assert(memcmp(list->entries[4].value.value, "value_5", sizeof("value_5")) == 0);
    free_tree(&btree);
    free_entry_list(&list);
}

void test_remove_entry() {
    btree_t* btree = new_btree();
    add_entry(btree, "entry_1", sizeof("entry_1"), "value_1", sizeof("value_1"));
    add_entry(btree, "entry_2", sizeof("entry_2"), "value_2", sizeof("value_2"));
    add_entry(btree, "entry_3", sizeof("entry_3"), "value_3", sizeof("value_3"));
    add_entry(btree, "entry_4", sizeof("entry_4"), "value_4", sizeof("value_4"));
    add_entry(btree, "entry_5", sizeof("entry_5"), "value_5", sizeof("value_5"));
    remove_entry(btree, "entry_3", sizeof("entry_3"));
    entry_list_t* list = list_entries(btree);
    assert(list->len == 4);
    assert(memcmp(list->entries[0].key.key, "entry_1", sizeof("entry_1")) == 0);
    assert(memcmp(list->entries[0].value.value, "value_1", sizeof("value_1")) == 0);
    assert(memcmp(list->entries[1].key.key, "entry_2", sizeof("entry_2")) == 0);
    assert(memcmp(list->entries[1].value.value, "value_2", sizeof("value_2")) == 0);
    assert(memcmp(list->entries[2].key.key, "entry_4", sizeof("entry_4")) == 0);
    assert(memcmp(list->entries[2].value.value, "value_4", sizeof("value_4")) == 0);
    assert(memcmp(list->entries[3].key.key, "entry_5", sizeof("entry_5")) == 0);
    assert(memcmp(list->entries[3].value.value, "value_5", sizeof("value_5")) == 0);
    free_tree(&btree);
    free_entry_list(&list);
}

void test_multiple_key_types() {
    btree_t* btree = new_btree();
    add_entry(btree, "entry_1", sizeof("entry_1"), "value_1", sizeof("value_1"));
    uint32_t int_key = 1;
    add_entry(btree, &int_key, sizeof(int_key), "value_2", sizeof("value_2"));
    uint64_t long_key = 10;
    add_entry(btree, &long_key, sizeof(long_key), "value_3", sizeof("value_3"));
    uint8_t byte_key = 9;
    add_entry(btree, &byte_key, sizeof(byte_key), "value_4", sizeof("value_4"));

    value_t* value = find_entry(btree, "entry_1", sizeof("entry_1"));
    assert(value != NULL);
    assert(memcmp(value->value, "value_1", sizeof("value_1")) == 0);

    value = find_entry(btree, &int_key, sizeof(int_key));
    assert(value != NULL);
    assert(memcmp(value->value, "value_2", sizeof("value_2")) == 0);

    value = find_entry(btree, &long_key, sizeof(long_key));
    assert(value != NULL);
    assert(memcmp(value->value, "value_3", sizeof("value_3")) == 0);

    value = find_entry(btree, &byte_key, sizeof(byte_key));
    assert(value != NULL);
    assert(memcmp(value->value, "value_4", sizeof("value_4")) == 0);

    free_tree(&btree);
}

void test_add_custom_struct() {
    btree_t* btree = new_btree();
    typedef struct custom_key {
        uint32_t key;
        uint32_t key2;
    }custom_key_t;

    custom_key_t key = {1, 2};
    add_entry(btree, &key, sizeof(key), "value_1", sizeof("value_1"));
    value_t* value = find_entry(btree, &key, sizeof(key));
    assert(value != NULL);
    assert(memcmp(value->value, "value_1", sizeof("value_1")) == 0);
    free_tree(&btree);
}

void test_add_custom_struct_to_value() {
    btree_t* btree = new_btree();
    typedef struct custom_value {
        uint32_t value;
        uint32_t value2;
    }custom_value_t;

    custom_value_t c_value = {1, 2};
    add_entry(btree, "key_1", sizeof("key_1"), &c_value, sizeof(c_value));
    value_t* value = find_entry(btree, "key_1", sizeof("key_1"));
    assert(value != NULL);
    assert(memcmp(value->value, &c_value, sizeof(c_value)) == 0);
    free_tree(&btree);
}

int main() {
    test_add_entry();
    test_entry_list();
    test_remove_entry();
    test_multiple_key_types();
    test_add_custom_struct();
    test_add_custom_struct_to_value();
    printf("All tests passed!\n");
    return 0;
}