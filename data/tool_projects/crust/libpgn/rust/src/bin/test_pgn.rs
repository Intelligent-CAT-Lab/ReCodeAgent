use libpgn::annotation::PgnAnnotation;
use libpgn::metadata::PgnMetadata;
use libpgn::moves::PgnMoves;
use libpgn::pgn::Pgn;
use libpgn::piece::PgnPiece;
use libpgn::score::PgnScore;

#[test]
fn test_parsing_pgn() {
    let mut pgn = Pgn::new();
    pgn.parse(
        "[Event \"Ch City (open)\"]\n\
        [Site \"Frankfurt (Germany)\"]\n\
        \n\
        1.e4 e5\n\
        2.Nc3 Nc6\n\
        3. g3 0-1",
    );

    assert_eq!(
        pgn.metadata.as_ref().unwrap().get("Event"),
        Some("Ch City (open)")
    );
    assert_eq!(
        pgn.metadata.as_ref().unwrap().get("Site"),
        Some("Frankfurt (Germany)")
    );

    assert_eq!(pgn.moves.as_ref().unwrap().values[0].white.notation, "e4");
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].white.piece,
        PgnPiece::Pawn
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].white.dest.file,
        Some('e')
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].white.dest.rank,
        Some(4)
    );
    assert_eq!(pgn.moves.as_ref().unwrap().values[0].white.captures, false);
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].white.annotation,
        PgnAnnotation::Unknown
    );
    assert_eq!(pgn.moves.as_ref().unwrap().values[0].black.notation, "e5");
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].black.piece,
        PgnPiece::Pawn
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].black.dest.file,
        Some('e')
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].black.dest.rank,
        Some(5)
    );
    assert_eq!(pgn.moves.as_ref().unwrap().values[0].black.captures, false);
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[0].black.annotation,
        PgnAnnotation::Unknown
    );

    assert_eq!(pgn.moves.as_ref().unwrap().values[1].white.notation, "Nc3");
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].white.piece,
        PgnPiece::Knight
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].white.dest.file,
        Some('c')
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].white.dest.rank,
        Some(3)
    );
    assert_eq!(pgn.moves.as_ref().unwrap().values[1].white.captures, false);
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].white.annotation,
        PgnAnnotation::Unknown
    );
    assert_eq!(pgn.moves.as_ref().unwrap().values[1].black.notation, "Nc6");
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].black.piece,
        PgnPiece::Knight
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].black.dest.file,
        Some('c')
    );
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].black.dest.rank,
        Some(6)
    );
    assert_eq!(pgn.moves.as_ref().unwrap().values[1].black.captures, false);
    assert_eq!(
        pgn.moves.as_ref().unwrap().values[1].black.annotation,
        PgnAnnotation::Unknown
    );

    assert_eq!(pgn.score, PgnScore::BlackWon);
}
fn main(){}