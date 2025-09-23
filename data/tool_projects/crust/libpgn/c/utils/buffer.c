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

#include "buffer.h"

pgn_buffer_t *pgn_buffer_init(void)
{
    pgn_buffer_t *buf = malloc(sizeof *buf);
    buf->buf = malloc((sizeof *buf->buf) * PGN_BUFFER_INITIAL_SIZE);
    buf->size = PGN_BUFFER_INITIAL_SIZE;

    pgn_buffer_reset(buf);
    return buf;
}

bool pgn_buffer_equal(pgn_buffer_t *buf, char *str)
{
    for (size_t i = 0; i <= buf->length; i++) {
        if (buf->buf[i] != str[i]) {
            return false;
        }
    }

    return true;
}

void pgn_buffer_grow(pgn_buffer_t *buf)
{
    buf->size += PGN_BUFFER_GROW_SIZE;
    buf->buf = realloc(buf->buf, (sizeof *buf->buf) * buf->size);
}

void pgn_buffer_reset(pgn_buffer_t *buf)
{
    buf->length = 0;
    pgn_buffer_append_null_terminator(buf);
}

void pgn_buffer_append(pgn_buffer_t *buf, char ch)
{
    if (buf->length >= buf->size) {
        pgn_buffer_grow(buf);
    }

    buf->buf[buf->length++] = ch;
}

void pgn_buffer_append_null_terminator(pgn_buffer_t *buf)
{
    if (buf->length >= buf->size) {
        pgn_buffer_grow(buf);
    }

    buf->buf[buf->length] = '\0';
}

void pgn_buffer_concat(pgn_buffer_t *buf, char *str)
{
    for (; *str != '\0'; str++) {
        pgn_buffer_append(buf, *str);
    }

    pgn_buffer_append_null_terminator(buf);
}

char *pgn_buffer_detach(pgn_buffer_t *buf)
{
    char *value = buf->buf;
    free(buf);
    return value;
}

void pgn_buffer_cleanup(pgn_buffer_t *buf)
{
    free(buf->buf);
    free(buf);
}

