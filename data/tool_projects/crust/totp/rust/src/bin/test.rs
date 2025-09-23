use totp::totp::{unpack32, unpack64, pack32, sha1, hmac_sha1, hotp, from_base32};
use totp::std::{memset, memcpy, strlen};

fn to_hex(a: &[u8], len: usize) -> String {
    let mut buf = String::new();
    for i in 0..len {
        buf.push_str(&format!("{:02x}", a[i]));
    }
    buf
}

#[test]
fn test_pack() {
    let mut a = [0u8; 4];
    
    unpack32(0x12345678, &mut a);
    // create a new array with the first 4 bytes of a
    let mut a = [a[0], a[1], a[2], a[3], 0, 0, 0, 0];
    assert_eq!(a[0], 0x12);
    assert_eq!(a[1], 0x34);
    assert_eq!(a[2], 0x56);
    assert_eq!(a[3], 0x78);
    unpack64(0x123456789ABCDEF0, &mut a);
    assert_eq!(a[0], 0x12);
    assert_eq!(a[1], 0x34);
    assert_eq!(a[2], 0x56);
    assert_eq!(a[3], 0x78);
    assert_eq!(a[4], 0x9A);
    assert_eq!(a[5], 0xBC);
    assert_eq!(a[6], 0xDE);
    assert_eq!(a[7], 0xF0);
    // create a new array with the first 4 bytes of a
    let a = [a[0], a[1], a[2], a[3]];
     // pack the array into a u32
    assert_eq!(pack32(&a), 0x12345678);
}

#[test]
fn test_sha1() {
    let mut buf = [0u8; 512];
    let mut hash = [0u8; 20];
    buf[0] = 0;
    let buf_len = buf.len();
    sha1(&mut buf, 0, buf_len, &mut hash);
    assert_eq!(to_hex(&hash, 20), "da39a3ee5e6b4b0d3255bfef95601890afd80709");

    let abc = "abc";
    memcpy(&mut buf, abc.as_bytes(), abc.len());
    let buf_len = buf.len();
    sha1(&mut buf, abc.len(), buf_len, &mut hash);
    assert_eq!(to_hex(&hash, 20), "a9993e364706816aba3e25717850c26c9cd0d89d");

    let sentence = "The quick brown fox jumps over the lazy dog";
    memcpy(&mut buf, sentence.as_bytes(), sentence.len());
    let buf_len = buf.len();
    sha1(&mut buf, sentence.len(), buf_len, &mut hash);
    assert_eq!(to_hex(&hash, 20), "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
}

#[test]
fn test_hmac_sha1() {
    let mut key = [0u8; 64];
    let mut text = [0u8; 64];
    let mut hash = [0u8; 20];
    memset(&mut key, 0xAA, 20);
    memset(&mut text, 0xDD, 50);
    hmac_sha1(&key, &text, 50, &mut hash);
    assert_eq!(to_hex(&hash, 20), "125d7342b9ac11cd91a39af48aa17b4f63f175d3");
}

#[test]
fn test_hotp() {
    let secret: [u8; 64] = [
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
        0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
        0x37, 0x38, 0x39, 0x30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    assert_eq!(hotp(&secret, 0), 755224);
    assert_eq!(hotp(&secret, 1), 287082);
    assert_eq!(hotp(&secret, 2), 359152);
}

#[test]
fn test_from_base32() {
    let mut buf = [0u8; 10];
    let buf_len = buf.len();
    assert_eq!(from_base32("MZxw6===", &mut buf, buf_len), 3);
    assert_eq!(from_base32("MZxw6YQ=", &mut buf, buf_len), 4);
    assert_eq!(from_base32("MZxw6YTB", &mut buf, buf_len), 5);
    assert_eq!(from_base32("MZxw6YTBOI======", &mut buf, buf_len), 6);
    assert_eq!(std::str::from_utf8(&buf[..6]).unwrap(), "foobar");
}

fn main() {}