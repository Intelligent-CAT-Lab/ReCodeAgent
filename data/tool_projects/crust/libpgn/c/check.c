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

#include "check.h"

pgn_check_t __pgn_check_from_string(char *str, size_t *consumed)
{
    size_t cursor = 0;
    pgn_check_t check = PGN_CHECK_NONE;

    switch (str[cursor]) {
        case '+':
            while (str[cursor] == '+') {
                check++;
                cursor++;
            }

            assert(check <= 2);
            break;
        case '#':
            check = PGN_CHECK_MATE;
            cursor++;
            break;
    }

    *consumed += cursor;
    return check;
}

pgn_check_t pgn_check_from_string(char *str)
{
    size_t consumed = 0;
    return __pgn_check_from_string(str, &consumed);
}
