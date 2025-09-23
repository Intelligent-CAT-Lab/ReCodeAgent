use libpgn::coordinate::pgn_coordinate_file_as_index;

#[test]
fn test_convert_coordinate() {
    assert_eq!(pgn_coordinate_file_as_index('a'), 0);
    assert_eq!(pgn_coordinate_file_as_index('b'), 1);
    assert_eq!(pgn_coordinate_file_as_index('c'), 2);
    assert_eq!(pgn_coordinate_file_as_index('d'), 3);
    assert_eq!(pgn_coordinate_file_as_index('e'), 4);
    assert_eq!(pgn_coordinate_file_as_index('f'), 5);
    assert_eq!(pgn_coordinate_file_as_index('g'), 6);
    assert_eq!(pgn_coordinate_file_as_index('h'), 7);
}
fn main(){}