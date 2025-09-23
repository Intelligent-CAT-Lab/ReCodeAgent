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

#include <string.h>

#include "score.h"

typedef enum pgn_score_single_t {
    ZERO = 0,
    ONE,
    HALF,
} pgn_score_single_t;

/* only parses the score one sided
 */
pgn_score_single_t __pgn_score_single_from_string(char *str, size_t *consumed)
{
    if (strncmp(str, "1/2", 3) == 0) {
        *consumed += 3;
        return HALF;
    } else if (*str == '0') {
        *consumed += 1;
        return ZERO;
    } else if (*str == '1') {
        *consumed += 1;
        return ONE;
    }

    return -1;
}

pgn_score_t __pgn_score_from_string(char *str, size_t *consumed)
{
    size_t cursor = 0;

    if (str[cursor] == '*') {
        cursor++;
        *consumed += cursor;
        return PGN_SCORE_ONGOING;
    }

    pgn_score_single_t white = __pgn_score_single_from_string(str + cursor, &cursor);
    if ((int)white == -1) return PGN_SCORE_UNKNOWN;

    if (str[cursor++] != '-') return PGN_SCORE_UNKNOWN;

    pgn_score_single_t black = __pgn_score_single_from_string(str + cursor, &cursor);
    if ((int)black == -1) return PGN_SCORE_UNKNOWN;

    *consumed += cursor;

    if (white == HALF && black == HALF)   return PGN_SCORE_DRAW;
    else if (white == 1 && black == 0)    return PGN_SCORE_WHITE_WON;
    else if (white == 0 && black == 1)    return PGN_SCORE_BLACK_WON;
    else if (white == 0 && black == 0)    return PGN_SCORE_FORFEIT;
    else if (white == 0 && black == HALF) return PGN_SCORE_WHITE_FORFEIT;
    else if (white == HALF && black == 0) return PGN_SCORE_BLACK_FORFEIT;
    else                                  return PGN_SCORE_UNKNOWN;
}

pgn_score_t pgn_score_from_string(char *str)
{
    size_t consumed = 0;
    return __pgn_score_from_string(str, &consumed);
}

char *pgn_score_to_string(pgn_score_t score)
{
    switch (score) {
    case PGN_SCORE_UNKNOWN:       break;
    case PGN_SCORE_ONGOING:       return "*";
    case PGN_SCORE_DRAW:          return "1/2-1/2";
    case PGN_SCORE_WHITE_WON:     return "1-0";
    case PGN_SCORE_BLACK_WON:     return "0-1";
    case PGN_SCORE_FORFEIT:       return "0-0";
    case PGN_SCORE_WHITE_FORFEIT: return "0-1/2";
    case PGN_SCORE_BLACK_FORFEIT: return "1/2-0";
    }

    return "";
}
