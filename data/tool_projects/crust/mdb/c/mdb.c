#include "mdb.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>

#include <errno.h>

typedef uint32_t mdb_size_t;
typedef uint32_t mdb_ptr_t;

enum {
  MDB_PTR_SIZE = sizeof(mdb_ptr_t),
  MDB_DATALEN_SIZE = sizeof(mdb_size_t)
};

typedef struct {
  char *db_name;

  FILE *fp_superblock;
  FILE *fp_index;
  FILE *fp_data;

  mdb_options_t options;

  uint32_t index_record_size;
} mdb_int_t;

typedef struct {
  mdb_ptr_t next_ptr;
  mdb_ptr_t value_ptr;
  mdb_size_t value_size;
  char key[0];
} mdb_index_t;

static mdb_status_t mdb_status(uint8_t code, const char *desc);
static mdb_int_t *mdb_alloc(void);
static void mdb_free(mdb_int_t *db);
static uint32_t mdb_hash(const char *key);

static mdb_status_t mdb_read_bucket(mdb_int_t *db, uint32_t bucket,
                                    mdb_ptr_t *ptr);
static mdb_status_t mdb_read_index(mdb_int_t *db, mdb_ptr_t idxptr,
                                   mdb_index_t *index);
static mdb_status_t mdb_write_bucket(mdb_int_t *db, mdb_ptr_t bucket,
                                     mdb_ptr_t value);
static mdb_status_t mdb_write_index(mdb_int_t *db, mdb_ptr_t idxptr,
                                    const char *keybuf, mdb_ptr_t valptr,
                                    mdb_size_t valsize);
static mdb_status_t mdb_read_data(mdb_int_t *db, mdb_ptr_t valptr,
                                  mdb_size_t valsize, char *valbuf,
                                  mdb_size_t bufsiz);
static mdb_status_t mdb_write_data(mdb_int_t *db, mdb_ptr_t valptr,
                                   const char *valbuf, mdb_size_t valsize);
static mdb_status_t mdb_read_nextptr(mdb_int_t *db, mdb_ptr_t idxptr,
                                     mdb_ptr_t *nextptr);
static mdb_status_t mdb_write_nextptr(mdb_int_t *db, mdb_ptr_t ptr,
                                      mdb_ptr_t nextptr);
static mdb_status_t mdb_stretch_index_file(mdb_int_t *db, mdb_ptr_t *ptr);
static mdb_status_t mdb_index_alloc(mdb_int_t *db, mdb_ptr_t *ptr);
static mdb_status_t mdb_data_alloc(mdb_int_t *db, mdb_size_t valsize,
                                   mdb_ptr_t *ptr);
static mdb_status_t mdb_index_free(mdb_int_t *db, mdb_ptr_t ptr);
static mdb_status_t mdb_data_free(mdb_int_t *db, mdb_ptr_t valptr,
                                  mdb_size_t valsize);

static char pathbuf[4096];

mdb_status_t mdb_open(mdb_t *handle, const char *path) {
  mdb_int_t *db = mdb_alloc();
  if (db == NULL) {
    return mdb_status(MDB_ERR_ALLOC,
                      "failed allocating memory buffer for database");
  }

  strcpy(pathbuf, path);
  strcat(pathbuf, ".db.super");
  db->fp_superblock = fopen(pathbuf, "r");
  if (db->fp_superblock == NULL) {
    mdb_free(db);
    return mdb_status(MDB_ERR_OPEN_FILE, "cannot open superblock file as read");
  }

  fscanf(db->fp_superblock, "%s", db->db_name);
  fscanf(db->fp_superblock, "%hu", &(db->options.key_size_max));
  fscanf(db->fp_superblock, "%u", &(db->options.data_size_max));
  fscanf(db->fp_superblock, "%u", &(db->options.hash_buckets));
  fscanf(db->fp_superblock, "%u", &(db->options.items_max));

  db->index_record_size = db->options.key_size_max
                          + MDB_PTR_SIZE * 2
                          + MDB_DATALEN_SIZE;

  if (ferror(db->fp_superblock)) {
    mdb_free(db);
    return mdb_status(MDB_ERR_READ, "read error when parsing superblock");
  }

  strcpy(pathbuf, path);
  strcat(pathbuf, ".db.index");
  db->fp_index = fopen(pathbuf, "rb+");
  if (db->fp_index == NULL) {
    mdb_free(db);
    return mdb_status(MDB_ERR_OPEN_FILE, "cannot open index file as readwrite");
  }

  strcpy(pathbuf, path);
  strcat(pathbuf, ".db.data");
  db->fp_data = fopen(pathbuf, "rb+");
  if (db->fp_data == NULL) {
    mdb_free(db);
    return mdb_status(MDB_ERR_OPEN_FILE, "cannot open data file as readwrite");
  }

  if (fflush(NULL) != 0) {
    mdb_free(db);
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }

  *handle = (mdb_t)db;
  return mdb_status(MDB_OK, NULL);
}

mdb_status_t mdb_create(mdb_t *handle, mdb_options_t options) {
  mdb_int_t *db = mdb_alloc();
  if (db == NULL) {
    return mdb_status(MDB_ERR_ALLOC,
                      "failed allocating memory buffer for database");
  }

  strcpy(db->options.db_name, options.db_name);
  db->options.items_max = options.items_max;
  db->options.hash_buckets = options.hash_buckets;
  db->options.key_size_max = options.key_size_max;
  db->options.data_size_max = options.data_size_max;

  db->index_record_size = db->options.key_size_max
                          + MDB_PTR_SIZE * 2
                          + MDB_DATALEN_SIZE;

  strcpy(pathbuf, options.db_name);
  strcat(pathbuf, ".db.super");
  db->fp_superblock = fopen(pathbuf, "w");
  if (db->fp_superblock == NULL) {
    mdb_free(db);
    return mdb_status(MDB_ERR_OPEN_FILE,
                      "cannot open superblock file as write");
  }

  fprintf(db->fp_superblock, "%s\n", db->db_name);
  fprintf(db->fp_superblock, "%hu\n", db->options.key_size_max);
  fprintf(db->fp_superblock, "%u\n", db->options.data_size_max);
  fprintf(db->fp_superblock, "%u\n", db->options.hash_buckets);
  fprintf(db->fp_superblock, "%u\n", db->options.items_max);

  if (ferror(db->fp_superblock)) {
    mdb_free(db);
    return mdb_status(MDB_ERR_WRITE, "write error when writing superblock");
  }

  strcpy(pathbuf, options.db_name);
  strcat(pathbuf, ".db.index");
  db->fp_index = fopen(pathbuf, "wb+");
  if (db->fp_index == NULL) {
    mdb_free(db);
    return mdb_status(MDB_ERR_OPEN_FILE, "cannot open index file as readwrite");
  }

  mdb_ptr_t zero_ptr = 0;
  if (fwrite(&zero_ptr, MDB_PTR_SIZE, 1, db->fp_index) < 1) {
    mdb_free(db);
    return mdb_status(MDB_ERR_WRITE, "write error when writing freeptr");
  }
  for (size_t i = 0; i < options.hash_buckets; i++) {
    if (fwrite(&zero_ptr, MDB_PTR_SIZE, 1, db->fp_index) < 1) {
      mdb_free(db);
      return mdb_status(MDB_ERR_WRITE, "write error when writing hash buckets");
    }
  }

  strcpy(pathbuf, options.db_name);
  strcat(pathbuf, ".db.data");
  db->fp_data = fopen(pathbuf, "wb+");
  if (db->fp_data == NULL) {
    mdb_free(db);
    return mdb_status(MDB_ERR_OPEN_FILE, "cannot open data file as readwrite");
  }

  if (fflush(NULL) != 0) {
    mdb_free(db);
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }

  *handle = (mdb_t)db;
  return mdb_status(MDB_OK, NULL);
}

mdb_status_t mdb_read(mdb_t handle, const char *key, char *buf, size_t bufsiz) {
  mdb_int_t *db = (mdb_int_t*)handle;
  mdb_size_t bucket = mdb_hash(key) % db->options.hash_buckets;

  mdb_ptr_t ptr;
  mdb_status_t bucket_read_status = mdb_read_bucket(db, bucket, &ptr);
  STAT_CHECK_RET(bucket_read_status, {;});

  mdb_index_t *index =
      alloca(sizeof(mdb_index_t) + db->options.key_size_max + 1);

  while (ptr != 0) {
    mdb_status_t read_status = mdb_read_index(db, ptr, index);
    STAT_CHECK_RET(read_status, {;});
    if (strcmp(index->key, key) == 0) {
      return mdb_read_data(db, index->value_ptr, index->value_size, buf,
                           bufsiz);
    }
    ptr = index->next_ptr;
  }

  return mdb_status(MDB_NO_KEY, "Key not found");
}

mdb_status_t mdb_write(mdb_t handle, const char *key, const char *value) {
  mdb_int_t *db = (mdb_int_t*)handle;
  mdb_size_t bucket = mdb_hash(key) % db->options.hash_buckets;
  mdb_size_t key_size = strlen(key);
  if (key_size > db->options.key_size_max) {
    return mdb_status(MDB_ERR_KEY_SIZE, "key size too large");
  }
  mdb_size_t value_size = strlen(value);
  if (value_size > db->options.data_size_max) {
    return mdb_status(MDB_ERR_VALUE_SIZE, "value size too large");
  }

  mdb_ptr_t save_ptr = MDB_PTR_SIZE * (bucket + 1);
  mdb_ptr_t ptr;
  mdb_status_t bucket_read_status = mdb_read_bucket(db, bucket, &ptr);
  STAT_CHECK_RET(bucket_read_status, {;});

  mdb_index_t *index = alloca(sizeof(mdb_index_t)
                              + db->options.key_size_max + 1);
  while (ptr != 0) {
    mdb_status_t index_read_status = mdb_read_index(db, ptr, index);
    STAT_CHECK_RET(index_read_status, {;});
    if (strcmp(index->key, key) == 0) {
      break;
    }
    save_ptr = ptr;
    ptr = index->next_ptr;
  }

  if (ptr == 0) {
    mdb_ptr_t index_ptr;
    mdb_status_t index_alloc_status = mdb_index_alloc(db, &index_ptr);
    STAT_CHECK_RET(index_alloc_status, {;});
    mdb_ptr_t value_ptr;
    mdb_status_t data_alloc_status = mdb_data_alloc(db, value_size, &value_ptr);
    STAT_CHECK_RET(data_alloc_status, {
                     (void)mdb_index_free(db, index_ptr);
                   });
    mdb_status_t data_write_status = mdb_write_data(db, value_ptr, value,
                                                    value_size);
    STAT_CHECK_RET(data_write_status, {
                     (void)mdb_data_free(db, value_ptr, value_size);
                     (void)mdb_index_free(db, index_ptr);
                   });
    mdb_status_t index_write_status = mdb_write_index(db, index_ptr, key,
                                                      value_ptr, value_size);
    STAT_CHECK_RET(index_write_status, {
                     (void)mdb_data_free(db, value_ptr, value_size);
                     (void)mdb_index_free(db, index_ptr);
                   });
    mdb_status_t ptr_update_status = mdb_write_nextptr(db, save_ptr, index_ptr);
    STAT_CHECK_RET(ptr_update_status, {
                     (void)mdb_data_free(db, value_ptr, value_size);
                     (void)mdb_index_free(db, index_ptr);
                   });
    return mdb_status(MDB_OK, NULL);
  } else {
    /// @todo errors are only handled roughly here, needs refinement
    mdb_status_t data_free_status = mdb_data_free(db, index->value_ptr,
                                                  index->value_size);
    STAT_CHECK_RET(data_free_status, {;});
    mdb_ptr_t value_ptr;
    mdb_status_t data_alloc_status = mdb_data_alloc(db, value_size, &value_ptr);
    STAT_CHECK_RET(data_alloc_status, {;});
    mdb_status_t data_write_status = mdb_write_data(db, value_ptr, value,
                                                    value_size);
    STAT_CHECK_RET(data_write_status, {;});
    mdb_status_t index_write_status = mdb_write_index(db, ptr, key, value_ptr,
                                                      value_size);
    STAT_CHECK_RET(index_write_status, {;});
    return mdb_status(MDB_OK, NULL);
  }
}

mdb_status_t mdb_delete(mdb_t handle, const char *key) {
  mdb_int_t *db = (mdb_int_t*)handle;
  mdb_size_t bucket = mdb_hash(key) % db->options.hash_buckets;

  mdb_ptr_t save_ptr = MDB_PTR_SIZE * (bucket + 1);
  mdb_ptr_t ptr;
  mdb_status_t bucket_read_status = mdb_read_bucket(db, bucket, &ptr);
  STAT_CHECK_RET(bucket_read_status, {;});

  mdb_index_t *index = alloca(sizeof(mdb_index_t)
                              + db->options.key_size_max + 1);
  while (ptr != 0) {
    mdb_status_t index_read_status = mdb_read_index(db, ptr, index);
    STAT_CHECK_RET(index_read_status, {;});
    if (strcmp(index->key, key) == 0) {
      break;
    }
    save_ptr = ptr;
    ptr = index->next_ptr;
  }

  if (ptr == 0) {
    return mdb_status(MDB_NO_KEY, NULL);
  }

  mdb_status_t data_free_status = mdb_data_free(db, index->value_ptr,
                                                index->value_size);
  STAT_CHECK_RET(data_free_status, {;});
  mdb_status_t index_free_status = mdb_index_free(db, ptr);
  STAT_CHECK_RET(index_free_status, {;});
  mdb_status_t nextptr_update_status = mdb_write_nextptr(db, save_ptr,
                                                         index->next_ptr);
  STAT_CHECK_RET(nextptr_update_status, {;});

  return mdb_status(MDB_OK, NULL);
}

mdb_options_t mdb_get_options(mdb_t handle) {
  mdb_int_t *db = (mdb_int_t*)handle;
  return db->options;
}

size_t mdb_index_size(mdb_t *handle) {
  mdb_int_t *db = (mdb_int_t*)handle;
  (void)fseek(db->fp_index, 0, SEEK_END);
  return ftell(db->fp_index);
}

size_t mdb_data_size(mdb_t *handle) {
  mdb_int_t *db = (mdb_int_t*)handle;
  (void)fseek(db->fp_data, 0, SEEK_END);
  return ftell(db->fp_data);
}

static mdb_status_t mdb_read_bucket(mdb_int_t *db, uint32_t bucket,
                                    mdb_ptr_t *ptr) {
  if (fseek(db->fp_index, (long)(MDB_PTR_SIZE * (bucket + 1)), SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to bucket");
  }
  if (fread(ptr, MDB_PTR_SIZE, 1, db->fp_index) != 1) {
    return mdb_status(MDB_ERR_READ, "cannot read bucket");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_read_index(mdb_int_t *db, mdb_ptr_t idxptr,
                                   mdb_index_t *index) {
  if (fseek(db->fp_index, (long)idxptr, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to ptr");
  }
  if (fread(&(index->next_ptr), MDB_PTR_SIZE, 1, db->fp_index) != 1) {
    return mdb_status(MDB_ERR_READ, "cannot read next ptr");
  }
  if (fread(index->key, 1, db->options.key_size_max, db->fp_index)
      != db->options.key_size_max) {
    return mdb_status(MDB_ERR_READ, "cannot read key");
  }
  index->key[db->options.key_size_max] = '\0';
  if (fread(&(index->value_ptr), MDB_PTR_SIZE, 1, db->fp_index) != 1) {
    return mdb_status(MDB_ERR_READ, "cannot read value ptr");
  }
  if (fread(&(index->value_size), MDB_DATALEN_SIZE, 1, db->fp_index) != 1) {
    return mdb_status(MDB_ERR_READ, "cannot read value length");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_write_bucket(mdb_int_t *db, mdb_ptr_t bucket,
                                     mdb_ptr_t value) {
  if (fseek(db->fp_index, (long)(MDB_PTR_SIZE * (bucket + 1)), SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to bucket");
  }
  if (fwrite(&value, MDB_PTR_SIZE, 1, db->fp_index) != 1) {
    return mdb_status(MDB_ERR_WRITE, "cannot write bucket");
  }
  if (fflush(db->fp_index) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_write_index(mdb_int_t *db, mdb_ptr_t idxptr,
                                    const char *keybuf, mdb_ptr_t valptr,
                                    mdb_size_t valsize) {
  if (fseek(db->fp_index, (long)(idxptr + MDB_PTR_SIZE), SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to ptr");
  }
  size_t key_len = strlen(keybuf);
  if (fwrite(keybuf, 1, key_len, db->fp_index) < key_len) {
    return mdb_status(MDB_ERR_WRITE, "cannot write key part of index");
  }
  size_t value_ptr_pos = idxptr + MDB_PTR_SIZE + db->options.key_size_max;
  if (fseek(db->fp_index, (long)value_ptr_pos, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to value part of index");
  }
  if (fwrite(&valptr, MDB_PTR_SIZE, 1, db->fp_index) < 1) {
    return mdb_status(MDB_ERR_WRITE, "cannot write to value part of index");
  }
  if (fwrite(&valsize, MDB_DATALEN_SIZE, 1, db->fp_index) < 1) {
    return mdb_status(MDB_ERR_WRITE, "cannot write to value part of index");
  }
  if (fflush(db->fp_index) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_read_nextptr(mdb_int_t *db, mdb_ptr_t idxptr,
                                     mdb_ptr_t *nextptr) {
  if (fseek(db->fp_index, (long)idxptr, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to ptr");
  }
  if (fread(nextptr, MDB_PTR_SIZE, 1, db->fp_index) != 1) {
    return mdb_status(MDB_ERR_READ, "cannot read next ptr");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_read_data(mdb_int_t *db, mdb_ptr_t valptr,
                                  mdb_size_t valsize, char *valbuf,
                                  mdb_size_t bufsiz) {
  if (bufsiz < valsize + 1) {
    return mdb_status(MDB_ERR_BUFSIZ, "value buffer size too small");
  }
  if (fseek(db->fp_data, (long)valptr, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to value");
  }
  if (fread(valbuf, 1, valsize, db->fp_data) != valsize) {
    return mdb_status(MDB_ERR_READ, "cannot read data");
  }
  valbuf[valsize] = '\0';
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_write_nextptr(mdb_int_t *db, mdb_ptr_t ptr,
                                      mdb_ptr_t nextptr) {
  if (fseek(db->fp_index, (long)ptr, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to head of index file");
  }
  if (fwrite(&nextptr, MDB_PTR_SIZE, 1, db->fp_index) < 1) {
    return mdb_status(MDB_ERR_WRITE, "cannot write to head of index file");
  }
  if (fflush(db->fp_index) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_write_data(mdb_int_t *db, mdb_ptr_t valptr,
                                   const char *valbuf, mdb_size_t valsize) {
  if (fseek(db->fp_data, (long)valptr, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to value");
  }
  if (fwrite(valbuf, 1, valsize, db->fp_data) < valsize) {
    return mdb_status(MDB_ERR_WRITE, "cannot write data");
  }
  if (fflush(db->fp_data) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_stretch_index_file(mdb_int_t *db, mdb_ptr_t *ptr) {
  if (fseek(db->fp_index, 0, SEEK_END) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to end of index file");
  }
  *ptr = (mdb_ptr_t)ftell(db->fp_index);

  unsigned char zero = '\0';
  for (size_t i = 0; i < db->index_record_size; i++) {
    if (fwrite(&zero, 1, 1, db->fp_index) < 1) {
      return mdb_status(MDB_ERR_WRITE, "cannot stretch index file");
    }
  }
  if (fflush(db->fp_index) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_index_alloc(mdb_int_t *db, mdb_ptr_t *ptr) {
  mdb_ptr_t freeptr;
  mdb_status_t freeptr_read_status = mdb_read_nextptr(db, 0, &freeptr);
  STAT_CHECK_RET(freeptr_read_status, {;});

  if (freeptr != 0) {
    mdb_ptr_t new_freeptr;
    mdb_status_t nextptr_read_stat =
        mdb_read_nextptr(db, freeptr, &new_freeptr);
    STAT_CHECK_RET(nextptr_read_stat, {;});
    mdb_status_t freeptr_update_stat = mdb_write_nextptr(db, 0, new_freeptr);
    STAT_CHECK_RET(freeptr_update_stat, {;});
    mdb_status_t freeptr_cleanup_stat = mdb_write_nextptr(db, freeptr, 0);
    STAT_CHECK_RET(freeptr_cleanup_stat, {;});
    *ptr = freeptr;
    return mdb_status(MDB_OK, NULL);
  } else {
    return mdb_stretch_index_file(db, ptr);
  }
}

static mdb_status_t mdb_data_alloc(mdb_int_t *db, mdb_size_t valsize,
                                   mdb_ptr_t *ptr) {
  if (fseek(db->fp_data, 0, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to head of data file");
  }
  while (!feof(db->fp_data)) {
    uint8_t byte;
    if (fread(&byte, 1, 1, db->fp_data) != 1 && !feof(db->fp_data)) {
      return mdb_status(MDB_ERR_READ, "cannot read data file");
    }
    while (!feof(db->fp_data) && byte != '\0') {
      if (fread(&byte, 1, 1, db->fp_data) != 1 && !feof(db->fp_data)) {
        return mdb_status(MDB_ERR_READ, "cannot read data file");
      }
    }
    
    mdb_ptr_t start_ptr = (mdb_ptr_t)ftell(db->fp_data);
    while (!feof(db->fp_data) && byte == '\0') {
      if (fread(&byte, 1, 1, db->fp_data) != 1 && !feof(db->fp_data)) {
        return mdb_status(MDB_ERR_READ, "cannot read data file");
      }
    }
    mdb_ptr_t end_ptr = (mdb_ptr_t)ftell(db->fp_data);

    /// @todo currently we rely on the extra padding to keep correct results.
    /// refactor and remove this padding sometime.
    if (end_ptr - start_ptr >= valsize + 2) {
      *ptr = start_ptr + 1;
      return mdb_status(MDB_OK, NULL);
    }
  }

  mdb_ptr_t end_ptr = (mdb_ptr_t)ftell(db->fp_data);
  unsigned char zero = '\0';
  for (size_t i = 0; i < valsize; i++) {
    if (fwrite(&zero, 1, 1, db->fp_data) < 1) {
      return mdb_status(MDB_ERR_WRITE, "cannot stretch data file");
    }
  }
  if (fflush(db->fp_data) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }
  *ptr = end_ptr;
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_index_free(mdb_int_t *db, mdb_ptr_t ptr) {
  if (fseek(db->fp_index, 0, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to head of index file");
  }
  mdb_t freeptr;
  if (fread(&freeptr, MDB_PTR_SIZE, 1, db->fp_index) != 1) {
    return mdb_status(MDB_ERR_READ, "cannot read free ptr");
  }

  if (fseek(db->fp_index, 0, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to head of index file");
  }
  if (fwrite(&ptr, MDB_PTR_SIZE, 1, db->fp_index) < 1) {
    return mdb_status(MDB_ERR_WRITE, "cannot write to head of index file");
  }

  if (fseek(db->fp_index, (long)ptr, SEEK_SET) != 0) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to ptr");
  }

  if (fwrite(&freeptr, MDB_PTR_SIZE, 1, db->fp_index) < 1) {
    return mdb_status(MDB_ERR_WRITE, "cannot write to ptr");
  }

  for (size_t i = 0; i < db->options.key_size_max; i++) {
    char zero = '\0';
    if (fwrite(&zero, 1, 1, db->fp_index) < 1) {
      return mdb_status(MDB_ERR_WRITE, "cannot clean key part of index");
    }
  }

  if (fflush(db->fp_index) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }

  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_data_free(mdb_int_t *db, mdb_ptr_t valptr,
                                  mdb_size_t valsize) {
  if (fseek(db->fp_data, (long)valptr, SEEK_SET)) {
    return mdb_status(MDB_ERR_SEEK, "cannot seek to data record");
  }
  uint8_t zero = '\0';
  for (size_t i = 0; i < valsize; i++) {
    if (fwrite(&zero, 1, 1, db->fp_data) != 1) {
      return mdb_status(MDB_ERR_WRITE, "cannot write empty data");
    }
  }
  if (fflush(db->fp_data) != 0) {
    return mdb_status(MDB_ERR_FLUSH, "fflush failed");
  }
  return mdb_status(MDB_OK, NULL);
}

static mdb_status_t mdb_status(uint8_t code, const char *desc) {
  mdb_status_t s;
  s.code = code;
  s.desc = desc;
  return s;
}

static mdb_int_t *mdb_alloc() {
  mdb_int_t *ret = (mdb_int_t*)malloc(sizeof(mdb_int_t));
  if (!ret) {
    return NULL;
  }
  ret->db_name = (char*)malloc(DB_NAME_MAX + 1);
  if (!ret->db_name) {
    free(ret);
    return NULL;
  }
  ret->options.db_name = ret->db_name;
  return ret;
}

static void mdb_free(mdb_int_t *db) {
  if (db->fp_superblock != NULL) {
    fclose(db->fp_superblock);
  }
  if (db->fp_index != NULL) {
    fclose(db->fp_index);
  }
  if (db->fp_data != NULL) {
    fclose(db->fp_data);
  }
  free(db->db_name);
  free(db);
}

static uint32_t mdb_hash(const char *key) {
  uint32_t ret = 0;
  for (uint32_t i = 0; *key != '\0'; ++key, ++i) {
    ret += (uint32_t)*key * i;
  }
  return ret;
}

void mdb_close(mdb_t handle) {
  mdb_int_t *db = (mdb_int_t*)handle;
  fclose(db->fp_superblock);
  fclose(db->fp_index);
  fclose(db->fp_data);
  free(db->db_name);
  free(db);
}
