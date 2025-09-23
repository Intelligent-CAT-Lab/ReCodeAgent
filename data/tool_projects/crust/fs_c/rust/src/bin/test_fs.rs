use fs_c::fs::*;

use std::fs;

static TMP_DIR: &str = "./tmp";
static ALPHA: &str = "abcdefghijklmnopqrstuvwxyz\n";

fn setup() {
    let _ = fs::create_dir(TMP_DIR); // Ignore errors if it already exists
}

#[test]
fn test_fs_open() {
    setup();
    let file_path = format!("{}/file", TMP_DIR);

    fs::write(&file_path, "").unwrap();
    let fd = fs_open(&file_path, FS_OPEN_READWRITE);
    assert!(fd.is_some());

    fs_close(fd.unwrap()).unwrap();

    let bad_fd = fs_open("/root/foo", FS_OPEN_WRITE);
    assert!(bad_fd.is_none());
}

#[test]
fn test_fs_close() {
    setup();
    let file_path = format!("{}/file", TMP_DIR);

    fs::write(&file_path, "").unwrap();
    let fd = fs_open(&file_path, FS_OPEN_READWRITE);
    assert!(fs_close(fd.unwrap()).is_ok());
}

#[test]
fn test_fs_mkdir() {
    setup();
    let dir_path = format!("{}/dir", TMP_DIR);

    let _ = fs_rmdir(&dir_path);
    assert!(fs_mkdir(&dir_path, 0o777).is_ok());
    assert!(fs_exists(&dir_path));
    assert!(fs_rmdir(&dir_path).is_ok());
}

#[test]
fn test_fs_exists() {
    setup();
    let file_path = format!("{}/file", TMP_DIR);

    fs::write(&file_path, "").unwrap();
    assert!(fs_exists(&file_path));

    let non_existent = format!("{}/missing_file", TMP_DIR);
    assert!(!fs_exists(&non_existent));
}

#[test]
fn test_fs_stat() {
    setup();
    let file_path = format!("{}/file", TMP_DIR);

    fs::write(&file_path, ALPHA).unwrap();
    let metadata = fs_stat(&file_path);
    assert!(metadata.is_ok());

    let bad_metadata = fs_stat(&format!("{}/missing", TMP_DIR));
    assert!(bad_metadata.is_err());
}

#[test]
fn test_fs_fstat() {
    setup();
    let file_path = format!("{}/file", TMP_DIR);

    fs::write(&file_path, ALPHA).unwrap();
    let fd = fs_open(&file_path, FS_OPEN_READWRITE).unwrap();
    let metadata = fs_fstat(&fd);
    assert!(metadata.is_ok());
    fs_close(fd).unwrap();
}

#[test]
fn test_fs_lstat() {
    setup();
    let file_path = format!("{}/file", TMP_DIR);

    fs::write(&file_path, ALPHA).unwrap();
    // create a symlink for testing
    let symlink_path = format!("{}/symlink", TMP_DIR);
    std::os::unix::fs::symlink(&file_path, &symlink_path).unwrap();
    let metadata = fs_lstat(&symlink_path);
    assert!(metadata.is_ok());
}

#[test]
fn test_fs_truncate() {
    setup();
    let file_path = format!("{}/alpha", TMP_DIR);

    fs_write(&file_path, ALPHA.as_bytes()).unwrap();
    assert!(fs_truncate(&file_path, 9).is_ok());

    let buf = fs_read(&file_path).unwrap();
    assert_eq!(buf, b"abcdefghi");
}

#[test]
fn test_fs_read() {
    setup();
    let file_path = format!("{}/file", TMP_DIR);

    fs_write(&file_path, ALPHA.as_bytes()).unwrap();
    let buf = fs_read(&file_path).unwrap();
    assert_eq!(buf, ALPHA.as_bytes());
}

#[test]
fn test_fs_write() {
    setup();
    let file_path = format!("{}/alpha", TMP_DIR);

    fs_write(&file_path, ALPHA.as_bytes()).unwrap();
    let buf = fs_read(&file_path).unwrap();
    assert_eq!(buf, ALPHA.as_bytes());
}

#[test]
fn test_fs_rename() {
    setup();
    let from_path = format!("{}/alpha", TMP_DIR);
    let to_path = format!("{}/foo", TMP_DIR);

    fs_write(&from_path, ALPHA.as_bytes()).unwrap();
    assert!(fs_rename(&from_path, &to_path).is_ok());
    assert!(!fs_exists(&from_path));

    let buf = fs_read(&to_path).unwrap();
    assert_eq!(buf, ALPHA.as_bytes());

    fs::remove_file(&to_path).unwrap();
}

#[test]
fn test_fs_rmdir() {
    setup();
    let dir_path = format!("{}/dir", TMP_DIR);

    assert!(fs_mkdir(&dir_path, 0o777).is_ok());
    assert!(fs_rmdir(&dir_path).is_ok());
    assert!(!fs_exists(&dir_path));
}

#[test]
fn test_fs_fwrite() {
    setup();
    let file_path = format!("{}/alpha", TMP_DIR);

    let fd = fs_open(&file_path, FS_OPEN_WRITE).unwrap();
    fs_fwrite(&fd, ALPHA.as_bytes()).unwrap();
    fs_close(fd).unwrap();

    let buf = fs_read(&file_path).unwrap();
    assert_eq!(buf, ALPHA.as_bytes());
}

#[test]
fn test_fs_nread() {
    setup();
    let file_path = format!("{}/alpha", TMP_DIR);

    fs_write(&file_path, ALPHA.as_bytes()).unwrap();
    let buf = fs_nread(&file_path, 9).unwrap();
    assert_eq!(buf, b"abcdefghi");
}

#[test]
fn test_fs_fread() {
    setup();
    let file_path = format!("{}/alpha", TMP_DIR);

    fs_write(&file_path, ALPHA.as_bytes()).unwrap();
    let fd = fs_open(&file_path, FS_OPEN_READ).unwrap();
    let buf = fs_fread(&fd).unwrap();
    assert_eq!(buf, ALPHA.as_bytes());
    fs_close(fd).unwrap();
}

#[test]
fn test_fs_nwrite() {
    setup();
    let file_path = format!("{}/alpha", TMP_DIR);

    fs_nwrite(&file_path, ALPHA.as_bytes(), 9).unwrap();
    let buf = fs_read(&file_path).unwrap();
    assert_eq!(buf, b"abcdefghi");
}

#[test]
fn test_fs_fnwrite() {
    setup();
    let file_path = format!("{}/alpha", TMP_DIR);

    let fd = fs_open(&file_path, FS_OPEN_WRITE).unwrap();
    fs_fnwrite(&fd, ALPHA.as_bytes(), 9).unwrap();
    fs_close(fd).unwrap();

    let buf = fs_read(&file_path).unwrap();
    assert_eq!(buf, b"abcdefghi");
}
fn main(){}
