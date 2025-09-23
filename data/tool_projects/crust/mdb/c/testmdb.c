#define VK_TERMINATE_ON_FAIL

#include "vktest.h"
#include "mdb.h"

void happy_test0() {
  VK_TEST_SECTION_BEGIN("misakawa happy test");

  mdb_options_t options;
  options.db_name = "misakawa";
  options.key_size_max = 64;
  options.data_size_max = 256;
  options.hash_buckets = 128;
  options.items_max = 166716;

  mdb_t db;
  mdb_status_t db_create_status = mdb_create(&db, options);
  VK_ASSERT_EQUALS(MDB_OK, db_create_status.code);

  mdb_status_t db_write_status = mdb_write(db, "misakawa", "mikoto");
  VK_ASSERT_EQUALS(MDB_OK, db_write_status.code);

  char buffer[257];
  mdb_status_t db_read_status = mdb_read(db, "misakawa", buffer, 257);
  VK_ASSERT_EQUALS(MDB_OK, db_read_status.code);

  fprintf(stderr, "\"misakawa\" => \"%s\"\n", buffer);

  mdb_status_t db_delete_status = mdb_delete(db, "misakawa");
  VK_ASSERT_EQUALS(MDB_OK, db_delete_status.code);

  db_read_status = mdb_read(db, "misakawa", buffer, 257);
  VK_ASSERT_EQUALS(MDB_NO_KEY, db_read_status.code);

  mdb_close(db);

  VK_TEST_SECTION_END("misakawa happy test");
}

void reopen_test1() {
  VK_TEST_SECTION_BEGIN("lambda re-open test");

  mdb_options_t options;
  options.db_name = "lambda";
  options.key_size_max = 64;
  options.data_size_max = 256;
  options.hash_buckets = 128;
  options.items_max = 166716;

  mdb_t db;
  (void)mdb_create(&db, options);
  (void)mdb_write(db, "Lisp", "LambdaExpression");

  mdb_close(db);

  mdb_t db1;
  mdb_status_t reopen_status = mdb_open(&db1, "lambda");

  mdb_options_t db1_options = mdb_get_options(db1);
  VK_ASSERT_EQUALS_S(options.db_name, db1_options.db_name);
  VK_ASSERT_EQUALS(options.key_size_max, db1_options.key_size_max);
  VK_ASSERT_EQUALS(options.data_size_max, db1_options.data_size_max);
  VK_ASSERT_EQUALS(options.hash_buckets, db1_options.hash_buckets);
  VK_ASSERT_EQUALS(options.items_max, db1_options.items_max);

  VK_ASSERT_EQUALS(MDB_OK, reopen_status.code);

  char buffer[257];
  mdb_status_t mdb_read_status = mdb_read(db1, "Lisp", buffer, 257);
  VK_ASSERT_EQUALS(MDB_OK, mdb_read_status.code);
  VK_ASSERT_EQUALS_S("LambdaExpression", buffer);

  mdb_close(db1);

  VK_TEST_SECTION_END("lambda re-open-test");
}

void load_test2() {
  VK_TEST_SECTION_BEGIN("accelerator load test");

  char *preset_values[18] = {
    "misakawa", "kamijou", "accelerator", "index", "lyzh", "chuigda",
    "de_nuke", "de_mirage", "de_cache", "de_vertigo", "de_inferno", "de_cbble",
    "de_dust2", "fy_iceworld", "cs_assault", "komakan", "scarlet", "flandre"
   };

  char **values = (char**)malloc(sizeof(char*) * 1000);
  char key[4] = { 0 };
  size_t i = 0;

  mdb_options_t options;
  options.db_name = "accelerator";
  options.key_size_max = 8;
  options.data_size_max = 256;
  options.hash_buckets = 128;
  options.items_max = 166716;

  mdb_t db;
  (void)mdb_create(&db, options);

  for (char c1 = '0'; c1 <= '9'; c1++) {
    for (char c2 = '0'; c2 <= '9'; c2++) {
      for (char c3 = '0'; c3 <= '9'; c3++) {
        key[0] = c1;
        key[1] = c2;
        key[2] = c3;
        values[i] = preset_values[rand() % 18];
        mdb_status_t write_status = mdb_write(db, key, values[i]);
        VK_ASSERT_EQUALS(MDB_OK, write_status.code);
        i++;
      }
    }
  }

  i = 0;
  char buffer[257];
  for (char c1 = '0'; c1 <= '9'; c1++) {
    for (char c2 = '0'; c2 <= '9'; c2++) {
      for (char c3 = '0'; c3 <= '9'; c3++) {
        key[0] = c1;
        key[1] = c2;
        key[2] = c3;
        mdb_status_t read_status = mdb_read(db, key, buffer, 257);
        VK_ASSERT_EQUALS(MDB_OK, read_status.code);
        VK_ASSERT_EQUALS_S(buffer, values[i]);
        i++;
      }
    }
  }

  free(values);
  mdb_close(db);

  VK_TEST_SECTION_END("accelerator load test");
}

void load_test3() {
  VK_TEST_SECTION_BEGIN("gabriel load test");

  char *preset_values[18] = {
    "misakawa", "kamijou", "accelerator", "index", "lyzh", "chuigda",
    "de_nuke", "de_mirage", "de_cache", "de_vertigo", "de_inferno", "de_cbble",
    "de_dust2", "fy_iceworld", "cs_assault", "komakan", "scarlet", "flandre"
   };

  char **values = (char**)malloc(sizeof(char*) * 1000);
  char **values2 = (char**)malloc(sizeof(char*) * 260);
  char key[4] = { 0 };
  size_t i = 0;

  mdb_options_t options;
  options.db_name = "gabriel";
  options.key_size_max = 8;
  options.data_size_max = 256;
  options.hash_buckets = 128;
  options.items_max = 166716;

  mdb_t db;
  (void)mdb_create(&db, options);

  for (char c1 = '0'; c1 <= '9'; c1++) {
    for (char c2 = '0'; c2 <= '9'; c2++) {
      for (char c3 = '0'; c3 <= '9'; c3++) {
        key[0] = c1;
        key[1] = c2;
        key[2] = c3;
        values[i] = preset_values[rand() % 18];
        (void)mdb_write(db, key, values[i]);
        i++;
      }
    }
  }

  size_t deleted_entries_cnt = 0;
  i = 0;
  for (char c1 = '0'; c1 <= '9'; c1++) {
    for (char c2 = '0'; c2 <= '9'; c2++) {
      for (char c3 = '0'; c3 <= '9'; c3++) {
        if (deleted_entries_cnt >= 128) {
          goto delete_done;
        }
        if (rand() % 10 == 0) {
          key[0] = c1;
          key[1] = c2;
          key[2] = c3;
          mdb_status_t delete_status = mdb_delete(db, key);
          VK_ASSERT_EQUALS(MDB_OK, delete_status.code);
          values[i] = NULL;
          deleted_entries_cnt++;
        }
        i++;
      }
    }
  }
delete_done:

  i = 0;
  key[2] = 0;
  for (char c1 = '0'; c1 <= '9'; c1++) {
    for (char c2 = 'A'; c2 <= 'Z'; c2++) {
      key[0] = c1;
      key[1] = c2;
      values2[i] = preset_values[rand() % 18];

      mdb_status_t write_status = mdb_write(db, key, values2[i]);
      VK_ASSERT_EQUALS(MDB_OK, write_status.code);
      i++;
    }
  }

  i = 0;
  char buffer[257];
  for (char c1 = '0'; c1 <= '9'; c1++) {
    for (char c2 = '0'; c2 <= '9'; c2++) {
      for (char c3 = '0'; c3 <= '9'; c3++) {
        key[0] = c1;
        key[1] = c2;
        key[2] = c3;
        mdb_status_t read_status = mdb_read(db, key, buffer, 257);
        if (values[i] == NULL) {
          VK_ASSERT_EQUALS(MDB_NO_KEY, read_status.code);
        } else {
          VK_ASSERT_EQUALS(MDB_OK, read_status.code);
          VK_ASSERT_EQUALS_S(values[i], buffer);
        }
        i++;
      }
    }
  }

  i = 0;
  key[2] = 0;
  for (char c1 = '0'; c1 <= '9'; c1++) {
    for (char c2 = 'A'; c2 <= 'Z'; c2++) {
      key[0] = c1;
      key[1] = c2;

      mdb_status_t read_status = mdb_read(db, key, buffer, 257);
      VK_ASSERT_EQUALS(MDB_OK, read_status.code);
      VK_ASSERT_EQUALS_S(values2[i], buffer);
      i++;
    }
  }

  free(values);
  free(values2);

  mdb_close(db);

  VK_TEST_SECTION_END("gabriel load test");
}

int main() {
  VK_TEST_BEGIN;

  happy_test0();
  reopen_test1();
  load_test2();
  load_test3();

  VK_TEST_END;
}
