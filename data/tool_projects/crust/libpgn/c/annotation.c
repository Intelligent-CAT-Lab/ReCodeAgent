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

#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

#include "annotation.h"
#include "utils/cursor.h"

pgn_annotation_t __pgn_annotation_nag_from_string(char *str, size_t *consumed)
{
    pgn_annotation_t annotation = PGN_ANNOTATION_UNKNOWN;
    size_t cursor = 0;

    /* works for not revisiting whitespaces
     */
    if (str[cursor] != '$')
        return annotation;

    /* NOTE: NAG annotation tends to have $0 followed by another $<num>.
     * thus why the while loop, take the last.
     */
    while (str[cursor] == '$')
    {
        assert(isdigit(str[++cursor]));

        /* TODO: don't discard the rest.
         */
        char *endptr = NULL;
        long num = strtol(str + cursor, &endptr, 10);

        annotation = num;
        cursor += endptr - (str + cursor);

        pgn_cursor_skip_whitespace(str, &cursor);
    }
    pgn_cursor_revisit_whitespace(str, &cursor);

    *consumed += cursor;
    return annotation;
}

pgn_annotation_t __pgn_annotation_from_string(char *str, size_t *consumed)
{
    /* TODO: maybe refactor using strcmp()?
     */
    pgn_annotation_t annotation = PGN_ANNOTATION_UNKNOWN;

    if (str[0] == '\0')
        return annotation;

    if (str[0] == '!') {
        (*consumed)++;
        annotation = PGN_ANNOTATION_GOOD_MOVE;
    }

    if (str[0] == '?') {
        (*consumed)++;
        annotation = PGN_ANNOTATION_MISTAKE;
    }

    if (str[1] == '\0')
        return annotation;

    if (str[0] == '!' && str[1] == '!') {
        (*consumed)++;
        annotation = PGN_ANNOTATION_BRILLIANT_MOVE;
    }

    if (str[0] == '!' && str[1] == '?') {
        (*consumed)++;
        annotation = PGN_ANNOTATION_INTRESTING_MOVE;
    }

    if (str[0] == '?' && str[1] == '!') {
        (*consumed)++;
        annotation = PGN_ANNOTATION_DUBIOUS_MOVE;
    }

    if (str[0] == '?' && str[1] == '?') {
        (*consumed)++;
        annotation = PGN_ANNOTATION_BLUNDER;
    }

    return annotation;
}

pgn_annotation_t pgn_annotation_from_string(char *str)
{
    size_t consumed = 0;
    return __pgn_annotation_from_string(str, &consumed);
}

size_t pgn_annotation_to_string(pgn_annotation_t annotation, char *dest)
{
    switch (annotation) {
    case PGN_ANNOTATION_UNKNOWN: return 0;
    case PGN_ANNOTATION_NULL: break;
    case PGN_ANNOTATION_GOOD_MOVE:
        strcpy(dest, "!");
        return 1;
    case PGN_ANNOTATION_MISTAKE:
        strcpy(dest, "?");
        return 1;
    case PGN_ANNOTATION_BRILLIANT_MOVE:
        strcpy(dest, "!!");
        return 2;
    case PGN_ANNOTATION_BLUNDER:
        strcpy(dest, "??");
        return 2;
    case PGN_ANNOTATION_INTRESTING_MOVE:
        strcpy(dest, "!?");
        return 2;
    case PGN_ANNOTATION_DUBIOUS_MOVE:
        strcpy(dest, "?!");
        return 2;
    }

    int bytes = sprintf(dest, "$%d", annotation);
    assert(bytes >= 0);

    return bytes;
}
