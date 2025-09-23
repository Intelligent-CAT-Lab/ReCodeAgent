use std::fmt::Display;
#[repr(i8)] // Ensures the enum has a fixed representation like in C
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgnAnnotation {
    Unknown = -1,
    Null = 0,
    GoodMove,        // !
    Mistake,         // ?
    BrilliantMove,   // !!
    Blunder,         // ??
    InterestingMove, // !?
    DubiousMove,     // ?!
}
impl From<i8> for PgnAnnotation {
    fn from(value: i8) -> Self {
        unimplemented!()
    }
}
impl PgnAnnotation {
    /// Parses a PGN annotation from a string, consuming characters as needed.
    pub fn pgn_annotation_nag_from_string(str: &str, consumed: &mut usize) -> Self {
        unimplemented!()
    }
    /// Parses a PGN annotation from a string (wrapper around the inner function).
    pub fn pgn_annotation_from_string(str: &str, consumed: &mut usize) -> Self {
        unimplemented!()
    }
}
impl Display for PgnAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
impl From<&str> for PgnAnnotation {
    fn from(s: &str) -> Self {
        unimplemented!()
    }
}