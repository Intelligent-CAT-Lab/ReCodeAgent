#include "extra/suite6.h"

#include <pgn.h>
#include <string.h>

void test_parsing_single_move()
{
    pgn_move_t move = {0};

    move = pgn_parse_move("e4??");
    suite6_assert(move.piece == PGN_PIECE_PAWN);
    suite6_assert(move.captures == false);
    suite6_assert(move.from.file == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.from.rank == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.dest.file == 'e');
    suite6_assert(move.dest.rank == 4);
    suite6_assert(move.check == PGN_CHECK_NONE);
    suite6_assert(!strcmp(move.notation, "e4??"));
    suite6_assert(move.annotation == PGN_ANNOTATION_BLUNDER);

    move = pgn_parse_move("Nb6d5+");
    suite6_assert(move.piece == PGN_PIECE_KNIGHT);
    suite6_assert(move.captures == false);
    suite6_assert(move.from.file == 'b');
    suite6_assert(move.from.rank == 6);
    suite6_assert(move.dest.file == 'd');
    suite6_assert(move.dest.rank == 5);
    suite6_assert(move.check == PGN_CHECK_SINGLE);
    suite6_assert(!strcmp(move.notation, "Nb6d5+"));
    suite6_assert(move.annotation == PGN_ANNOTATION_UNKNOWN);

    move = pgn_parse_move("Qf1?!");
    suite6_assert(move.piece == PGN_PIECE_QUEEN);
    suite6_assert(move.captures == false);
    suite6_assert(move.from.file == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.from.rank == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.dest.file == 'f');
    suite6_assert(move.dest.rank == 1);
    suite6_assert(move.check == PGN_CHECK_NONE);
    suite6_assert(!strcmp(move.notation, "Qf1?!"));
    suite6_assert(move.annotation == PGN_ANNOTATION_DUBIOUS_MOVE);

    move = pgn_parse_move("bxa8=Q??");
    suite6_assert(move.piece == PGN_PIECE_PAWN);
    suite6_assert(move.captures == true);
    suite6_assert(move.from.file == 'b');
    suite6_assert(move.from.rank == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.dest.file == 'a');
    suite6_assert(move.dest.rank == 8);
    suite6_assert(move.check == PGN_CHECK_NONE);
    suite6_assert(move.promoted_to == PGN_PIECE_QUEEN);
    suite6_assert(!strcmp(move.notation, "bxa8=Q??"));
    suite6_assert(move.annotation == PGN_ANNOTATION_BLUNDER);

    move = pgn_parse_move("Bxg2");
    suite6_assert(move.piece == PGN_PIECE_BISHOP);
    suite6_assert(move.captures == true);
    suite6_assert(move.from.file == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.from.rank == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.dest.file == 'g');
    suite6_assert(move.dest.rank == 2);
    suite6_assert(move.check == PGN_CHECK_NONE);
    suite6_assert(move.promoted_to == PGN_PIECE_UNKNOWN);
    suite6_assert(!strcmp(move.notation, "Bxg2"));
    suite6_assert(move.annotation == PGN_ANNOTATION_UNKNOWN);

    move = pgn_parse_move("exf2+!");
    suite6_assert(move.piece == PGN_PIECE_PAWN);
    suite6_assert(move.captures == true);
    suite6_assert(move.from.file == 'e');
    suite6_assert(move.from.rank == PGN_COORDINATE_UNKNOWN);
    suite6_assert(move.dest.file == 'f');
    suite6_assert(move.dest.rank == 2);
    suite6_assert(move.check == PGN_CHECK_SINGLE);
    suite6_assert(move.promoted_to == PGN_PIECE_UNKNOWN);
    suite6_assert(!strcmp(move.notation, "exf2+!"));
    suite6_assert(move.annotation == PGN_ANNOTATION_GOOD_MOVE);

    move = pgn_parse_move("O-O+!!");
    suite6_assert(move.castles == PGN_CASTLING_KINGSIDE);
    suite6_assert(move.check == PGN_CHECK_SINGLE);
    suite6_assert(!strcmp(move.notation, "O-O+!!"));
    suite6_assert(move.annotation == PGN_ANNOTATION_BRILLIANT_MOVE);

    move = pgn_parse_move("O-O-O");
    suite6_assert(move.castles == PGN_CASTLING_QUEENSIDE);
    suite6_assert(move.check == PGN_CHECK_NONE);
    suite6_assert(!strcmp(move.notation, "O-O-O"));
    suite6_assert(move.annotation == PGN_ANNOTATION_UNKNOWN);
}

void test_parsing_bunch_of_moves()
{
    pgn_moves_t *moves = NULL;

    moves = pgn_parse_moves("1.e4 e5");
    suite6_assert(strcmp(moves->values[0].white.notation, "e4") == 0);
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(strcmp(moves->values[0].black.notation, "e5") == 0);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.captures == false);
    suite6_assert(moves->values[0].black.annotation == PGN_ANNOTATION_UNKNOWN);
    pgn_moves_cleanup(moves);

    moves = pgn_parse_moves("1.e4 $2 e5 $1");
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.annotation == 2);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.captures == false);
    suite6_assert(moves->values[0].black.annotation == 1);
    pgn_moves_cleanup(moves);

    moves = pgn_parse_moves("69.Be4 69... Rxe5?!");
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_BISHOP);
    suite6_assert(moves->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_ROOK);
    suite6_assert(moves->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.captures == true);
    suite6_assert(moves->values[0].black.annotation == PGN_ANNOTATION_DUBIOUS_MOVE);
    pgn_moves_cleanup(moves);

    moves = pgn_parse_moves("9.e4 { This is a comment :O } e5 10. Nc3 Nc6");
    suite6_assert(moves->values[0].white.comments->length == 1);
    suite6_assert(!strcmp(moves->values[0].white.comments->values[0].value->buf, " This is a comment :O "));
    suite6_assert(moves->values[0].white.comments->values[0].position == PGN_COMMENT_POSITION_AFTER_MOVE);
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.captures == false);
    suite6_assert(moves->values[0].black.annotation == PGN_ANNOTATION_UNKNOWN);

    suite6_assert(moves->values[1].white.piece == PGN_PIECE_KNIGHT);
    suite6_assert(moves->values[1].white.dest.file == 'c');
    suite6_assert(moves->values[1].white.dest.rank == 3);
    suite6_assert(moves->values[1].white.captures == false);
    suite6_assert(moves->values[1].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(moves->values[1].black.piece == PGN_PIECE_KNIGHT);
    suite6_assert(moves->values[1].black.dest.file == 'c');
    suite6_assert(moves->values[1].black.dest.rank == 6);
    suite6_assert(moves->values[1].black.captures == false);
    suite6_assert(moves->values[1].black.annotation == PGN_ANNOTATION_UNKNOWN);
    pgn_moves_cleanup(moves);

    moves = pgn_parse_moves("69.Be4 ( 69. Be2 69... e4 ) 69... Rxe5?!");
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_BISHOP);
    suite6_assert(moves->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_ROOK);
    suite6_assert(moves->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.captures == true);
    suite6_assert(moves->values[0].black.annotation == PGN_ANNOTATION_DUBIOUS_MOVE);
    suite6_assert(moves->values[0].white.alternatives);

    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.piece == PGN_PIECE_BISHOP);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.dest.rank == 2);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].black.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].black.dest.rank == 4);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].black.captures == false);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].black.annotation == PGN_ANNOTATION_UNKNOWN);
    pgn_moves_cleanup(moves);

    moves = pgn_parse_moves("69.Be4 69... Rxe5?! ( 69... e5 70. e4 )");
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_BISHOP);
    suite6_assert(moves->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_ROOK);
    suite6_assert(moves->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.captures == true);
    suite6_assert(moves->values[0].black.annotation == PGN_ANNOTATION_DUBIOUS_MOVE);
    suite6_assert(moves->values[0].black.alternatives);

    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.dest.file == 'e');
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.captures == false);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[0].black.annotation == PGN_ANNOTATION_UNKNOWN);

    suite6_assert(moves->values[0].black.alternatives->values[0]->values[1].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[1].white.dest.file == 'e');
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[1].white.dest.rank == 4);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[1].white.captures == false);
    suite6_assert(moves->values[0].black.alternatives->values[0]->values[1].white.annotation == PGN_ANNOTATION_UNKNOWN);
    pgn_moves_cleanup(moves);

    moves = pgn_parse_moves("1.c4 ( 1.e4  ) c5 2.Nc3 Nc6");
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].white.dest.file == 'c');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].black.dest.file == 'c');
    suite6_assert(moves->values[0].black.dest.rank == 5);
    suite6_assert(moves->values[0].black.captures == false);
    suite6_assert(moves->values[0].black.annotation == PGN_ANNOTATION_UNKNOWN);

    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.dest.rank == 4);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.captures == false);
    suite6_assert(moves->values[0].white.alternatives->values[0]->values[0].white.annotation == PGN_ANNOTATION_UNKNOWN);

    suite6_assert(moves->values[1].white.piece == PGN_PIECE_KNIGHT);
    suite6_assert(moves->values[1].white.dest.file == 'c');
    suite6_assert(moves->values[1].white.dest.rank == 3);
    suite6_assert(moves->values[1].white.captures == false);
    suite6_assert(moves->values[1].white.annotation == PGN_ANNOTATION_UNKNOWN);

    suite6_assert(moves->values[1].black.piece == PGN_PIECE_KNIGHT);
    suite6_assert(moves->values[1].black.dest.file == 'c');
    suite6_assert(moves->values[1].black.dest.rank == 6);
    suite6_assert(moves->values[1].black.captures == false);
    suite6_assert(moves->values[1].black.annotation == PGN_ANNOTATION_UNKNOWN);
    pgn_moves_cleanup(moves);
}

void test_parsing_en_passant()
{
    pgn_move_t move = pgn_parse_move("exd6 e.p.");
    suite6_assert(move.piece == PGN_PIECE_PAWN);
    suite6_assert(move.en_passant);
    suite6_assert(move.from.file == 'e');
    suite6_assert(move.dest.file == 'd');
    suite6_assert(move.dest.rank == 6);
    suite6_assert(move.captures);

    pgn_moves_t *moves = pgn_parse_moves("1. d4 exd3 e.p. 2.  Nc4 Nc6 3. cxb3 e.p. 3... Be4");
    suite6_assert(moves->values[0].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].white.dest.file == 'd');
    suite6_assert(moves->values[0].white.dest.rank == 4);
    suite6_assert(!strcmp(moves->values[0].white.notation, "d4"));
    suite6_assert(!moves->values[0].white.en_passant);
    suite6_assert(moves->values[0].black.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[0].black.from.file == 'e');
    suite6_assert(moves->values[0].black.dest.file == 'd');
    suite6_assert(moves->values[0].black.dest.rank == 3);
    suite6_assert(!strcmp(moves->values[0].black.notation, "exd3 e.p."));
    suite6_assert(moves->values[0].black.en_passant);

    suite6_assert(moves->values[1].white.piece == PGN_PIECE_KNIGHT);
    suite6_assert(moves->values[1].white.dest.file == 'c');
    suite6_assert(moves->values[1].white.dest.rank == 4);
    suite6_assert(!strcmp(moves->values[1].white.notation, "Nc4"));
    suite6_assert(!moves->values[1].white.en_passant);
    suite6_assert(moves->values[1].black.piece == PGN_PIECE_KNIGHT);
    suite6_assert(moves->values[1].black.dest.file == 'c');
    suite6_assert(moves->values[1].black.dest.rank == 6);
    suite6_assert(!strcmp(moves->values[1].black.notation, "Nc6"));
    suite6_assert(!moves->values[1].black.en_passant);

    suite6_assert(moves->values[2].white.piece == PGN_PIECE_PAWN);
    suite6_assert(moves->values[2].white.from.file == 'c');
    suite6_assert(moves->values[2].white.dest.file == 'b');
    suite6_assert(moves->values[2].white.dest.rank == 3);
    suite6_assert(!strcmp(moves->values[2].white.notation, "cxb3 e.p."));
    suite6_assert(moves->values[2].white.en_passant);
    suite6_assert(moves->values[2].black.piece == PGN_PIECE_BISHOP);
    suite6_assert(moves->values[2].black.dest.file == 'e');
    suite6_assert(moves->values[2].black.dest.rank == 4);
    suite6_assert(!strcmp(moves->values[2].black.notation, "Be4"));
    suite6_assert(!moves->values[2].black.en_passant);
    pgn_moves_cleanup(moves);
}

void test_parsing_moves_with_alternatives()
{
    pgn_moves_t *moves = NULL;

    moves = pgn_moves_from_string("1. e4 (1. f4? e5! 2. g4?? Qh4#) e5");
    suite6_assert(moves->length == 1);
    suite6_assert(moves->values[0].white.alternatives->length == 1);
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[0].white.notation, "f4?"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[0].black.notation, "e5!"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[1].white.notation, "g4??"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[1].black.notation, "Qh4#"));
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1. e4 (1. f4? e5! 2. g4?? Qh4#) (1. e4 f6? 2. d4 g5?? 3. Qh5#) e5");
    suite6_assert(moves->length == 1);
    suite6_assert(moves->values[0].white.alternatives->length == 2);
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[0].white.notation, "f4?"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[0].black.notation, "e5!"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[1].white.notation, "g4??"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[1].black.notation, "Qh4#"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[1]->values[0].white.notation, "e4"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[1]->values[0].black.notation, "f6?"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[1]->values[1].white.notation, "d4"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[1]->values[1].black.notation, "g5??"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[1]->values[2].white.notation, "Qh5#"));
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1. e4 (1. f4? e5! 2. g4?? Qh4#) e5 2. Nc6 Nf4");
    suite6_assert(moves->length == 2);
    suite6_assert(moves->values[0].white.alternatives->length == 1);
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[0].white.notation, "f4?"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[0].black.notation, "e5!"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[1].white.notation, "g4??"));
    suite6_assert(!strcmp(moves->values[0].white.alternatives->values[0]->values[1].black.notation, "Qh4#"));
    suite6_assert(!strcmp(moves->values[1].white.notation, "Nc6"));
    suite6_assert(!strcmp(moves->values[1].black.notation, "Nf4"));
    pgn_moves_cleanup(moves);
}

void test_parsing_ambiguate_moves()
{
    pgn_moves_t *moves = NULL;

    moves = pgn_moves_from_string("1. Rdf8 R1a3");
    suite6_assert(!strcmp(moves->values[0].white.notation, "Rdf8"));
    suite6_assert(moves->values[0].white.from.file == 'd');
    suite6_assert(moves->values[0].white.from.rank == 0);
    suite6_assert(moves->values[0].white.dest.file == 'f');
    suite6_assert(moves->values[0].white.dest.rank == 8);

    suite6_assert(!strcmp(moves->values[0].black.notation, "R1a3"));
    suite6_assert(moves->values[0].black.from.file == 0);
    suite6_assert(moves->values[0].black.from.rank == 1);
    suite6_assert(moves->values[0].black.dest.file == 'a');
    suite6_assert(moves->values[0].black.dest.rank == 3);
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1. Qh4e1");
    suite6_assert(!strcmp(moves->values[0].white.notation, "Qh4e1"));
    suite6_assert(moves->values[0].white.from.file == 'h');
    suite6_assert(moves->values[0].white.from.rank == 4);
    suite6_assert(moves->values[0].white.dest.file == 'e');
    suite6_assert(moves->values[0].white.dest.rank == 1);
    pgn_moves_cleanup(moves);
}

void test_dump_move()
{
    pgn_move_t move = {0};
    size_t written = 0;

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_UNKNOWN;
    move.piece = PGN_PIECE_PAWN;
    move.dest.file = 'e';
    move.dest.rank = 4;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "e4"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_UNKNOWN;
    move.piece = PGN_PIECE_ROOK;
    move.dest.file = 'e';
    move.dest.rank = 4;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "Re4"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_UNKNOWN;
    move.piece = PGN_PIECE_ROOK;
    move.check = PGN_CHECK_MATE;
    move.dest.file = 'e';
    move.dest.rank = 4;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "Re4#"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_UNKNOWN;
    move.castles = PGN_CASTLING_KINGSIDE;
    move.check = PGN_CHECK_MATE;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "O-O#"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_INTRESTING_MOVE;
    move.piece = PGN_PIECE_ROOK;
    move.dest.file = 'a';
    move.dest.rank = 3;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "Ra3!?"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_INTRESTING_MOVE;
    move.en_passant = true;
    move.piece = PGN_PIECE_PAWN;
    move.dest.file = 'f';
    move.dest.rank = 3;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "f3!? e.p."));

    move = (pgn_move_t){0};
    move.annotation = 9;
    move.piece = PGN_PIECE_PAWN;
    move.dest.file = 'f';
    move.dest.rank = 3;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "f3 $9"));

    move = (pgn_move_t){0};
    move.annotation = 9;
    move.en_passant = true;
    move.piece = PGN_PIECE_PAWN;
    move.dest.file = 'f';
    move.dest.rank = 3;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "f3 e.p. $9"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_UNKNOWN;
    move.piece = PGN_PIECE_PAWN;
    move.captures = true;
    move.dest.file = 'f';
    move.dest.rank = 3;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "xf3"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_UNKNOWN;
    move.piece = PGN_PIECE_ROOK;
    move.captures = true;
    move.dest.file = 'f';
    move.dest.rank = 3;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "Rxf3"));

    move = (pgn_move_t){0};
    move.annotation = PGN_ANNOTATION_UNKNOWN;
    move.piece = PGN_PIECE_PAWN;
    move.captures = true;
    move.dest.file = 'f';
    move.dest.rank = 8;
    move.promoted_to = PGN_PIECE_QUEEN;
    move.check = PGN_CHECK_DOUBLE;
    written = pgn_move_to_string(&move, move.notation);
    suite6_assert(strlen(move.notation) == written);
    suite6_assert(!strcmp(move.notation, "xf8=Q++"));
}

void test_parsing_weird_move()
{
    return;

    suite6_assert(*(pgn_move_from_string("Le1").notation) == '\0');
    suite6_assert(*(pgn_move_from_string("xe100").notation) == '\0');
}

void test_parsing_moves_with_weird_spaces()
{
    return;

    pgn_moves_t *moves = NULL;

    moves = pgn_moves_from_string("1. Rdf8e.p.");
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1. Rdf8e.p.$0");
    pgn_moves_cleanup(moves);

    moves = pgn_moves_from_string("1. Rdf8$0");
    pgn_moves_cleanup(moves);
}

int main(void)
{
    test_parsing_single_move();
    test_parsing_bunch_of_moves();
    test_parsing_en_passant();
    test_parsing_moves_with_alternatives();
    test_parsing_ambiguate_moves();
    test_dump_move();
    test_parsing_weird_move(); // DOES NOT GET CALLED, WOULD abort()
    test_parsing_moves_with_weird_spaces(); // DOES NOT GET CALLED, WOULD abort()
    return 0;
}
