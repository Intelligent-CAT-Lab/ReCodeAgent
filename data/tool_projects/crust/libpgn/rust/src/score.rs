#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PgnScore {
    Unknown = 0,
    Ongoing,
    Draw,
    WhiteWon,
    BlackWon,
    Forfeit,
    WhiteForfeit,
    BlackForfeit,
}
impl PgnScore {
    /// Parses a PGN score from a string, tracking characters consumed (`__pgn_score_from_string`)
    fn from_string_with_consumption(s: &str, consumed: &mut usize) -> Self {
        unimplemented!()
    }
}
impl From<&str> for PgnScore {
    fn from(s: &str) -> Self {
        unimplemented!()
    }
}
impl std::fmt::Display for PgnScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}