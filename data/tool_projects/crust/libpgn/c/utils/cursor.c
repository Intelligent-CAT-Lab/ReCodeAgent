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
#include <ctype.h>

#include "cursor.h"

bool pgn_cursor_skip_whitespace(char *str, size_t *cursor)
{
    bool skipped = false;

    while (isspace(str[*cursor])) {
        (*cursor)++;
        skipped = true;
    }

    return skipped;
}

bool pgn_cursor_revisit_whitespace(char *str, size_t *cursor)
{
    bool skipped = false;

    while (isspace(str[*cursor - 1])) {
        (*cursor)--;
        skipped = true;
    }

    return skipped;
}

bool pgn_cursor_skip_comment(char *str, size_t *cursor)
{
    if (str[*cursor] != '{')
        return false;

    (*cursor)++;

    unsigned int left_brace_count = 1, right_brace_count = 0; 
    while (right_brace_count != left_brace_count) {
        /* TODO: how to error idk???
         * maybe return corrupted err file idk wtf
         */
        if (str[*cursor] == '\0')
            abort();

        left_brace_count += str[*cursor] == '{';
        right_brace_count += str[*cursor] == '}';
        (*cursor)++;
    }

    return true;
}

bool pgn_cursor_skip_newline(char *str, size_t *cursor)
{
    if (str[*cursor] == '\r') {
        assert(str[(*cursor)++] == '\r');
        assert(str[(*cursor)++] == '\n');
        return true;
    }

    assert(str[(*cursor)++] == '\n');
    return true;
}
