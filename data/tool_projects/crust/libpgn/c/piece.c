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

#include <stddef.h>

#include "piece.h"

pgn_piece_t pgn_piece_from_char(char ch)
{
    switch (ch) {
    case PGN_PIECE_PAWN:
    case PGN_PIECE_ROOK:
    case PGN_PIECE_KNIGHT:
    case PGN_PIECE_BISHOP:
    case PGN_PIECE_QUEEN:
    case PGN_PIECE_KING:
        return ch;
    default: return PGN_PIECE_UNKNOWN;
    }
}

char *pgn_piece_to_string(pgn_piece_t piece)
{
    switch (piece) {
    case PGN_PIECE_PAWN: return "Pawn";
    case PGN_PIECE_ROOK: return "Rook";
    case PGN_PIECE_KNIGHT: return "Knight";
    case PGN_PIECE_BISHOP: return "Bishop";
    case PGN_PIECE_QUEEN: return "Queen";
    case PGN_PIECE_KING: return "King";
    default: return NULL;
    }
}
