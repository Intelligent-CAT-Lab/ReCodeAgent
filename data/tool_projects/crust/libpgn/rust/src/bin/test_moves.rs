use libpgn::annotation::PgnAnnotation;
use libpgn::check::PgnCheck;
use libpgn::comments::PgnCommentPosition;
use libpgn::coordinate::{PgnCoordinate, UNKNOWN};
use libpgn::moves::{PgnMove, PgnMoves, PGN_CASTLING_KINGSIDE, PGN_CASTLING_QUEENSIDE};
use libpgn::piece::PgnPiece;

#[test]
fn test_parsing_single_move() {
    let mut mv: PgnMove;

    mv = PgnMove::from("e4??");
    assert_eq!(mv.piece, PgnPiece::Pawn);
    assert_eq!(mv.captures, false);
    assert_eq!(mv.from.file, None);
    assert_eq!(mv.from.rank, None);
    assert_eq!(mv.dest.file, Some('e'));
    assert_eq!(mv.dest.rank, Some(4));
    assert_eq!(mv.check, PgnCheck::None);
    assert_eq!(mv.notation, "e4??");
    assert_eq!(mv.annotation, PgnAnnotation::Blunder);

    mv = PgnMove::from("Nb6d5+");
    assert_eq!(mv.piece, PgnPiece::Knight);
    assert_eq!(mv.captures, false);
    assert_eq!(mv.from.file, Some('b'));
    assert_eq!(mv.from.rank, Some(6));
    assert_eq!(mv.dest.file, Some('d'));
    assert_eq!(mv.dest.rank, Some(5));
    assert_eq!(mv.check, PgnCheck::Single);
    assert_eq!(mv.notation, "Nb6d5+");
    assert_eq!(mv.annotation, PgnAnnotation::Unknown);

    mv = PgnMove::from("Qf1?!");
    assert_eq!(mv.piece, PgnPiece::Queen);
    assert_eq!(mv.captures, false);
    assert_eq!(mv.from.file, None);
    assert_eq!(mv.from.rank, None);
    assert_eq!(mv.dest.file, Some('f'));
    assert_eq!(mv.dest.rank, Some(1));
    assert_eq!(mv.check, PgnCheck::None);
    assert_eq!(mv.notation, "Qf1?!");
    assert_eq!(mv.annotation, PgnAnnotation::DubiousMove);

    mv = PgnMove::from("bxa8=Q??");
    assert_eq!(mv.piece, PgnPiece::Pawn);
    assert_eq!(mv.captures, true);
    assert_eq!(mv.from.file, Some('b'));
    assert_eq!(mv.from.rank, None);
    assert_eq!(mv.dest.file, Some('a'));
    assert_eq!(mv.dest.rank, Some(8));
    assert_eq!(mv.check, PgnCheck::None);
    assert_eq!(mv.promoted_to, PgnPiece::Queen);
    assert_eq!(mv.notation, "bxa8=Q??");
    assert_eq!(mv.annotation, PgnAnnotation::Blunder);

    mv = PgnMove::from("Bxg2");
    assert_eq!(mv.piece, PgnPiece::Bishop);
    assert_eq!(mv.captures, true);
    assert_eq!(mv.from.file, None);
    assert_eq!(mv.from.rank, None);
    assert_eq!(mv.dest.file, Some('g'));
    assert_eq!(mv.dest.rank, Some(2));
    assert_eq!(mv.check, PgnCheck::None);
    assert_eq!(mv.promoted_to, PgnPiece::Unknown);
    assert_eq!(mv.notation, "Bxg2");
    assert_eq!(mv.annotation, PgnAnnotation::Unknown);

    mv = PgnMove::from("exf2+!");
    assert_eq!(mv.piece, PgnPiece::Pawn);
    assert_eq!(mv.captures, true);
    assert_eq!(mv.from.file, Some('e'));
    assert_eq!(mv.from.rank, None);
    assert_eq!(mv.dest.file, Some('f'));
    assert_eq!(mv.dest.rank, Some(2));
    assert_eq!(mv.check, PgnCheck::Single);
    assert_eq!(mv.promoted_to, PgnPiece::Unknown);
    assert_eq!(mv.notation, "exf2+!");
    assert_eq!(mv.annotation, PgnAnnotation::GoodMove);

    mv = PgnMove::from("O-O+!!");
    assert_eq!(mv.castles, PGN_CASTLING_KINGSIDE);
    assert_eq!(mv.check, PgnCheck::Single);
    assert_eq!(mv.notation, "O-O+!!");
    assert_eq!(mv.annotation, PgnAnnotation::BrilliantMove);

    mv = PgnMove::from("O-O-O");
    assert_eq!(mv.castles, PGN_CASTLING_QUEENSIDE);
    assert_eq!(mv.check, PgnCheck::None);
    assert_eq!(mv.notation, "O-O-O");
    assert_eq!(mv.annotation, PgnAnnotation::Unknown);
}

#[test]
fn test_parsing_bunch_of_moves() {
    let mut moves: PgnMoves;

    moves = PgnMoves::from("1.e4 e5");
    assert_eq!(moves.values[0].white.notation, "e4");
    assert_eq!(moves.values[0].white.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].white.dest.file, Some('e'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.captures, false);
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Unknown);
    assert_eq!(moves.values[0].black.notation, "e5");
    assert_eq!(moves.values[0].black.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].black.dest.file, Some('e'));
    assert_eq!(moves.values[0].black.dest.rank, Some(5));
    assert_eq!(moves.values[0].black.captures, false);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::Unknown);

    moves = PgnMoves::from("1.e4 $2 e5 $1");
    assert_eq!(moves.values[0].white.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].white.dest.file, Some('e'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.captures, false);
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::GoodMove);
    assert_eq!(moves.values[0].black.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].black.dest.file, Some('e'));
    assert_eq!(moves.values[0].black.dest.rank, Some(5));
    assert_eq!(moves.values[0].black.captures, false);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::Mistake);

    moves = PgnMoves::from("69.Be4 69... Rxe5?!");
    assert_eq!(moves.values[0].white.piece, PgnPiece::Bishop);
    assert_eq!(moves.values[0].white.dest.file, Some('e'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.captures, false);
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Unknown);
    assert_eq!(moves.values[0].black.piece, PgnPiece::Rook);
    assert_eq!(moves.values[0].black.dest.file, Some('e'));
    assert_eq!(moves.values[0].black.dest.rank, Some(5));
    assert_eq!(moves.values[0].black.captures, true);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::DubiousMove);

    moves = PgnMoves::from("9.e4 { This is a comment :O } e5 10. Nc3 Nc6");
    assert_eq!(
        moves.values[0]
            .white
            .comments
            .as_ref()
            .unwrap()
            .values
            .len(),
        1
    );
    assert_eq!(
        moves.values[0].white.comments.as_ref().unwrap().values[0].value,
        " This is a comment :O "
    );
    assert_eq!(
        moves.values[0].white.comments.as_ref().unwrap().values[0].position,
        PgnCommentPosition::AfterMove
    );
    assert_eq!(moves.values[0].white.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].white.dest.file, Some('e'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.captures, false);
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Unknown);
    assert_eq!(moves.values[0].black.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].black.dest.file, Some('e'));
    assert_eq!(moves.values[0].black.dest.rank, Some(5));
    assert_eq!(moves.values[0].black.captures, false);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::Unknown);

    assert_eq!(moves.values[1].white.piece, PgnPiece::Knight);
    assert_eq!(moves.values[1].white.dest.file, Some('c'));
    assert_eq!(moves.values[1].white.dest.rank, Some(3));
    assert_eq!(moves.values[1].white.captures, false);
    assert_eq!(moves.values[1].white.annotation, PgnAnnotation::Unknown);
    assert_eq!(moves.values[1].black.piece, PgnPiece::Knight);
    assert_eq!(moves.values[1].black.dest.file, Some('c'));
    assert_eq!(moves.values[1].black.dest.rank, Some(6));
    assert_eq!(moves.values[1].black.captures, false);
    assert_eq!(moves.values[1].black.annotation, PgnAnnotation::Unknown);

    moves = PgnMoves::from("69.Be4 ( 69. Be2 69... e4 ) 69... Rxe5?!");
    assert_eq!(moves.values[0].white.piece, PgnPiece::Bishop);
    assert_eq!(moves.values[0].white.dest.file, Some('e'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.captures, false);
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Unknown);
    assert_eq!(moves.values[0].black.piece, PgnPiece::Rook);
    assert_eq!(moves.values[0].black.dest.file, Some('e'));
    assert_eq!(moves.values[0].black.dest.rank, Some(5));
    assert_eq!(moves.values[0].black.captures, true);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::DubiousMove);
    assert!(moves.values[0].white.alternatives.is_some());

    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .piece,
        PgnPiece::Bishop
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .dest
            .file,
        Some('e')
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .dest
            .rank,
        Some(2)
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .captures,
        false
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .annotation,
        PgnAnnotation::Unknown
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .piece,
        PgnPiece::Pawn
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .dest
            .file,
        Some('e')
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .dest
            .rank,
        Some(4)
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .captures,
        false
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .annotation,
        PgnAnnotation::Unknown
    );

    moves = PgnMoves::from("69.Be4 69... Rxe5?! ( 69... e5 70. e4 )");
    assert_eq!(moves.values[0].white.piece, PgnPiece::Bishop);
    assert_eq!(moves.values[0].white.dest.file, Some('e'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.captures, false);
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Unknown);
    assert_eq!(moves.values[0].black.piece, PgnPiece::Rook);
    assert_eq!(moves.values[0].black.dest.file, Some('e'));
    assert_eq!(moves.values[0].black.dest.rank, Some(5));
    assert_eq!(moves.values[0].black.captures, true);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::DubiousMove);
    assert!(moves.values[0].black.alternatives.is_some());

    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .piece,
        PgnPiece::Pawn
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .dest
            .file,
        Some('e')
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .dest
            .rank,
        Some(5)
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .captures,
        false
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .annotation,
        PgnAnnotation::Unknown
    );

    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .piece,
        PgnPiece::Pawn
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .dest
            .file,
        Some('e')
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .dest
            .rank,
        Some(4)
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .captures,
        false
    );
    assert_eq!(
        moves.values[0].black.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .annotation,
        PgnAnnotation::Unknown
    );

    moves = PgnMoves::from("1.c4 ( 1.e4  ) c5 2.Nc3 Nc6");
    assert_eq!(moves.values[0].white.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].white.dest.file, Some('c'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.captures, false);
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Unknown);
    assert_eq!(moves.values[0].black.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].black.dest.file, Some('c'));
    assert_eq!(moves.values[0].black.dest.rank, Some(5));
    assert_eq!(moves.values[0].black.captures, false);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::Unknown);

    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .piece,
        PgnPiece::Pawn
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .dest
            .file,
        Some('e')
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .dest
            .rank,
        Some(4)
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .captures,
        false
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .annotation,
        PgnAnnotation::Unknown
    );

    assert_eq!(moves.values[1].white.piece, PgnPiece::Knight);
    assert_eq!(moves.values[1].white.dest.file, Some('c'));
    assert_eq!(moves.values[1].white.dest.rank, Some(3));
    assert_eq!(moves.values[1].white.captures, false);
    assert_eq!(moves.values[1].white.annotation, PgnAnnotation::Unknown);

    assert_eq!(moves.values[1].black.piece, PgnPiece::Knight);
    assert_eq!(moves.values[1].black.dest.file, Some('c'));
    assert_eq!(moves.values[1].black.dest.rank, Some(6));
    assert_eq!(moves.values[1].black.captures, false);
    assert_eq!(moves.values[1].black.annotation, PgnAnnotation::Unknown);
}

#[test]
fn test_parsing_en_passant() {
    let mv = PgnMove::from("exd6 e.p.");
    assert_eq!(mv.piece, PgnPiece::Pawn);
    assert!(mv.en_passant);
    assert_eq!(mv.from.file, Some('e'));
    assert_eq!(mv.dest.file, Some('d'));
    assert_eq!(mv.dest.rank, Some(6));
    assert!(mv.captures);

    let moves = PgnMoves::from("1. d4 exd3 e.p. 2. Nc4 Nc6 3. cxb3 e.p. 3... Be4");
    assert_eq!(moves.values[0].white.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].white.dest.file, Some('d'));
    assert_eq!(moves.values[0].white.dest.rank, Some(4));
    assert_eq!(moves.values[0].white.notation, "d4");
    assert!(!moves.values[0].white.en_passant);
    assert_eq!(moves.values[0].black.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[0].black.from.file, Some('e'));
    assert_eq!(moves.values[0].black.dest.file, Some('d'));
    assert_eq!(moves.values[0].black.dest.rank, Some(3));
    assert_eq!(moves.values[0].black.notation, "exd3 e.p.");
    assert!(moves.values[0].black.en_passant);

    assert_eq!(moves.values[1].white.piece, PgnPiece::Knight);
    assert_eq!(moves.values[1].white.dest.file, Some('c'));
    assert_eq!(moves.values[1].white.dest.rank, Some(4));
    assert_eq!(moves.values[1].white.notation, "Nc4");
    assert!(!moves.values[1].white.en_passant);
    assert_eq!(moves.values[1].black.piece, PgnPiece::Knight);
    assert_eq!(moves.values[1].black.dest.file, Some('c'));
    assert_eq!(moves.values[1].black.dest.rank, Some(6));
    assert_eq!(moves.values[1].black.notation, "Nc6");
    assert!(!moves.values[1].black.en_passant);

    assert_eq!(moves.values[2].white.piece, PgnPiece::Pawn);
    assert_eq!(moves.values[2].white.from.file, Some('c'));
    assert_eq!(moves.values[2].white.dest.file, Some('b'));
    assert_eq!(moves.values[2].white.dest.rank, Some(3));
    assert_eq!(moves.values[2].white.notation, "cxb3 e.p.");
    assert!(moves.values[2].white.en_passant);
    assert_eq!(moves.values[2].black.piece, PgnPiece::Bishop);
    assert_eq!(moves.values[2].black.dest.file, Some('e'));
    assert_eq!(moves.values[2].black.dest.rank, Some(4));
    assert_eq!(moves.values[2].black.notation, "Be4");
    assert!(!moves.values[2].black.en_passant);
}

#[test]
fn test_parsing_moves_with_alternatives() {
    let mut moves: PgnMoves;

    moves = PgnMoves::from("1. e4 (1. f4? e5! 2. g4?? Qh4#) e5");
    assert_eq!(moves.values.len(), 1);
    assert_eq!(
        moves.values[0]
            .white
            .alternatives
            .as_ref()
            .unwrap()
            .values
            .len(),
        1
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .notation,
        "f4?"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .notation,
        "e5!"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .notation,
        "g4??"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[1]
            .black
            .notation,
        "Qh4#"
    );

    moves = PgnMoves::from("1. e4 (1. f4? e5! 2. g4?? Qh4#) (1. e4 f6? 2. d4 g5?? 3. Qh5#) e5");
    assert_eq!(moves.values.len(), 1);
    assert_eq!(
        moves.values[0]
            .white
            .alternatives
            .as_ref()
            .unwrap()
            .values
            .len(),
        2
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .notation,
        "f4?"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .notation,
        "e5!"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .notation,
        "g4??"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[1]
            .black
            .notation,
        "Qh4#"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[1].values[0]
            .white
            .notation,
        "e4"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[1].values[0]
            .black
            .notation,
        "f6?"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[1].values[1]
            .white
            .notation,
        "d4"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[1].values[1]
            .black
            .notation,
        "g5??"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[1].values[2]
            .white
            .notation,
        "Qh5#"
    );

    moves = PgnMoves::from("1. e4 (1. f4? e5! 2. g4?? Qh4#) e5 2. Nc6 Nf4");
    assert_eq!(moves.values.len(), 2);
    assert_eq!(
        moves.values[0]
            .white
            .alternatives
            .as_ref()
            .unwrap()
            .values
            .len(),
        1
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .white
            .notation,
        "f4?"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[0]
            .black
            .notation,
        "e5!"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[1]
            .white
            .notation,
        "g4??"
    );
    assert_eq!(
        moves.values[0].white.alternatives.as_ref().unwrap().values[0].values[1]
            .black
            .notation,
        "Qh4#"
    );
    assert_eq!(moves.values[1].white.notation, "Nc6");
    assert_eq!(moves.values[1].black.notation, "Nf4");
}

#[test]
fn test_parsing_ambiguate_moves() {
    let mut moves: PgnMoves;

    moves = PgnMoves::from("1. Rdf8 R1a3");
    assert_eq!(moves.values[0].white.notation, "Rdf8");
    assert_eq!(moves.values[0].white.from.file, Some('d'));
    assert_eq!(moves.values[0].white.from.rank, None);
    assert_eq!(moves.values[0].white.dest.file, Some('f'));
    assert_eq!(moves.values[0].white.dest.rank, Some(8));

    assert_eq!(moves.values[0].black.notation, "R1a3");
    assert_eq!(moves.values[0].black.from.file, None);
    assert_eq!(moves.values[0].black.from.rank, Some(1));
    assert_eq!(moves.values[0].black.dest.file, Some('a'));
    assert_eq!(moves.values[0].black.dest.rank, Some(3));

    moves = PgnMoves::from("1. Qh4e1");
    assert_eq!(moves.values[0].white.notation, "Qh4e1");
    assert_eq!(moves.values[0].white.from.file, Some('h'));
    assert_eq!(moves.values[0].white.from.rank, Some(4));
    assert_eq!(moves.values[0].white.dest.file, Some('e'));
    assert_eq!(moves.values[0].white.dest.rank, Some(1));
}

#[test]
fn test_dump_move() {
    let mut mv: PgnMove;
    let mut written: usize;

    mv = PgnMove {
        annotation: PgnAnnotation::Unknown,
        piece: PgnPiece::Pawn,
        dest: PgnCoordinate {
            file: Some('e'),
            rank: Some(4),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "e4");

    mv = PgnMove {
        annotation: PgnAnnotation::Unknown,
        piece: PgnPiece::Rook,
        dest: PgnCoordinate {
            file: Some('e'),
            rank: Some(4),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "Re4");

    mv = PgnMove {
        annotation: PgnAnnotation::Unknown,
        piece: PgnPiece::Rook,
        check: PgnCheck::Mate,
        dest: PgnCoordinate {
            file: Some('e'),
            rank: Some(4),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "Re4#");

    mv = PgnMove {
        annotation: PgnAnnotation::Unknown,
        castles: PGN_CASTLING_KINGSIDE,
        check: PgnCheck::Mate,
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "O-O#");

    mv = PgnMove {
        annotation: PgnAnnotation::InterestingMove,
        piece: PgnPiece::Rook,
        dest: PgnCoordinate {
            file: Some('a'),
            rank: Some(3),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "Ra3!?");

    mv = PgnMove {
        annotation: PgnAnnotation::InterestingMove,
        en_passant: true,
        piece: PgnPiece::Pawn,
        dest: PgnCoordinate {
            file: Some('f'),
            rank: Some(3),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "f3!? e.p.");

    mv = PgnMove {
        annotation: PgnAnnotation::from(9),
        piece: PgnPiece::Pawn,
        dest: PgnCoordinate {
            file: Some('f'),
            rank: Some(3),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "f3 $9");

    mv = PgnMove {
        annotation: PgnAnnotation::from(9),
        en_passant: true,
        piece: PgnPiece::Pawn,
        dest: PgnCoordinate {
            file: Some('f'),
            rank: Some(3),
        },
        ..Default::default()
    };
    assert_eq!(mv.notation, "f3 e.p. $9");

    mv = PgnMove {
        annotation: PgnAnnotation::Unknown,
        piece: PgnPiece::Pawn,
        captures: true,
        dest: PgnCoordinate {
            file: Some('f'),
            rank: Some(3),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "xf3");

    mv = PgnMove {
        annotation: PgnAnnotation::Unknown,
        piece: PgnPiece::Rook,
        captures: true,
        dest: PgnCoordinate {
            file: Some('f'),
            rank: Some(3),
        },
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "Rxf3");

    mv = PgnMove {
        annotation: PgnAnnotation::Unknown,
        piece: PgnPiece::Pawn,
        captures: true,
        dest: PgnCoordinate {
            file: Some('f'),
            rank: Some(8),
        },
        promoted_to: PgnPiece::Queen,
        check: PgnCheck::Double,
        ..Default::default()
    };
    assert_eq!(mv.to_string(), "xf8=Q++");
}

#[test]
fn test_parsing_weird_move() {
    // NOTE: the original test case has a return statement at the beginning
    // thus the test is not executed
    // assert_eq!(PgnMove::from("Le1").notation, "");
    // assert_eq!(PgnMove::from("xe100").notation, "");
}

#[test]
fn test_parsing_moves_with_weird_spaces() {
    // NOTE: the original test case has a return statement at the beginning
    // thus the test is not executed

    /*
    let mut moves: PgnMoves;

    moves = PgnMoves::from("1. Rdf8e.p.");

    moves = PgnMoves::from("1. Rdf8e.p.$0");

    moves = PgnMoves::from("1. Rdf8$0");
     */
}
fn main(){}