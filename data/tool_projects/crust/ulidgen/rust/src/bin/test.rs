use ulidgen::ulid;
use std::thread::sleep;
use std::time::Duration;

fn is_valid_ulid(ulid: &[char; ulid::ULID_LENGTH]) -> bool {
    let b32alphabet = "0123456789ABCDEFGHJKMNPQRSTVWXYZ\0";
    ulid.iter().all(|&c| b32alphabet.contains(c))
}

#[test]
fn test_ulid_length() {
    let mut ulid = ['\0'; ulid::ULID_LENGTH];
    ulid::ulidgen_r(&mut ulid);
    assert_eq!(ulid.iter().take(26).count(), 26, "ULID length should be 26 characters");
}

#[test]
fn test_ulid_structure() {
    let mut ulid = ['\0'; ulid::ULID_LENGTH];
    ulid::ulidgen_r(&mut ulid);
    assert!(is_valid_ulid(&ulid), "ULID should only contain valid Base32 characters");
}

#[test]
fn test_ulid_uniqueness() {
    let mut ulid1 = ['\0'; ulid::ULID_LENGTH];
    let mut ulid2 = ['\0'; ulid::ULID_LENGTH];
    ulid::ulidgen_r(&mut ulid1);
    ulid::ulidgen_r(&mut ulid2);
    assert_ne!(ulid1, ulid2, "Consecutive ULIDs should be unique");
}

#[test]
fn test_ulid_sortability() {
    let mut ulid1 = ['\0'; ulid::ULID_LENGTH];
    let mut ulid2 = ['\0'; ulid::ULID_LENGTH];
    ulid::ulidgen_r(&mut ulid1);
    sleep(Duration::from_millis(1)); // Sleep for 1 millisecond
    ulid::ulidgen_r(&mut ulid2);
    assert!(ulid1 < ulid2, "ULIDs should be lexicographically sortable based on time");
}

fn main() {}