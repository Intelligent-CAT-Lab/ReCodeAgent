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

#include <ctype.h>
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "moves.h"
#include "annotation.h"
#include "check.h"
#include "comments.h"
#include "piece.h"
#include "score.h"

#include "utils/cursor.h"

#define PGN_EXPECT_WHITE 0
#define PGN_EXPECT_BLACK 1

pgn_move_t __pgn_move_from_string(char *str, size_t *consumed)
{
    pgn_move_t move = {0};
    size_t cursor = 0;
    
    if (str[cursor] == 'O') {
        assert(str[++cursor] == '-');
        assert(str[++cursor] == 'O');
        move.castles = PGN_CASTLING_KINGSIDE;
        cursor++;

        if (str[cursor] == '-') {
            assert(str[++cursor] == 'O');
            move.castles = PGN_CASTLING_QUEENSIDE;
            cursor++;
        }

        goto check;
    }

    move.piece = pgn_piece_from_char(str[cursor++]);
    if (move.piece == PGN_PIECE_UNKNOWN) {
        move.piece = PGN_PIECE_PAWN;
        cursor--;
    }

    /* NOTE: can be refactored using a stack
     */
    if (islower(str[cursor]) && str[cursor] != 'x') move.from.file = str[cursor++];
    if (isdigit(str[cursor])) move.from.rank = str[cursor++] - '0';

    move.captures = str[cursor] == 'x' || str[cursor] == ':';
    cursor += move.captures;

    if (islower(str[cursor])) {
        assert(islower(str[cursor]));
        move.dest.file = str[cursor++];
        assert(isdigit(str[cursor]));
        move.dest.rank = str[cursor++] - '0';
    } else {
        move.dest = move.from;
        move.from = (pgn_coordinate_t){0};
    }

    move.promoted_to = pgn_piece_from_char(str[cursor]);
    if (move.promoted_to == PGN_PIECE_UNKNOWN) {
        switch (str[cursor]) {
        case '(':
            move.promoted_to = pgn_piece_from_char(str[++cursor]);
            assert(move.promoted_to != PGN_PIECE_UNKNOWN);
            assert(str[++cursor] == ')');
            cursor++;
            break;
        case '=':
        case '/':
            move.promoted_to = pgn_piece_from_char(str[++cursor]);
            assert(move.promoted_to != PGN_PIECE_UNKNOWN);
            cursor++;
            break;
        }
    }

    assert(move.dest.file != 0);
    assert(move.dest.rank != 0);

check:
    move.check = __pgn_check_from_string(str + cursor, &cursor);
    move.annotation = __pgn_annotation_from_string(str + cursor, &cursor);

    bool skipped_whitespace_before_ep = pgn_cursor_skip_whitespace(str, &cursor);

    /* could be en passant */
    if (str[cursor] == 'e' && str[cursor + 1] == '.') {
        /* TODO: error no spaces in between
         */
        assert(skipped_whitespace_before_ep);

        assert(str[cursor] == 'e');
        assert(str[++cursor] == '.');
        assert(str[++cursor] == 'p');
        assert(str[++cursor] == '.');
        cursor++;
        move.en_passant = true;
    }

    bool skipped_whitespace_after_ep = move.en_passant ? pgn_cursor_skip_whitespace(str, &cursor) : skipped_whitespace_before_ep;

    /* Check for NAG annotation.
     */
    if (move.annotation == PGN_ANNOTATION_UNKNOWN) {
        move.annotation = __pgn_annotation_nag_from_string(str + cursor, &cursor);

        /* TODO: error no spaces in between
         */
        if (move.annotation != PGN_ANNOTATION_UNKNOWN) {
            assert(skipped_whitespace_after_ep);
        }
    }

    pgn_cursor_revisit_whitespace(str, &cursor);

    size_t notation_len = cursor;
    strncpy(move.notation, str, notation_len);

    *consumed += cursor;
    return move;
}

pgn_move_t pgn_move_from_string(char *str)
{
    size_t consumed = 0;
    return __pgn_move_from_string(str, &consumed);
}

size_t pgn_move_to_string(pgn_move_t *move, char *dest)
{
    size_t cursor = 0;

    switch (move->castles) {
    case PGN_CASTLING_NONE: break;
    case PGN_CASTLING_KINGSIDE:
        strcpy(dest, "O-O");
        cursor += PGN_CASTLING_KINGSIDE * 2 - 1;
        goto check;
    case PGN_CASTLING_QUEENSIDE:
        strcpy(dest, "O-O-O");
        cursor += PGN_CASTLING_QUEENSIDE * 2 - 1;
        goto check;
    }

    assert(move->piece != PGN_PIECE_UNKNOWN);
    if (move->piece != PGN_PIECE_PAWN) {
        dest[cursor++] = move->piece;
    }

    if (move->from.file) dest[cursor++] = move->from.file;
    if (move->from.rank) dest[cursor++] = '0' + move->from.rank;

    if (move->captures) dest[cursor++] = 'x';

    dest[cursor++] = move->dest.file;
    dest[cursor++] = '0' + move->dest.rank;

    if (move->promoted_to) {
        assert(move->piece == PGN_PIECE_PAWN);
        assert(move->promoted_to != PGN_PIECE_PAWN);
        dest[cursor++] = '=';
        dest[cursor++] = move->promoted_to;
    }

check:
    switch (move->check) {
        case PGN_CHECK_NONE:
            break;
        case PGN_CHECK_MATE:
            dest[cursor++] = '#';
            break;
        case PGN_CHECK_SINGLE:
            dest[cursor++] = '+';
            break;
        case PGN_CHECK_DOUBLE:
            dest[cursor++] = '+';
            dest[cursor++] = '+';
            break;
    }

    if (move->annotation >= PGN_ANNOTATION_GOOD_MOVE && move->annotation <= PGN_ANNOTATION_DUBIOUS_MOVE) {
        cursor += pgn_annotation_to_string(move->annotation, dest + cursor);
    }

    if (move->en_passant) {
        dest[cursor++] = ' ';
        strcpy(dest + cursor, "e.p.");
        cursor += 4;
    }

    if (move->annotation > PGN_ANNOTATION_DUBIOUS_MOVE || move->annotation == PGN_ANNOTATION_NULL) {
        dest[cursor++] = ' ';
        cursor += pgn_annotation_to_string(move->annotation, dest + cursor);
    }

    dest[cursor] = '\0';
    return cursor;
}

pgn_moves_t *__pgn_moves_from_string_recurse(char *str, size_t *consumed, pgn_moves_t *moves, int expect)
{
    if (str[0] == ')' || str[0] == '\0')
        return moves;

    size_t cursor = 0;
    __pgn_moves_item_t move = {0};

    /* comments placeholder to wait for knowing which move the comment belongs to.
     */
    pgn_comments_t *comments = NULL;

    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_BEFORE_MOVE, str + cursor);

    assert(isdigit(str[cursor]));
    while (isdigit(str[cursor])) cursor++;

    int dots_count = 0;
    assert(str[cursor] == '.');
    while (str[cursor] == '.') {
        cursor++;
        dots_count++;
    }

    if (expect == PGN_EXPECT_WHITE) assert(dots_count == 1);
    if (expect == PGN_EXPECT_BLACK) assert(dots_count == 3);

    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_BETWEEN_MOVE, str + cursor);

    if (dots_count == 3) {
        move.black = __pgn_move_from_string(str + cursor, &cursor);

        pgn_cursor_skip_whitespace(str, &cursor);
        cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_AFTER_MOVE, str + cursor);
        cursor += pgn_alternative_moves_poll(&move.black.alternatives, &comments, str + cursor, PGN_EXPECT_BLACK);
        pgn_cursor_skip_whitespace(str, &cursor);
        cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_AFTER_MOVE, str + cursor);

        if (comments) {
            move.black.comments = comments;
            comments = NULL;
        }

        pgn_moves_push(moves, move);
        __pgn_moves_from_string_recurse(str + cursor, &cursor, moves, PGN_EXPECT_WHITE);
        *consumed += cursor;
        return moves;
    }

    move.white = __pgn_move_from_string(str + cursor, &cursor);
    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_AFTER_MOVE, str + cursor);
    cursor += pgn_alternative_moves_poll(&move.white.alternatives, &comments, str + cursor, PGN_EXPECT_WHITE);
    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_AFTER_MOVE, str + cursor);

    if (comments) {
        move.white.comments = comments;
        comments = NULL;
    }

    if (pgn_score_from_string(str + cursor)) {
        pgn_moves_push(moves, move);
        *consumed += cursor;
        return moves;
    }

    /* We're at the end of pgn, no black move present.
     *
     * TODO: investigate (might be a corrupt file?)
     */
    if (str[cursor] == ')' || str[cursor] == '\0') {
        pgn_moves_push(moves, move);
        *consumed += cursor;
        return moves;
    }

    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_BEFORE_MOVE, str + cursor);

    if (isdigit(str[cursor])) {
        while (isdigit(str[cursor])) cursor++;

        for (int i = 0; i < 3; i++)
            assert(str[cursor++] == '.');
    }

    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_BETWEEN_MOVE, str + cursor);

    move.black = __pgn_move_from_string(str + cursor, &cursor);
    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_AFTER_MOVE, str + cursor);
    cursor += pgn_alternative_moves_poll(&move.black.alternatives, &comments, str + cursor, PGN_EXPECT_BLACK);
    pgn_cursor_skip_whitespace(str, &cursor);
    cursor += pgn_comments_poll(&comments, PGN_COMMENT_POSITION_AFTER_MOVE, str + cursor);

    if (comments) {
        move.black.comments = comments;
        comments = NULL;
    }

    pgn_moves_push(moves, move);

    if (pgn_score_from_string(str + cursor)) {
        *consumed += cursor;
        return moves;
    }

    __pgn_moves_from_string_recurse(str + cursor, &cursor, moves, PGN_EXPECT_WHITE);
    *consumed += cursor;
    return moves;
}

pgn_moves_t *__pgn_moves_from_string(char *str, size_t *consumed)
{
    return __pgn_moves_from_string_recurse(str, consumed, pgn_moves_init(), PGN_EXPECT_WHITE);
}

pgn_moves_t *pgn_moves_from_string(char *str)
{
    size_t consumed = 0;
    return __pgn_moves_from_string(str, &consumed);
}

pgn_alternative_moves_t *pgn_alternative_moves_init(void)
{
    pgn_alternative_moves_t *alt = malloc(sizeof *alt);
    alt->values = malloc(sizeof *alt->values * PGN_ALTERNATIVE_MOVES_INITIAL_SIZE);
    alt->size = PGN_ALTERNATIVE_MOVES_INITIAL_SIZE;
    alt->length = 0;
    return alt;
}

size_t pgn_alternative_moves_poll(pgn_alternative_moves_t **alt, pgn_comments_t **placeholder, char *str, int expect)
{
    size_t cursor = 0;

    while (str[cursor] == '(') {
        cursor++;

        if (!*alt)
            *alt = pgn_alternative_moves_init();

        pgn_cursor_skip_whitespace(str, &cursor);
        pgn_alternative_moves_push(*alt, __pgn_moves_from_string_recurse(str + cursor, &cursor, pgn_moves_init(), expect));
        pgn_cursor_skip_whitespace(str, &cursor);
        assert(str[cursor++] == ')');

        pgn_cursor_skip_whitespace(str, &cursor);
        cursor += pgn_comments_poll(placeholder, PGN_COMMENT_POSITION_AFTER_ALTERNATIVE, str + cursor);
    }

    return cursor;
}

void pgn_alternative_moves_push(pgn_alternative_moves_t *alt, pgn_moves_t *moves)
{
    if (alt->length >= alt->size) {
        alt->size += PGN_ALTERNATIVE_MOVES_GROW_SIZE;
        alt->values = realloc(alt->values,  (sizeof *alt->values) * alt->size);
    }

    alt->values[alt->length++] = moves;
}

void pgn_alternative_moves_cleanup(pgn_alternative_moves_t *alt)
{
    for (size_t i = 0; i < alt->length; i++) {
        pgn_moves_cleanup(alt->values[i]);
    }

    free(alt->values);
    free(alt);
}

pgn_moves_t *pgn_moves_init(void)
{
    pgn_moves_t *moves = malloc(sizeof *moves);
    moves->values = malloc(sizeof *moves->values * PGN_MOVES_INITIAL_SIZE);
    moves->size = PGN_MOVES_INITIAL_SIZE;
    moves->length = 0;
    return moves;
}

void pgn_moves_push(pgn_moves_t *moves, __pgn_moves_item_t __moves)
{
    if (moves->length >= moves->size) {
        moves->size += PGN_MOVES_GROW_SIZE;
        moves->values = realloc(moves->values,  (sizeof *moves->values) * moves->size);
    }

    moves->values[moves->length++] = __moves;
}

void pgn_moves_cleanup(pgn_moves_t *moves)
{
    for (size_t i = 0; i < moves->length; i++) {
        if (moves->values[i].white.comments) {
            pgn_comments_cleanup(moves->values[i].white.comments);
        }

        if (moves->values[i].black.comments) {
            pgn_comments_cleanup(moves->values[i].black.comments);
        }

        if (moves->values[i].white.alternatives) {
            pgn_alternative_moves_cleanup(moves->values[i].white.alternatives);
        }

        if (moves->values[i].black.alternatives) {
            pgn_alternative_moves_cleanup(moves->values[i].black.alternatives);
        }
    }

    free(moves->values);
    free(moves);
}
