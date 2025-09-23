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

#ifndef __LIBPGN_COMMENTS_H
#define __LIBPGN_COMMENTS_H

#include <stddef.h>

#include "utils/buffer.h"

typedef enum pgn_comment_position_t {
    PGN_COMMENT_POSITION_UNKNOWN = 0,

    /* {} 1. e4
     * ^ comment is here
     *
     * {} 1... e5
     * ^ comment is here
     */
    PGN_COMMENT_POSITION_BEFORE_MOVE,

    /* 1. {} e4
     *    ^ comment is here
     *
     * 1... {} e5
     *      ^ comment is here
     */
    PGN_COMMENT_POSITION_BETWEEN_MOVE,

    /* 1. e4 {}
     *       ^ comment is here
     *
     * 1... e5 {}
     *         ^ comment is here
     */
    PGN_COMMENT_POSITION_AFTER_MOVE,

    /* You can get the index of which alternative comment using `__alt_index`
     *
     * 1. e4 (1. f4) {} (1. g4)
     *               ^ comment is here
     * 
     * TODO: how else if not including an index variable?
     * 
     *   - we can check the index relative to the index of the first after alt comment,
     *     current_index = i
     *     index = find_first_after_alternative_index(comments) + current_index
     */
    PGN_COMMENT_POSITION_AFTER_ALTERNATIVE,
} pgn_comment_position_t;

#define PGN_COMMENTS_INITIAL_SIZE 1
#define PGN_COMMENTS_GROW_SIZE 1

typedef struct pgn_comment_t {
    /* relative to a move */
    pgn_comment_position_t position;
    pgn_buffer_t *value;
} pgn_comment_t;

typedef struct pgn_comments_t {
    pgn_comment_t *values;
    size_t length;
    size_t size;
} pgn_comments_t;

void pgn_comment_init(pgn_comment_t *comment);
/* @abstraction
 *
 * it's the same as
 *
 * ```
 *   char *value = comment->value->buf;
 * ```
 */
char *pgn_comment_value(pgn_comment_t *comment);
pgn_comment_t __pgn_comment_from_string(char *str, size_t *consumed);
void pgn_comment_cleanup(pgn_comment_t *comment);

pgn_comments_t *pgn_comments_init(void);
void pgn_comments_push(pgn_comments_t *comments, pgn_comment_t comment);
/* parse all comments into **comments
 *
 * also skips whitespaces if comment is present
 *
 * returns how much char parsed
 */
size_t pgn_comments_poll(pgn_comments_t **comments, pgn_comment_position_t pos, char *str);
int pgn_comments_get_first_after_alternative_index(pgn_comments_t *comments);
void pgn_comments_cleanup(pgn_comments_t *comments);

#endif // __LIBPGN_COMMENTS_H
