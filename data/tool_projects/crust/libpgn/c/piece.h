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

#ifndef __LIBPGN_PIECE_H
#define __LIBPGN_PIECE_H

typedef enum pgn_piece_t {
    PGN_PIECE_UNKNOWN = 0,
    PGN_PIECE_PAWN    = 'P',
    PGN_PIECE_ROOK    = 'R',
    PGN_PIECE_KNIGHT  = 'N',
    PGN_PIECE_BISHOP  = 'B',
    PGN_PIECE_QUEEN   = 'Q',
    PGN_PIECE_KING    = 'K',
} pgn_piece_t;

pgn_piece_t pgn_piece_from_char(char ch);
char *pgn_piece_to_string(pgn_piece_t piece);

#endif // __LIBPGN_PIECE_H
