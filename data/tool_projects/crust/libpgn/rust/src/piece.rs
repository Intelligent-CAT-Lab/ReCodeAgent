use std::fmt::Display;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PgnPiece {
    Unknown = 0,
    Pawn = b'P',
    Rook = b'R',
    Knight = b'N',
    Bishop = b'B',
    Queen = b'Q',
    King = b'K',
}
impl From<char> for PgnPiece {
    fn from(ch: char) -> Self {
        unimplemented!()
    }
}
impl Display for PgnPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}