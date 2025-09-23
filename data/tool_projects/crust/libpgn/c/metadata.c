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

#include <stdlib.h>
#include <assert.h>

#include "metadata.h"
#include "utils/cursor.h"

pgn_metadata_t *__pgn_metadata_from_string(char *str, size_t *consumed)
{
    size_t cursor = 0;

    /* Redundant check, but it works to not make unnecessary
     * memory allocations (key_buf, value_buf)
     */
    if (str[cursor] != '[') return NULL;

    pgn_metadata_t *metadata = pgn_metadata_init();
    pgn_buffer_t *key_buffer = pgn_buffer_init();
    pgn_buffer_t *value_buffer = pgn_buffer_init();

    for (;;) {
        if (str[cursor] != '[') break;
        cursor++;

        while (str[cursor] != ' ') pgn_buffer_append(key_buffer, str[cursor++]);
        pgn_buffer_append_null_terminator(key_buffer);

        assert(str[++cursor] == '"');
        cursor++;

        while (str[cursor] != '"') pgn_buffer_append(value_buffer, str[cursor++]);
        pgn_buffer_append_null_terminator(value_buffer);

        pgn_metadata_insert(metadata, key_buffer->buf, value_buffer->buf);
        pgn_buffer_reset(key_buffer);
        pgn_buffer_reset(value_buffer);

        assert(str[++cursor] == ']');
        cursor++;

        pgn_cursor_skip_newline(str, &cursor);
    }

    *consumed += cursor;
    pgn_buffer_cleanup(key_buffer);
    pgn_buffer_cleanup(value_buffer);
    return metadata;
}

pgn_metadata_t *pgn_metadata_from_string(char *str)
{
    size_t consumed = 0;
    return __pgn_metadata_from_string(str, &consumed);
}

void __pgn_metadata_item_init(__pgn_metadata_item_t *item)
{
    item->key = pgn_buffer_init();
    item->value = pgn_buffer_init();
}

void __pgn_metadata_item_cleanup(__pgn_metadata_item_t *item)
{
    pgn_buffer_cleanup(item->key);
    pgn_buffer_cleanup(item->value);
}

pgn_metadata_t *pgn_metadata_init(void)
{
    pgn_metadata_t *metadata = malloc(sizeof *metadata);
    metadata->items = malloc((sizeof *metadata->items) * PGN_METADATA_INITIAL_SIZE);
    metadata->size = PGN_METADATA_INITIAL_SIZE;
    metadata->length = 0;
    return metadata;
}

void pgn_metadata_insert(pgn_metadata_t *metadata, char *key, char *value)
{
    if (metadata->length >= metadata->size) {
        metadata->size += PGN_METADATA_GROW_SIZE;
        metadata->items = realloc(metadata->items, (sizeof *metadata->items) * metadata->size);
    }

    __pgn_metadata_item_t item = {0};
    __pgn_metadata_item_init(&item);

    pgn_buffer_concat(item.key, key);
    pgn_buffer_concat(item.value, value);

    metadata->items[metadata->length++] = item;
}

char *pgn_metadata_get(pgn_metadata_t *metadata, char *key)
{
    for (size_t i = 0; i < metadata->length; i++) {
        if (pgn_buffer_equal(metadata->items[i].key, key)) {
            return metadata->items[i].value->buf;
        }
    }

    return NULL;
}

void pgn_metadata_delete(pgn_metadata_t *metadata, char *key)
{
    for (size_t i = 0; i < metadata->length; i++) {
        if (!pgn_buffer_equal(metadata->items[i].key, key)) {
            continue;
        }

        __pgn_metadata_item_cleanup(&metadata->items[i]);

        for (size_t j = i; j < metadata->length - 1; j++) {
            metadata->items[j] = metadata->items[j + 1];
        }

        metadata->length--;
        break;
    }
}

void pgn_metadata_cleanup(pgn_metadata_t *metadata)
{
    for (size_t i = 0; i < metadata->length; i++) {
        __pgn_metadata_item_cleanup(&metadata->items[i]);
    }

    free(metadata->items);
    free(metadata);
}
