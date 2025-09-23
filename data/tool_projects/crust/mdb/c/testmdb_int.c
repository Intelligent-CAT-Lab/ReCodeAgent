#define VK_TERMINATE_ON_FAIL

#include "mdb.c"
#include "vktest.h"

#include <time.h>
#include <stdbool.h>

#define TESTDB_DB_NAME "testdb"
#define TESTDB_KEY_SIZE_MAX 8
#define TESTDB_DATA_SIZE_MAX 1024
#define TESTDB_HASH_BUCKETS 512
#define TESTDB_ITEMS_MAX 65536

mdb_options_t get_default_options() {
  mdb_options_t ret;
  ret.db_name = TESTDB_DB_NAME;
  ret.key_size_max = TESTDB_KEY_SIZE_MAX;
  ret.data_size_max = TESTDB_DATA_SIZE_MAX;
  ret.hash_buckets = TESTDB_HASH_BUCKETS;
  ret.items_max = TESTDB_ITEMS_MAX;
  return ret;
}

mdb_options_t get_options_no_hash_buckets() {
  mdb_options_t ret = get_default_options();
  ret.hash_buckets = 0;
  return ret;
}

mdb_int_t open_test_db(mdb_options_t options) {
  mdb_int_t ret;
  ret.options = options;
  ret.fp_superblock = fopen("super", "wb+");
  ret.fp_index = fopen("index", "wb+");
  mdb_ptr_t free_ptr = 0;
  fwrite(&free_ptr, MDB_PTR_SIZE, 1, ret.fp_index);
  ret.fp_data = fopen("data", "wb+");

  ret.index_record_size = ret.options.key_size_max
                          + MDB_PTR_SIZE * 2
                          + MDB_DATALEN_SIZE;
  return ret;
}

void close_test_db(mdb_int_t db) {
  fclose(db.fp_superblock);
  fclose(db.fp_index);
  fclose(db.fp_data);
}

void generate_random_key(char *buffer) {
  for (size_t i = 0; i < TESTDB_KEY_SIZE_MAX - 1; i++) {
    buffer[i] = rand() % 128;
  }
  buffer[TESTDB_KEY_SIZE_MAX - 1] = 0;
}

void generate_seq_key(char *buffer, size_t seq) {
  memset(buffer, seq, TESTDB_KEY_SIZE_MAX - 1);
  buffer[TESTDB_KEY_SIZE_MAX] = 0;
}

bool check_seq(char *buffer, char value, size_t len) {
  for (size_t i = 0; i < len; i++) {
    if (buffer[i] != value) {
      return false;
    }
  }
  return true;
}

void test1() {
  VK_TEST_SECTION_BEGIN("simple index write");

  mdb_int_t testdb = open_test_db(get_options_no_hash_buckets());
  mdb_ptr_t idxptr_arr[32];
  static char keys[32][TESTDB_KEY_SIZE_MAX];
  mdb_ptr_t valptr_arr[32];
  mdb_size_t valsize_arr[32];

  for (size_t i = 0; i < 32; i++) {
    generate_seq_key(keys[i], i);
    valptr_arr[i] = rand();
    valsize_arr[i] = rand();
    mdb_status_t index_alloc_status = mdb_index_alloc(&testdb, idxptr_arr + i);
    mdb_status_t index_write_status = mdb_write_index(&testdb, idxptr_arr[i],
                                                      keys[i], valptr_arr[i],
                                                      valsize_arr[i]);
    VK_ASSERT_EQUALS(MDB_OK, index_alloc_status.code);
    VK_ASSERT_EQUALS(MDB_OK, index_write_status.code);
  }

  for (size_t i = 0; i < 32; i++) {
    mdb_index_t *test_index =
        alloca(sizeof(mdb_index_t) + TESTDB_KEY_SIZE_MAX + 1);
    mdb_status_t index_read_status = mdb_read_index(&testdb, idxptr_arr[i],
                                                    test_index);
    VK_ASSERT_EQUALS(MDB_OK, index_read_status.code);

    VK_ASSERT_EQUALS_S(keys[i], test_index->key);
    VK_ASSERT_EQUALS(valptr_arr[i], test_index->value_ptr);
    VK_ASSERT_EQUALS(valsize_arr[i], test_index->value_size);
  }

  VK_TEST_SECTION_END("simple index write");
}

void test2() {
  VK_TEST_SECTION_BEGIN("simple data write");

  mdb_int_t testdb = open_test_db(get_options_no_hash_buckets());

  mdb_ptr_t valptr_arr[32];
  mdb_size_t valsize_arr[32];

  char buffer[64 + 32 + 1];
  for (size_t i = 0; i < 32; i++) {
    memset(buffer, i + 1, 64 + 32);
    valsize_arr[i] = rand() % 64 + 32;
    mdb_status_t alloc_status = mdb_data_alloc(&testdb, valsize_arr[i],
                                               valptr_arr + i);
    VK_ASSERT_EQUALS(MDB_OK, alloc_status.code);
    mdb_status_t write_status = mdb_write_data(&testdb, valptr_arr[i],
                                               buffer, valsize_arr[i]);
    VK_ASSERT_EQUALS(MDB_OK, write_status.code);
  }

  for (size_t i = 0; i < 32; i++) {
    mdb_status_t read_status = mdb_read_data(&testdb, valptr_arr[i],
                                             valsize_arr[i], buffer,
                                             64 + 32 + 1);
    VK_ASSERT_EQUALS(MDB_OK, read_status.code);
    VK_ASSERT(check_seq(buffer, i + 1, valsize_arr[i]));
  }

  close_test_db(testdb);

  VK_TEST_SECTION_END("simple data write");
}

void test3() {
  VK_TEST_SECTION_BEGIN("index reuse");

  mdb_int_t testdb = open_test_db(get_options_no_hash_buckets());
  mdb_ptr_t idxptr_arr[32];
  static char keys[32][TESTDB_KEY_SIZE_MAX];
  mdb_ptr_t valptr_arr[32];
  mdb_size_t valsize_arr[32];

  for (size_t i = 0; i < 32; i++) {
    generate_seq_key(keys[i], i);
    valptr_arr[i] = rand();
    valsize_arr[i] = rand();
    (void)mdb_index_alloc(&testdb, idxptr_arr + i);
    (void)mdb_write_index(&testdb, idxptr_arr[i], keys[i], valptr_arr[i],
                          valsize_arr[i]);
  }

  for (size_t i = 0; i < 4; i++) {
    size_t to_remove = rand() % 32;
    while (idxptr_arr[to_remove] == 0) {
      to_remove = rand() % 32;
    }
    mdb_status_t free_status = mdb_index_free(&testdb, idxptr_arr[to_remove]);
    VK_ASSERT_EQUALS(MDB_OK, free_status.code);
    idxptr_arr[to_remove] = 0;
  }

  mdb_ptr_t new_idxptr_arr[8];
  static char new_keys[8][TESTDB_KEY_SIZE_MAX];
  mdb_ptr_t new_valptr_arr[8];
  mdb_size_t new_valsize_arr[8];

  for (size_t i = 0; i < 8; i++) {
    generate_seq_key(new_keys[i], i + 40);
    new_valptr_arr[i] = rand();
    new_valsize_arr[i] = rand();
    mdb_status_t alloc_status = mdb_index_alloc(&testdb, new_idxptr_arr + i);
    VK_ASSERT_EQUALS(MDB_OK, alloc_status.code);
    mdb_status_t write_status = mdb_write_index(&testdb, new_idxptr_arr[i],
                                                new_keys[i], new_valptr_arr[i],
                                                new_valsize_arr[i]);
    VK_ASSERT_EQUALS(MDB_OK, write_status.code);
  }

  for (size_t i = 0; i < 32; i++) {
    if (idxptr_arr[i] == 0) {
      continue;
    }

    mdb_index_t *test_index =
        alloca(sizeof(mdb_index_t) + TESTDB_KEY_SIZE_MAX + 1);
    mdb_status_t index_read_status = mdb_read_index(&testdb, idxptr_arr[i],
                                                    test_index);

    VK_ASSERT_EQUALS(MDB_OK, index_read_status.code);

    VK_ASSERT_EQUALS_S(keys[i], test_index->key);
    VK_ASSERT_EQUALS(valptr_arr[i], test_index->value_ptr);
    VK_ASSERT_EQUALS(valsize_arr[i], test_index->value_size);
  }

  for (size_t i = 0; i < 8; i++) {
    mdb_index_t *test_index =
        alloca(sizeof(mdb_index_t) + TESTDB_KEY_SIZE_MAX + 1);
    mdb_status_t index_read_status = mdb_read_index(&testdb, new_idxptr_arr[i],
                                                    test_index);
    VK_ASSERT_EQUALS(MDB_OK, index_read_status.code);

    VK_ASSERT_EQUALS_S(new_keys[i], test_index->key);
    VK_ASSERT_EQUALS(new_valptr_arr[i], test_index->value_ptr);
    VK_ASSERT_EQUALS(new_valsize_arr[i], test_index->value_size);
  }

  fprintf(stderr, "file size = %lu\n", mdb_index_size((mdb_t*)&testdb));
  fprintf(stderr, "MDB_PTR_SIZE = %u\n", MDB_PTR_SIZE);
  fprintf(stderr, "index record size size = %u\n", testdb.index_record_size);
  VK_ASSERT_EQUALS(MDB_PTR_SIZE + testdb.index_record_size * (32 - 4 + 8),
                   mdb_index_size((mdb_t*)&testdb));

  close_test_db(testdb);

  VK_TEST_SECTION_END("index reuse");
}

void test4() {
  VK_TEST_SECTION_BEGIN("data reuse");

  mdb_int_t testdb = open_test_db(get_options_no_hash_buckets());

  mdb_ptr_t valptr_arr[32];
  mdb_size_t valsize_arr[32];

  char buffer[64 + 32 + 1] = {0};
  for (size_t i = 0; i < 32; i++) {
    memset(buffer, i + 1, 64 + 32);
    valsize_arr[i] = rand() % 64 + 32;
    (void)mdb_data_alloc(&testdb, valsize_arr[i], valptr_arr + i);
    (void)mdb_write_data(&testdb, valptr_arr[i], buffer, valsize_arr[i]);
  }

  for (size_t i = 0; i < 8; i++) {
    size_t to_remove = rand() % 32;
    while (valptr_arr[to_remove] == 0) {
      to_remove = rand() % 32;
    }
    mdb_status_t free_status = mdb_data_free(&testdb, valptr_arr[to_remove],
                                             valsize_arr[to_remove]);
    valptr_arr[to_remove] = 0;
    VK_ASSERT_EQUALS(MDB_OK, free_status.code);
  }

  mdb_ptr_t new_valptr_arr[8];
  mdb_size_t new_valsize_arr[8];

  for (size_t i = 0; i < 8; i++) {
    memset(buffer, i + 44, 64 + 32);
    new_valsize_arr[i] = rand() % 64 + 32;
    mdb_status_t alloc_status = mdb_data_alloc(&testdb, new_valsize_arr[i],
                                               new_valptr_arr + i);
    VK_ASSERT_EQUALS(MDB_OK, alloc_status.code);
    mdb_status_t write_status = mdb_write_data(&testdb, new_valptr_arr[i],
                                               buffer, new_valsize_arr[i]);

    VK_ASSERT_EQUALS(MDB_OK, write_status.code);
  }

  for (size_t i = 0; i < 32; i++) {
    if (valptr_arr[i] == 0) {
      continue;
    }
    mdb_status_t read_status = mdb_read_data(&testdb, valptr_arr[i],
                                             valsize_arr[i], buffer,
                                             64 + 32 + 1);
    VK_ASSERT_EQUALS(MDB_OK, read_status.code);
    VK_ASSERT(check_seq(buffer, i + 1, valsize_arr[i]));
  }

  for (size_t i = 0; i < 8; i++) {
    mdb_status_t read_status = mdb_read_data(&testdb, new_valptr_arr[i],
                                             new_valsize_arr[i], buffer,
                                             64 + 32 + 1);
    VK_ASSERT_EQUALS(MDB_OK, read_status.code);
    VK_ASSERT(check_seq(buffer, i + 44, new_valsize_arr[i]));
  }

  close_test_db(testdb);

  VK_TEST_SECTION_END("data reuse");
}

int main() {
  srand(time(NULL));

  VK_TEST_BEGIN;

  test1();
  test2();
  test3();
  for (size_t i = 0; i < 8; i++) {
    test4();
  }

  VK_TEST_END;

  return 0;
}
