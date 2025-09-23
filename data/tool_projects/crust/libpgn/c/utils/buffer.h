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

#ifndef __LIBPGN_BUFFER_H
#define __LIBPGN_BUFFER_H

#include <stddef.h>
#include <stdbool.h>

#define PGN_BUFFER_INITIAL_SIZE 16
#define PGN_BUFFER_GROW_SIZE 32

typedef struct pgn_buffer_t {
    char *buf;
    size_t length;
    size_t size;
} pgn_buffer_t;

pgn_buffer_t *pgn_buffer_init(void);
void pgn_buffer_grow(pgn_buffer_t *buf);
void pgn_buffer_reset(pgn_buffer_t *buf);
bool pgn_buffer_equal(pgn_buffer_t *buf, char *str);
void pgn_buffer_append(pgn_buffer_t *buf, char ch);
void pgn_buffer_append_null_terminator(pgn_buffer_t *buf);
void pgn_buffer_concat(pgn_buffer_t *buf, char *str);

/* Same as cleanup, but free() is called only
 * to the struct buf itself, the value is returned and
 * the user who calls this should free the detached buf themselves.
 */
char *pgn_buffer_detach(pgn_buffer_t *buf);
void pgn_buffer_cleanup(pgn_buffer_t *buf);

#endif // __LIBPGN_BUFFER_H
