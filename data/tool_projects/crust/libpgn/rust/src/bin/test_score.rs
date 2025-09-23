use libpgn::score::PgnScore;

#[test]
fn test_parsing_score() {
    assert_eq!(PgnScore::from("1/2-1/2"), PgnScore::Draw);
    assert_eq!(PgnScore::from("1-0"), PgnScore::WhiteWon);
    assert_eq!(PgnScore::from("0-1"), PgnScore::BlackWon);
    assert_eq!(PgnScore::from("0-0"), PgnScore::Forfeit);
    assert_eq!(PgnScore::from("0-1/2"), PgnScore::WhiteForfeit);
    assert_eq!(PgnScore::from("1/2-0"), PgnScore::BlackForfeit);
}

#[test]
fn test_parsing_invalid_score() {
    assert_eq!(PgnScore::from("1-1/2"), PgnScore::Unknown);
    assert_eq!(PgnScore::from("1/2-1"), PgnScore::Unknown);
    assert_eq!(PgnScore::from("2-0"), PgnScore::Unknown);
    assert_eq!(PgnScore::from("0-2"), PgnScore::Unknown);
    assert_eq!(PgnScore::from(""), PgnScore::Unknown);
    assert_eq!(PgnScore::from("-"), PgnScore::Unknown);
    assert_eq!(PgnScore::from("0-"), PgnScore::Unknown);
    assert_eq!(PgnScore::from("-0"), PgnScore::Unknown);
}
fn main(){}