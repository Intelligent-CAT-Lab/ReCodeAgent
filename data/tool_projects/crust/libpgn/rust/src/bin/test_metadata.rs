use libpgn::metadata::PgnMetadata;
use libpgn::pgn::Pgn;
use libpgn::score::{PgnScore, PgnScore::Draw};

#[test]
fn test_parsing_metadata() {
    let metadata = Pgn::parse_metadata(
        "[Event \"F/S Return Match\"]\n\
        [Site \"Belgrade, Serbia JUG\"]\n\
        [Date \"1992.11.04\"]\n\
        [Round \"29\"]\n\
        [White \"Fischer, Robert J.\"]\n\
        [Black \"Spassky, Boris V.\"]\n\
        [Result \"1/2-1/2\"]\n",
    );

    assert_eq!(metadata.get("Event").unwrap(), "F/S Return Match");
    assert_eq!(metadata.get("Site").unwrap(), "Belgrade, Serbia JUG");
    assert_eq!(metadata.get("Date").unwrap(), "1992.11.04");
    assert_eq!(metadata.get("Round").unwrap(), "29");
    assert_eq!(metadata.get("White").unwrap(), "Fischer, Robert J.");
    assert_eq!(metadata.get("Black").unwrap(), "Spassky, Boris V.");
    assert_eq!(metadata.get("Result").unwrap(), "1/2-1/2");
    assert_eq!(PgnScore::from(metadata.get("Result").unwrap()), Draw);
}
fn main(){}