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

#include "pgn.h"

#include "utils/cursor.h"

pgn_t *pgn_init(void)
{
    pgn_t *pgn = malloc(sizeof *pgn);
    pgn->metadata = NULL;
    pgn->moves = NULL;
    return pgn;
}

void pgn_cleanup(pgn_t *pgn)
{
    if (pgn->metadata)
        pgn_metadata_cleanup(pgn->metadata);

    if (pgn->moves)
        pgn_moves_cleanup(pgn->moves);

    free(pgn);
}

pgn_metadata_t *pgn_parse_metadata(char *str)
{
    return pgn_metadata_from_string(str);
}

pgn_move_t pgn_parse_move(char *str)
{
    return pgn_move_from_string(str);
}

pgn_moves_t *pgn_parse_moves(char *str)
{
    return pgn_moves_from_string(str);
}

pgn_score_t pgn_parse_score(char *str)
{
    return pgn_score_from_string(str);
}

size_t pgn_parse(pgn_t *pgn, char *str)
{
    size_t cursor = 0;

    pgn->metadata = __pgn_metadata_from_string(str + cursor, &cursor);
    pgn_cursor_skip_whitespace(str, &cursor);

    pgn->moves = __pgn_moves_from_string(str + cursor, &cursor);
    pgn->score = __pgn_score_from_string(str + cursor, &cursor);

    return cursor;
}
