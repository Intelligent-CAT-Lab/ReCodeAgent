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

#ifndef __LIBPGN_SCORE_H
#define __LIBPGN_SCORE_H

#include <stddef.h>

typedef enum pgn_score_t {
    PGN_SCORE_UNKNOWN = 0,
    PGN_SCORE_ONGOING, // From wikipedia: Recorded as White score, dash, then Black score, or * (other, e.g., the game is ongoing).
    PGN_SCORE_DRAW,
    PGN_SCORE_WHITE_WON,
    PGN_SCORE_BLACK_WON,
    PGN_SCORE_FORFEIT,
    PGN_SCORE_WHITE_FORFEIT,
    PGN_SCORE_BLACK_FORFEIT,
} pgn_score_t;

pgn_score_t __pgn_score_from_string(char *str, size_t *consumed);
pgn_score_t pgn_score_from_string(char *str);
char *pgn_score_to_string(pgn_score_t score);

#endif // __LIBPGN_SCORE_H
