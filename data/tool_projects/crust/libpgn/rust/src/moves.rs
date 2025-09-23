use std::fmt::Display;
use crate::{
    annotation::PgnAnnotation, check::PgnCheck, comments::PgnComments, coordinate::PgnCoordinate,
    piece::PgnPiece,
};
pub const PGN_CASTLING_NONE: u8 = 0;
pub const PGN_CASTLING_KINGSIDE: u8 = 2;
pub const PGN_CASTLING_QUEENSIDE: u8 = 3;
pub const PGN_MOVES_INITIAL_SIZE: usize = 32;
pub const PGN_MOVES_GROW_SIZE: usize = 32;
pub const PGN_MOVE_NOTATION_SIZE: usize = 16;
pub const PGN_ALTERNATIVE_MOVES_INITIAL_SIZE: usize = 1;
pub const PGN_ALTERNATIVE_MOVES_GROW_SIZE: usize = 1;
#[derive(Debug)]
pub struct PgnMove {
    pub piece: PgnPiece,
    pub promoted_to: PgnPiece,
    pub notation: String,
    pub castles: u8,
    pub captures: bool,
    pub en_passant: bool,
    pub check: PgnCheck,
    pub from: PgnCoordinate,
    pub dest: PgnCoordinate,
    pub annotation: PgnAnnotation,
    pub comments: Option<PgnComments>,
    pub alternatives: Option<PgnAlternativeMoves>,
}
impl Default for PgnMove {
    fn default() -> Self {
        unimplemented!()
    }
}
impl PgnMove {
    pub fn from_string_with_consumption(s: &str, consumed: &mut usize) -> Self {
        unimplemented!()
    }
}
impl From<&str> for PgnMove {
    fn from(s: &str) -> Self {
        unimplemented!()
    }
}
impl Display for PgnMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
#[derive(Debug)]
pub struct PgnMoves {
    pub values: Vec<PgnMovesItem>,
}
impl From<&str> for PgnMoves {
    fn from(s: &str) -> Self {
        unimplemented!()
    }
}
impl PgnMoves {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn from_string_with_consumption(s: &str, consumed: &mut usize) -> Self {
        unimplemented!()
    }
    pub fn push(&mut self, moves: PgnMovesItem) {
        unimplemented!()
    }
}
#[derive(Debug)]
pub struct PgnMovesItem {
    pub white: PgnMove,
    pub black: PgnMove,
}
#[derive(Debug)]
pub struct PgnAlternativeMoves {
    pub values: Vec<Box<PgnMoves>>,
}
impl PgnAlternativeMoves {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn poll(
        alt: &mut Option<Self>,
        placeholder: &mut Option<PgnComments>,
        s: &str,
        expect: i32,
    ) -> usize {
        unimplemented!()
    }
    pub fn push(&mut self, moves: PgnMoves) {
        unimplemented!()
    }
}