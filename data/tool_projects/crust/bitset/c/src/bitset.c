/*
 * Copyright (c) 2020 Adil Benhlal <adil.benhlal@outlook.fr>
 *
 * bitset is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See LICENSE for details.
 */
#include <errno.h>
#include "./include/cborg/bitset.h"

bitset_t *cb_bitset_new(size_t nb_bits) {
  bitset_t *bs = (bitset_t *)calloc(1, sizeof(bitset_t));
  bs->array = (uint8_t *)calloc(__bitset_size(nb_bits), sizeof(uint8_t));
  bs->size = nb_bits;
  return bs;
}

void cb_bitset_delete(bitset_t *bs) {
  free(bs->array);
  free(bs);
}

size_t cb_bitset_count(bitset_t *bs) {
  return (bs->flip ? (bs->size - bs->count) : bs->count);
}

size_t cb_bitset_size(bitset_t *bs) { return bs->size; }

bool cb_bitset_test(bitset_t *bs, size_t idx) {
  return bs->flip ^
         ((bs->array[__bitset_byte(idx)] & __bitset_bit_mask(idx)) != 0);
}

bool cb_bitset_any(bitset_t *bs) {
  return (bs->flip ? (bs->count != bs->size) : (bs->count != 0));
}

bool cb_bitset_none(bitset_t *bs) { return !(cb_bitset_any(bs)); }

bool cb_bitset_all(bitset_t *bs) {
  return (bs->flip ? (bs->count == 0) : (bs->count == bs->size));
}

void cb_bitset_set(bitset_t *bs, size_t idx, bool value) {
  bool flip_value = bs->flip ? !value : value;
  size_t previous_count = __builtin_popcount(bs->array[__bitset_byte(idx)]);
  if (flip_value) {
    bs->array[__bitset_byte(idx)] |= __bitset_bit_mask(idx);
  } else {
    bs->array[__bitset_byte(idx)] &= ~__bitset_bit_mask(idx);
  }
  bs->count +=
      (__builtin_popcount(bs->array[__bitset_byte(idx)]) - previous_count);
}

void cb_bitset_reset(bitset_t *bs) {
  memset(bs->array, 0, __bitset_size(bs->size));
  bs->flip = false;
  bs->count = 0;
}

void cb_bitset_flip(bitset_t *bs) { bs->flip = !(bs->flip); }

char *cb_bitset_to_string(bitset_t *bs) {
  char *res = (char *)malloc(sizeof(char) * (bs->size + 1));
  for (size_t i = 0; i < bs->size; i++) {
    res[i] = cb_bitset_test(bs, i) ? '1' : '0';
  }
  res[bs->size] = '\0';
  return res;
}

void cb_bitset_write(bitset_t *bs, const char *path) {
  FILE *file = fopen(path, "wb");
  size_t bytes_to_write = __bitset_size(bs->size);
  size_t bytes_writen = 0;
  size_t total_writen = 0;

  if (file == NULL) {
    fprintf(stdout, "%s\n", strerror(errno));
    exit(EXIT_FAILURE);
  }

  fwrite(&(bs->size), sizeof(size_t), 1, file);
  fwrite(&(bs->count), sizeof(size_t), 1, file);
  fwrite(&(bs->flip), sizeof(bool), 1, file);
  while ((bytes_writen = fwrite(bs->array + total_writen, sizeof(uint8_t),
                                bytes_to_write - total_writen, file)) > 0) {
    total_writen += bytes_writen;
  }

  fclose(file);
}

bitset_t *cb_bitset_read(const char *path) {
  FILE *file = fopen(path, "rb");
  size_t size = 0;

  if (file == NULL) {
    fprintf(stdout, "%s\n", strerror(errno));
    exit(EXIT_FAILURE);
  }

  fread(&size, sizeof(size_t), 1, file);
  bitset_t *bs = cb_bitset_new(size);
  fread(&(bs->count), sizeof(size_t), 1, file);
  fread(&(bs->flip), sizeof(bool), 1, file);
  while (fread(bs->array, sizeof(uint8_t), bs->size, file) > 0);

  fclose(file);
  return bs;
}
