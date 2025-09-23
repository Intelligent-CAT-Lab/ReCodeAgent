use crate::{
    metadata::PgnMetadata,
    moves::{PgnMove, PgnMoves},
    score::PgnScore,
};
#[derive(Debug)]
pub struct Pgn {
    pub metadata: Option<Box<PgnMetadata>>, // Instead of raw `pgn_metadata_t *`
    pub moves: Option<Box<PgnMoves>>,       // Instead of raw `pgn_moves_t *`
    pub score: PgnScore,
}
impl Pgn {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn parse_metadata(s: &str) -> PgnMetadata {
        unimplemented!()
    }
    pub fn parse_move(s: &str) -> PgnMove {
        unimplemented!()
    }
    pub fn parse_moves(s: &str) -> PgnMoves {
        unimplemented!()
    }
    pub fn parse_score(s: &str) -> PgnScore {
        unimplemented!()
    }
    pub fn parse(&mut self, s: &str) -> usize {
        unimplemented!()
    }
}