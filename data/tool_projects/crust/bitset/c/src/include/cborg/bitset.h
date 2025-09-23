/*
 * Copyright (c) 2020 Adil Benhlal <adil.benhlal@outlook.fr>
 *
 * bitset is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See LICENSE for details.
 */

#ifndef _CBORG_BITSET_H
#define _CBORG_BITSET_H

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// Returns the number of bytes needed to store a number of bits
#define __bitset_size(nbits) ((nbits + 7) >> 3)

// Returns the index of the bit in bitset->array (=idx/8)
#define __bitset_byte(idx) ((idx) >> 3)

// Returns the bit mask
#define __bitset_bit_mask(idx) (1 << ((idx)&0x7))

// TODO: format bitset using cbor
// TODO: bitset thread-safe

typedef struct bitset_t {
  bool flip;
  size_t size;
  size_t count;
  uint8_t *array;
} bitset_t;

bitset_t *cb_bitset_new(size_t nb_bits);

void cb_bitset_delete(bitset_t *bs);

size_t cb_bitset_count(bitset_t *bs);

size_t cb_bitset_size(bitset_t *bs);

bool cb_bitset_test(bitset_t *bs, size_t idx);

bool cb_bitset_any(bitset_t *bs);

bool cb_bitset_none(bitset_t *bs);

bool cb_bitset_all(bitset_t *bs);

void cb_bitset_set(bitset_t *bs, size_t idx, bool value);

void cb_bitset_reset(bitset_t *bs);

void cb_bitset_flip(bitset_t *bs);

char *cb_bitset_to_string(bitset_t *bs);

void cb_bitset_write(bitset_t *bs, const char *path);

bitset_t *cb_bitset_read(const char *path);

#endif
