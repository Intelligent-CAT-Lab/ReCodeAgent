use file2str::file2str::*;

#[test]
fn test_file2str_file_read_properly() {
    let mut len: u32 = 0;
    let buf = file2strl("src/bin/test.txt", &mut len)
        .expect("file2strl returned None (file read error)");

    assert_eq!(len, 11, "Expected length of 11");
    assert_eq!(buf, "Text file\n", "Expected string content does not match");
}
fn main(){}
