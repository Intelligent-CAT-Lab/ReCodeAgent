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

#ifndef __LIBPGN_PGN_H
#define __LIBPGN_PGN_H

#include "metadata.h"
#include "piece.h"
#include "annotation.h"
#include "coordinate.h"
#include "score.h"
#include "check.h"
#include "moves.h"

#include "utils/export.h"

#define PGN_UNUSED(x) ((void)x)
#define PGN_NOT_IMPLEMENTED()                                                                                      \
    do {                                                                                                           \
        fprintf(stderr, "libpgn: function `%s` is not implemented yet, at %s:%d\n", __func__, __FILE__, __LINE__); \
        exit(EXIT_FAILURE);                                                                                        \
    } while (0)

typedef struct pgn_t {
    pgn_metadata_t *metadata;
    pgn_moves_t *moves;
    pgn_score_t score;
} pgn_t;

PGN_EXPORT pgn_t *pgn_init(void);
PGN_EXPORT void pgn_cleanup(pgn_t *pgn);

PGN_EXPORT pgn_metadata_t *pgn_parse_metadata(char *str);
PGN_EXPORT pgn_move_t pgn_parse_move(char *str);
PGN_EXPORT pgn_moves_t *pgn_parse_moves(char *str);
PGN_EXPORT pgn_score_t pgn_parse_score(char *str);

PGN_EXPORT size_t pgn_parse(pgn_t *pgn, char *str);

#endif // __LIBPGN_PGN_H
