#include "extra/suite6.h"

#include <pgn.h>
#include <string.h>

void test_parsing_moves_with_comments_nested(void)
{
    pgn_moves_t *moves;

    moves = pgn_moves_from_string("1.e4 { a comment {inside a comment} ? } e5");
    suite6_assert(moves->values[0].white.comments != NULL);
    suite6_assert(moves->values[0].white.comments->length == 1);
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, " a comment {inside a comment} ? "));
    pgn_moves_cleanup(moves);
}

void test_parsing_comments_on_position_before(void)
{
    pgn_moves_t *moves;

    moves = pgn_moves_from_string("{ hello} 1.e4 e5");
    suite6_assert(moves->values[0].white.comments != NULL);
    suite6_assert(moves->values[0].white.comments->length == 1);
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_BEFORE_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, " hello"));
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("{ hello} {libpgn} 1.e4 e5");
    suite6_assert(moves->values[0].white.comments != NULL);
    suite6_assert(moves->values[0].white.comments->length == 2);
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_BEFORE_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, " hello"));
    suite6_assert(moves->values[0].white.comments->values[1].position == PGN_COMMENT_POSITION_BEFORE_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[1].value->buf, "libpgn"));
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1.e4 e5 ({hello2} 1... g5)");
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.comments->length == 1);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[0].position == PGN_COMMENT_POSITION_BEFORE_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[0].value->buf, "hello2"));
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1.e4 e5 ({hello} {libpgn} 1... g5)");
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.comments->length == 2);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[0].position == PGN_COMMENT_POSITION_BEFORE_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[0].value->buf, "hello"));
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[1].position == PGN_COMMENT_POSITION_BEFORE_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[1].value->buf, "libpgn"));
    pgn_moves_cleanup(moves);
}

void test_parsing_comments_on_position_between(void)
{
    pgn_moves_t *moves;

    moves = pgn_moves_from_string("1. {jackalope:w} e4 e5");
    suite6_assert(moves->values[0].white.comments != NULL);
    suite6_assert(moves->values[0].white.comments->length == 1);
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_BETWEEN_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, "jackalope:w"));
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1.e4 e5 (1... {:::::::::::::} g5)");
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.comments->length == 1);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[0].position == PGN_COMMENT_POSITION_BETWEEN_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.alternatives->values[0]->values[0].black.comments->values[0].value->buf, ":::::::::::::"));
    pgn_moves_cleanup(moves);
}

void test_parsing_comments_on_position_after(void)
{
    pgn_moves_t *moves;

    moves = pgn_moves_from_string("1.e4 {hello} {from} {libpgn} e5");
    suite6_assert(moves->values[0].white.comments != NULL);
    suite6_assert(moves->values[0].white.comments->length == 3);
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, "hello"));
    suite6_assert(moves->values[0].white.comments->values[1].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[1].value->buf, "from"));
    suite6_assert(moves->values[0].white.comments->values[2].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[2].value->buf, "libpgn"));
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1.e4 {dump} e5 { spaces does not get trimmed } { trim it with pgn_comment_trim() } {this makes the pgn file original -> } ");
    suite6_assert(moves->values[0].white.comments != NULL);
    suite6_assert(moves->values[0].white.comments->length == 1);
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, "dump"));
    suite6_assert(moves->values[0].black.comments->length == 3);
    suite6_assert(moves->values[0].black.comments->values[0].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.comments->values[0].value->buf, " spaces does not get trimmed "));
    suite6_assert(moves->values[0].black.comments->values[1].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.comments->values[1].value->buf, " trim it with pgn_comment_trim() "));
    suite6_assert(moves->values[0].black.comments->values[2].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.comments->values[2].value->buf, "this makes the pgn file original -> "));
    pgn_moves_cleanup(moves);
}


void test_parsing_comments_on_position_after_alt()
{
    unsigned int alt_index;
    pgn_moves_t *moves;

    moves = pgn_moves_from_string("1. e4 e5 {IGNORE MEEE} (1... f5) {hello} {again}");
    suite6_assert(moves->values[0].black.comments != NULL);
    suite6_assert(moves->values[0].black.comments->length == 3);
    suite6_assert(moves->values[0].black.comments->values[0].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].black.comments->values[0].value->buf, "IGNORE MEEE"));
    suite6_assert(moves->values[0].black.comments->values[1].position == PGN_COMMENT_POSITION_AFTER_ALTERNATIVE);
    alt_index = 1 - pgn_comments_get_first_after_alternative_index(moves->values[0].black.comments);
    suite6_assert(alt_index == 0);
    suite6_assert(!strcmp(moves->values[0].black.comments->values[1].value->buf, "hello"));
    suite6_assert(moves->values[0].black.comments->values[2].position == PGN_COMMENT_POSITION_AFTER_ALTERNATIVE);
    alt_index = 2 - pgn_comments_get_first_after_alternative_index(moves->values[0].black.comments);
    suite6_assert(alt_index == 1);
    suite6_assert(!strcmp(moves->values[0].black.comments->values[2].value->buf, "again"));
    pgn_moves_cleanup(moves);
}

void test_parsing_comments_on_position_all()
{
    pgn_moves_t *moves;

    moves = pgn_moves_from_string("{hello} 1. {from}  e4 {libpgn} e5 (1... f5) {dawg}");
    suite6_assert(moves->values[0].white.comments != NULL);
    suite6_assert(moves->values[0].white.comments->length == 3);
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_BEFORE_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, "hello"));
    suite6_assert(moves->values[0].white.comments->values[1].position == PGN_COMMENT_POSITION_BETWEEN_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[1].value->buf, "from"));
    suite6_assert(moves->values[0].white.comments->values[2].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[2].value->buf, "libpgn"));
    suite6_assert(moves->values[0].black.comments->values[0].position == PGN_COMMENT_POSITION_AFTER_ALTERNATIVE);
    unsigned int alt_index = 0 - pgn_comments_get_first_after_alternative_index(moves->values[0].black.comments);
    suite6_assert(alt_index == 0);
    suite6_assert(!strcmp(moves->values[0].black.comments->values[0].value->buf, "dawg"));
    pgn_moves_cleanup(moves);
}

int main(void)
{
    test_parsing_moves_with_comments_nested();
    test_parsing_comments_on_position_before();
    test_parsing_comments_on_position_between();
    test_parsing_comments_on_position_after();
    test_parsing_comments_on_position_all();
    return 0;
}
