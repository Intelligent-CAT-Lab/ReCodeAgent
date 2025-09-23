#include <pgn.h>
#include <stdio.h>
#include <stdbool.h>

#include "fmt.h"

#define TAB_WIDTH 4

bool fmt_print_moves(pgn_moves_t *moves, int depth, size_t last_number, bool root);

void fmt_print_tab(int depth, int width)
{
    for (int i = 0; i < depth; i++) {
        for (int j = 0; j < width; j++) {
            printf(" ");
        }
    }
}

bool fmt_print_metadata(pgn_metadata_t *mt)
{
    bool printed = false;

    for (size_t i = 0; i < mt->length; i++) {
        printf("[%s \"%s\"]\n", mt->items[i].key->buf, mt->items[i].value->buf);
        printed = true;
    }

    return printed;
}

bool fmt_print_alternative_moves(pgn_alternative_moves_t *alt, int depth, size_t last_number)
{
    bool printed = false;

    for (size_t i = 0; i < alt->length; i++) {
        printf("\n");
        fmt_print_tab(depth, TAB_WIDTH);
        printf("(");
        fmt_print_moves(alt->values[i], depth, last_number, false);
        printf(")");
    }

    return printed;
}

bool __fmt_print_comment_before(pgn_comments_t *comments)
{
    bool printed = false;

    for (size_t i = 0; i < comments->length; i++) {
        pgn_comment_t comment = comments->values[i];

        if (comment.position == PGN_COMMENT_POSITION_BEFORE_MOVE) {
            printf("{%s} ", comment.value->buf);
        }

        printed = true;
    }

    return printed;
}

bool __fmt_print_comment_after(pgn_comments_t *comments)
{
    bool printed = false;

    for (size_t i = 0; i < comments->length; i++) {
        pgn_comment_t comment = comments->values[i];

        if (comment.position == PGN_COMMENT_POSITION_AFTER_MOVE) {
            printf(" {%s}", comment.value->buf);
        }

        printed = true;
    }

    return printed;
}

bool __fmt_print_white(pgn_move_t *move, size_t depth, size_t number)
{
    bool printed = false;

    if (move->notation[0]) {
        if (move->comments) {
            __fmt_print_comment_before(move->comments);
        }

        printf("%zu. %s", number, move->notation);

        if (move->comments) {
            __fmt_print_comment_after(move->comments);
        }

        if (move->alternatives) {
            fmt_print_alternative_moves(move->alternatives, depth + 1, number);
        }

        printed = true;
    }

    return printed;
}

bool __fmt_print_black(pgn_move_t *move, size_t depth, size_t number, bool white_printed)
{
    bool printed = false;

    if (move->notation[0]) {
        if (white_printed) {
            printf(" ");
        }

        if (move->comments) {
            __fmt_print_comment_before(move->comments);
        }

        if (!white_printed) {
            printf("%zu... ", number);
        }

        printf("%s", move->notation);

        if (move->comments) {
            __fmt_print_comment_after(move->comments);
        }

        if (move->alternatives) {
            fmt_print_alternative_moves(move->alternatives, depth + 1, number);
        }

        printed = true;
    }

    return printed;
}

bool fmt_print_moves(pgn_moves_t *moves, int depth, size_t last_number, bool root)
{
    bool printed = false;

    for (size_t i = 0; i < moves->length; i++) {
        pgn_move_t white = moves->values[i].white;
        pgn_move_t black = moves->values[i].black;

        bool white_printed = __fmt_print_white(&white, depth, last_number + i);
        bool black_printed = __fmt_print_black(&black, depth, last_number + i, white_printed);

        if (i != moves->length - 1) {
            printf("\n");
            fmt_print_tab(depth, TAB_WIDTH);

            if (!root)
                printf(" ");
        }

        printed = true;
    }

    return printed;
}

/* TODO: this is incorrect, try getting
 * the score from the actual end of moves
 */
bool fmt_print_score(pgn_t *pgn)
{
    char *score = pgn_metadata_get(pgn->metadata, "Result");
    if (!score) return false;

    printf("%s\n", score);
    return true;
}

void fmt_print(pgn_t *pgn)
{
    if (fmt_print_metadata(pgn->metadata))
        printf("\n");

    if (fmt_print_moves(pgn->moves, 0, 1, true))
        printf("\n");

    fmt_print_score(pgn);
}
