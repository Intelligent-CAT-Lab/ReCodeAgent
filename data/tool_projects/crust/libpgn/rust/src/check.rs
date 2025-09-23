#[repr(i8)] // Ensures binary compatibility with C enums
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgnCheck {
    Mate = -1,
    None = 0,
    Single,
    Double,
}
impl PgnCheck {
    /// Parses a PGN check annotation from a string, tracking characters consumed.
    pub fn __pgn_check_from_string(str: &str, consumed: &mut usize) -> Self {
        unimplemented!()
    }
}
impl From<&str> for PgnCheck {
    fn from(s: &str) -> Self {
        unimplemented!()
    }
}