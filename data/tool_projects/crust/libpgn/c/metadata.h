/* libpgn - PGN file parser.
 * Copyright (C) 2024 youkwhd <lolywk@tutanota.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#ifndef __LIBPGN_METADATA_H
#define __LIBPGN_METADATA_H

#include <stddef.h>

#include "piece.h"
#include "utils/export.h"
#include "utils/buffer.h"

#define PGN_METADATA_INITIAL_SIZE 8
#define PGN_METADATA_GROW_SIZE 8

typedef struct __pgn_metadata_item_t {
    pgn_buffer_t *key;
    pgn_buffer_t *value;
} __pgn_metadata_item_t;

typedef struct pgn_metadata_t {
    __pgn_metadata_item_t *items;
    size_t length, size;
} pgn_metadata_t;

void __pgn_metadata_item_init(__pgn_metadata_item_t *item);
void __pgn_metadata_item_cleanup(__pgn_metadata_item_t *item);

PGN_EXPORT pgn_metadata_t *pgn_metadata_init(void);
PGN_EXPORT pgn_metadata_t *__pgn_metadata_from_string(char *str, size_t *consumed);
PGN_EXPORT pgn_metadata_t *pgn_metadata_from_string(char *str);
PGN_EXPORT void pgn_metadata_insert(pgn_metadata_t *metadata, char *key, char *value);
PGN_EXPORT char *pgn_metadata_get(pgn_metadata_t *metadata, char *key);
PGN_EXPORT void pgn_metadata_delete(pgn_metadata_t *metadata, char *key);
PGN_EXPORT void pgn_metadata_cleanup(pgn_metadata_t *metadata);

#endif // __LIBPGN_METADATA_H
