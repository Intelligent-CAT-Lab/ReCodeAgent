use libpgn::annotation::PgnAnnotation;
use libpgn::moves::PgnMoves;

#[test]
fn test_parsing_moves_with_annotation() {
    let moves = PgnMoves::from("1. a4?? a5!");
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Blunder);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::GoodMove);
    assert_eq!(moves.values[0].white.notation, "a4??");
    assert_eq!(moves.values[0].black.notation, "a5!");

    let moves = PgnMoves::from("1. a4!! a5?!");
    assert_eq!(
        moves.values[0].white.annotation,
        PgnAnnotation::BrilliantMove
    );
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::DubiousMove);
    assert_eq!(moves.values[0].white.notation, "a4!!");
    assert_eq!(moves.values[0].black.notation, "a5?!");
}

#[test]
fn test_parsing_moves_with_nag_annotation() {
    let moves = PgnMoves::from("1. a4 $4 a5 $1");
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Blunder);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::GoodMove);
    assert_eq!(moves.values[0].white.notation, "a4 $4");
    assert_eq!(moves.values[0].black.notation, "a5 $1");

    let moves = PgnMoves::from("1. a4 $69 a5 $420");
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Unknown); // Assuming 69 is not defined
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::Unknown); // Assuming 420 is not defined
    assert_eq!(moves.values[0].white.notation, "a4 $69");
    assert_eq!(moves.values[0].black.notation, "a5 $420");

    let moves = PgnMoves::from("6. Nce2 $2 e5 $0 $19 {}");
    assert_eq!(moves.values[0].white.annotation, PgnAnnotation::Mistake);
    assert_eq!(moves.values[0].black.annotation, PgnAnnotation::Unknown); // Assuming 19 is not defined
    assert_eq!(moves.values[0].white.notation, "Nce2 $2");
    assert_eq!(moves.values[0].black.notation, "e5 $0 $19");
}
fn main(){}