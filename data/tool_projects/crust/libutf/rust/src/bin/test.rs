use libutf::libutf_utf;
use libutf::libutf_string;

#[test]
fn test_string_append() {
let mut string = Utf8String::new();
assert!(string.append_literal(b"hello").is_ok());
assert!(string.append_literal(b" world").is_ok());
assert_eq!(&string.data[..string.len], b"hello world");
string.destroy();
}

#[test]
fn test_string_prepend() {
let mut string = Utf8String::new();
assert!(string.prepend_literal(b" world").is_ok());
assert!(string.prepend_literal(b"hello").is_ok());
assert_eq!(&string.data[..string.len], b"hello world");
string.destroy();
}

#[test]
fn test_string_replace() {
let mut string = Utf8String::new();
assert!(string.append_literal(b"hello world").is_ok());
assert!(string.replace_literal(5, 1, b" there ").is_ok());
assert_eq!(&string.data[..string.len], b"hello there world");
string.destroy();
}

#[test]
fn test_string_substring() {
let mut string = Utf8String::new();
assert!(string.append_literal(b"hello world").is_ok());
let view = string.substring(2, 8);
assert_eq!(view.data, b"llo worl");
string.destroy();
}

#[test]
fn test_utf8_convert() {
fn test_convert(string: &[u16], expected: &[u8], written: usize) {
let mut result = vec![0u8; written];
assert_eq!(libutf_utf::utf16le_convert_to_utf8(string, &mut result), written);
assert_eq!(&result[..written], expected);
}

test_convert(&[0x0061, 0x0062, 0x0063], b"abc", 3);
test_convert(&[0x03B1, 0x03B2, 0x03B3], b"\xCE\xB1\xCE\xB2\xCE\xB3", 6);
test_convert(&[0x4E67, 0x97F3, 0x4E2D], b"\xE6\x9D\x8E\xE9\x9F\xB3\xE4\xB8\xAD", 9);
test_convert(&[0x0061, 0x03B1, 0x4E67], b"a\xCE\xB1\xE6\x9D\x8E", 6);
test_convert(&[0xD800, 0xDC37], b"\xF0\x90\x80\xB7", 4);
}

#[test]
fn test_utf16_convert() {
fn test_convert(string: &[u8], expected: &[u16], written: usize) {
let mut result = vec![0u16; written];
assert_eq!(libutf_utf::utf8_convert_to_utf16le(string, &mut result), written);
assert_eq!(&result[..written], expected);
}

test_convert(b"\x41", &[0x0041], 1);
test_convert(b"\xCE\xA9", &[0x03A9], 1);
test_convert(b"\xE2\x98\x83", &[0x2603], 1);
test_convert(b"\xE4\xB8\xAD", &[0x4E2D], 1);
test_convert(b"\xF0\x9F\x98\x82", &[0xD83D, 0xDE02], 2);
test_convert(b"\x41\xCE\xA9\xE2\x98\x83\xE4\xB8\xAD\xF0\x9F\x98\x82", &[0x0041, 0x03A9, 0x2603, 0x4E2D, 0xD83D, 0xDE02], 6);
}

#[test]
fn test_utf16_length() {
fn test_length(string: &[u8], expected: usize) {
assert_eq!(libutf_utf::utf16_length_from_utf8(string), expected);
}

test_length(b"\x41", 1);
test_length(b"\xCE\xA9", 1);
test_length(b"\xE2\x98\x83", 1);
test_length(b"\xE4\xB8\xAD", 1);
test_length(b"\xF0\x9F\x98\x82", 2);
test_length(b"\x41\xCE\xA9\xE2\x98\x83\xE4\xB8\xAD\xF0\x9F\x98\x82", 6);
}

#[test]
fn test_validate_invalid_utf8() {
fn test_validate(string: &[u8]) {
assert!(!libutf_utf::utf8_validate(string));
}

test_validate(b"\x80");
test_validate(b"\xC3");
test_validate(b"\xC0\x80");
test_validate(b"\xC3\xA9\xA8");
test_validate(b"\x80\x80");
test_validate(b"\xFE");
test_validate(b"\xE2\x82");
test_validate(b"\xF4\x90\x80\x80");
test_validate(b"\xED\xA0\x80");
test_validate(b"\xED\xBF\xBF");
test_validate(b"\xE2\x82\xC0");
test_validate(b"\x80\xC2");
test_validate(b"\xF8\x88\x88\x88\x88");
test_validate(b"\xFC\x84\x84\x84\x84\x84");
}

#[test]
fn test_validate_invalid() {
fn test_validate(string: &[u16]) {
assert!(!libutf_utf::utf16le_validate(string));
}

test_validate(&[0xD801]);
test_validate(&[0xDC37]);
test_validate(&[0xD801, 0xD802]);
test_validate(&[0xDC37, 0xDC38]);
test_validate(&[0xDC37, 0xD801]);
test_validate(&[0xD801, 0xD802, 0xDC37]);
test_validate(&[0xD801, 0xDC37, 0xDC38]);
test_validate(&[0xD801, 0xD802]);
test_validate(&[0xDC37, 0xDC38]);
}

#[test]
fn test_validate_utf8() {
fn test_validate(string: &[u8]) {
assert!(libutf_utf::utf8_validate(string));
}

test_validate(b"a");
test_validate(b"Hello, World!");
test_validate(b"\xC3\xA9");
test_validate(b"\xC3\xA9\xC3\xA8\xC3\xAA\xC3\xAB\xC3\xAF");
test_validate(b"\xE2\x82\xAC");
test_validate(b"\xE2\x82\xAC\xE2\x89\xA1\xE2\x9C\x93");
test_validate(b"\xF0\x9F\x98\x80");
test_validate(b"\xF0\x9F\x98\x80\xF0\x9F\x98\x81\xF0\x9F\x98\x82");
test_validate(b"a \xC3\xA9 \xE2\x82\xAC \xF0\x9F\x98\x80");
}

#[test]
fn test_validate() {
fn test_validate(string: &[u16]) {
assert!(libutf_utf::utf16le_validate(string));
}

test_validate(&[0x0061]);
test_validate(&[0x0048, 0x0065, 0x006C, 0x006C, 0x006F]);
test_validate(&[0x00E8]);
test_validate(&[0x00E8, 0x00E9, 0x00EA, 0x00EB]);
test_validate(&[0xD801, 0xDC37]);
test_validate(&[0xD801, 0xDC37, 0xD83F, 0xDC1F]);
test_validate(&[0x0048, 0x0065, 0x006C, 0x006C, 0x006F, 0xD801, 0xDC37]);
}


fn main() {
}

