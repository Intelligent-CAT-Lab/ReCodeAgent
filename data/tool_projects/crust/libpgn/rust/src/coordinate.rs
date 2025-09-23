pub const UNKNOWN: usize = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgnCoordinate {
    pub file: Option<char>,
    pub rank: Option<i32>,
}
pub fn pgn_coordinate_file_as_index(file: char) -> i32 {
    unimplemented!()
}