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

#include "comments.h"
#include "utils/cursor.h"
#include "utils/buffer.h"

void pgn_comment_init(pgn_comment_t *comment)
{
    comment->value = pgn_buffer_init();
    comment->position = PGN_COMMENT_POSITION_UNKNOWN;
}

char *pgn_comment_value(pgn_comment_t *comment)
{
    return comment->value->buf;
}

pgn_comment_t __pgn_comment_from_string(char *str, size_t *consumed)
{
    size_t cursor = 0;
    pgn_comment_t comment;
    pgn_comment_init(&comment);

    assert(str[cursor++] == '{');

    unsigned int left_brace_count = 1, right_brace_count = 0; 
    while (true) {
        /* TODO: how to error idk???
         * maybe return corrupted err file idk wtf
         */
        if (str[cursor] == '\0')
            abort();

        left_brace_count += str[cursor] == '{';
        right_brace_count += str[cursor] == '}';

        if (right_brace_count == left_brace_count) {
            break;
        }

        pgn_buffer_append(comment.value, str[cursor]);
        cursor++;
    }

    assert(str[cursor++] == '}');
    pgn_buffer_append_null_terminator(comment.value);

    *consumed += cursor;
    return comment;
}

void pgn_comment_cleanup(pgn_comment_t *comment)
{
    pgn_buffer_cleanup(comment->value);
}

pgn_comments_t *pgn_comments_init(void)
{
    pgn_comments_t *comments = malloc(sizeof *comments);
    comments->values = malloc((sizeof *comments->values) * PGN_COMMENTS_INITIAL_SIZE);
    comments->size = PGN_COMMENTS_INITIAL_SIZE;
    comments->length = 0;
    return comments;
}

void pgn_comments_push(pgn_comments_t *comments, pgn_comment_t comment)
{
    if (comments->length >= comments->size) {
        comments->size += PGN_COMMENTS_GROW_SIZE;
        comments->values = realloc(comments->values, (sizeof *comments->values) * comments->size);
    }

    comments->values[comments->length++] = comment;
}

size_t pgn_comments_poll(pgn_comments_t **comments, pgn_comment_position_t pos, char *str)
{
    size_t cursor = 0;

    if (str[cursor] == '{') {
        if (!*comments) *comments = pgn_comments_init();

        while (str[cursor] == '{') {
            pgn_comment_t comment = __pgn_comment_from_string(str + cursor, &cursor);
            comment.position = pos;

            pgn_comments_push(*comments, comment);
            pgn_cursor_skip_whitespace(str, &cursor);
        }

        assert(str[cursor] != '{');
        assert(str[cursor] != '}');
        pgn_cursor_skip_whitespace(str, &cursor);
    }
    
    return cursor;
}

int pgn_comments_get_first_after_alternative_index(pgn_comments_t *comments)
{
    for (int i = 0; i < (int)comments->length; i++) {
        if (comments->values[i].position == PGN_COMMENT_POSITION_AFTER_ALTERNATIVE) {
            return i;
        }
    }

    return -1;
}

void pgn_comments_cleanup(pgn_comments_t *comments)
{
    for (size_t i = 0; i < comments->length; i++) {
        pgn_comment_cleanup(&comments->values[i]);
    }

    free(comments->values);
    free(comments);
}
