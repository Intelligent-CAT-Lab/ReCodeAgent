use std::sync::Arc;
use Graph_recogniser::openhash::OpenHashTable;
static TEST_STRS: &[(&str, &str)] = &[
    ("stefan", "manov"),
    ("hristo", "tenchev"),
    ("dimitar", "kajabachev"),
    ("georgi", "popov"),
    ("stanislav", "ivanov"),
    ("nikola", "yolov"),
    ("andrei", "radev"),
    ("iulen", "dobrev"),
    ("iasen", "bantchev"),
    ("samuele", "carli"),
    ("henning", "weiler"),
    ("javier", "martin"),
];
static PERMUT: &[usize] = &[10, 0, 4, 3, 5, 3, 7, 11, 4, 11, 6, 0, 1, 8, 5, 1, 10, 3, 5, 2, 9];
#[test]
pub fn test_openhash() {
    let tt = OpenHashTable::new(2);
    {
        let mut tt_write = tt.write().unwrap();
        for &(key, value) in TEST_STRS.iter() {
            tt_write.insert(key, value);
        }
    }
    {
        let tt_read = tt.read().unwrap();
        for &index in PERMUT.iter() {
            assert_eq!(
                tt_read.find(TEST_STRS[index].0),
                Some(TEST_STRS[index].1),
                "Key {} not found or incorrect value",
                TEST_STRS[index].0
            );
        }
    }
}
fn main(){}