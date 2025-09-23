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

#ifndef __LIBPGN_CHECK_H
#define __LIBPGN_CHECK_H

#include <stddef.h>

typedef enum pgn_check_t {
    PGN_CHECK_MATE = -1,
    PGN_CHECK_NONE = 0,
    PGN_CHECK_SINGLE,
    PGN_CHECK_DOUBLE,
} pgn_check_t;

pgn_check_t __pgn_check_from_string(char *str, size_t *consumed);
pgn_check_t pgn_check_from_string(char *str);

#endif // __LIBPGN_CHECK_H
