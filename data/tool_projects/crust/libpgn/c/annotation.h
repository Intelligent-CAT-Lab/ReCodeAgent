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

#ifndef __LIBPGN_ANNOTATION_H
#define __LIBPGN_ANNOTATION_H

#include <stdlib.h>

/* See:
 *   http://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c10
 *   https://en.wikipedia.org/wiki/Numeric_Annotation_Glyphs#Standard_NAGs
 */
typedef enum pgn_annotation_t {
    PGN_ANNOTATION_UNKNOWN = -1,    //
    PGN_ANNOTATION_NULL = 0,        //
    PGN_ANNOTATION_GOOD_MOVE,       // !
    PGN_ANNOTATION_MISTAKE,         // ?
    PGN_ANNOTATION_BRILLIANT_MOVE,  // !!
    PGN_ANNOTATION_BLUNDER,         // ??
    PGN_ANNOTATION_INTRESTING_MOVE, // !?
    PGN_ANNOTATION_DUBIOUS_MOVE,    // ?!
} pgn_annotation_t;

pgn_annotation_t __pgn_annotation_nag_from_string(char *str, size_t *consumed);
pgn_annotation_t __pgn_annotation_from_string(char *str, size_t *consumed);
pgn_annotation_t pgn_annotation_from_string(char *str);
size_t pgn_annotation_to_string(pgn_annotation_t annotation, char *dest);

#endif // __LIBPGN_ANNOTATION_H
