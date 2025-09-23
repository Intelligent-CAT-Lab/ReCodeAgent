use utf8::utf8::{make_utf8_string, make_utf8_string_lossy, make_utf8_char_iter, next_utf8_char, free_owned_utf8_string, validate_utf8, slice_utf8_string, utf8_char_count, is_utf8_char_boundary, nth_utf8_char, unicode_code_point};
use utf8::utf8::{Utf8Char};
#[test]
fn test_validate_utf8_ok() {
    let validity = validate_utf8("Hello Здравствуйте こんにちは 🚩😁");
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 5 + 1 + 12 * 2 + 1 + 5 * 3 + 1 + 2 * 4);
}

#[test]
fn test_validate_utf8_boundary_ok() {
    let test_string = "\u{7F}";
    let mut validity = validate_utf8(test_string);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 1);
    let test_string = "\u{C2}\u{80}";
    validity = validate_utf8(test_string);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 2);
    let test_string = "\u{DF}\u{BF}";
    validity = validate_utf8(test_string);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 2);
    let test_string = "\u{E0}\u{A0}\u{80}";
    validity = validate_utf8(test_string);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 3);
    let test_string = "\u{EF}\u{BF}\u{BF}";
    validity = validate_utf8(test_string);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 3);
    let test_string = "\u{F0}\u{90}\u{80}\u{80}";
    validity = validate_utf8(test_string);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 4);
    let test_string = "\u{F4}\u{8F}\u{BF}\u{BF}";
    validity = validate_utf8(test_string);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, 4);
}

#[test]
fn test_surrogate_rejection() {
    let test_string = "\u{ED}\u{A0}\u{80}";
    let mut validity = validate_utf8(test_string);
    assert!(!validity.valid);
    assert_eq!(validity.valid_upto, 0);
    let test_string = "\u{ED}\u{AC}\u{80}";
    validity = validate_utf8(test_string);
    assert!(!validity.valid);
    assert_eq!(validity.valid_upto, 0);
    let test_string = "\u{ED}\u{AD}\u{BF}";
    validity = validate_utf8(test_string);
    assert!(!validity.valid);
    assert_eq!(validity.valid_upto, 0);
    let test_string = "\u{ED}\u{AE}\u{80}";
    validity = validate_utf8(test_string);
    assert!(!validity.valid);
    assert_eq!(validity.valid_upto, 0);
}

#[test]
fn test_validate_utf8_err() {
    let validity = validate_utf8("Hello Здравствуйте\u{C0}\u{C0} こんにちは 🚩😁");
    assert!(!validity.valid);
    assert_eq!(validity.valid_upto, 5 + 1 + 12 * 2);
}

fn assert_overlong_encodings(actual: Utf8Char, overlong: Utf8Char) {
    assert_eq!(unicode_code_point(actual.clone()), unicode_code_point(overlong.clone()));
    let mut validity = validate_utf8(&actual.str);
    assert!(validity.valid);
    assert_eq!(validity.valid_upto, actual.byte_len as usize);
    validity = validate_utf8(&overlong.str);
    assert!(!validity.valid);
    assert_eq!(validity.valid_upto, 0);
}

#[test]
fn test_validate_utf8_overlong_encoding_err() {
    assert_overlong_encodings(Utf8Char { str: "H".to_string(), byte_len: 1 }, Utf8Char { str: "\u{C1}\u{88}".to_string(), byte_len: 2 });
    assert_overlong_encodings(Utf8Char { str: "H".to_string(), byte_len: 1 }, Utf8Char { str: "\u{E0}\u{81}\u{88}".to_string(), byte_len: 3 });
    assert_overlong_encodings(Utf8Char { str: "H".to_string(), byte_len: 1 }, Utf8Char { str: "\u{F0}\u{80}\u{81}\u{88}".to_string(), byte_len: 4 });
    assert_overlong_encodings(Utf8Char { str: "д".to_string(), byte_len: 2 }, Utf8Char { str: "\u{E0}\u{90}\u{B4}".to_string(), byte_len: 3 });
    assert_overlong_encodings(Utf8Char { str: "д".to_string(), byte_len: 2 }, Utf8Char { str: "\u{F0}\u{80}\u{90}\u{B4}".to_string(), byte_len: 4 });
    assert_overlong_encodings(Utf8Char { str: "こ".to_string(), byte_len: 3 }, Utf8Char { str: "\u{F0}\u{83}\u{81}\u{93}".to_string(), byte_len: 4 });

    assert_overlong_encodings(Utf8Char { str: "\u{7F}".to_string(), byte_len: 1 }, Utf8Char { str: "\u{C1}\u{BF}".to_string(), byte_len: 2 });
    assert_overlong_encodings(Utf8Char { str: "\u{DF}\u{BF}".to_string(), byte_len: 2 }, Utf8Char { str: "\u{E0}\u{9F}\u{BF}".to_string(), byte_len: 3 });
    assert_overlong_encodings(Utf8Char { str: "\u{EF}\u{BF}\u{BF}".to_string(), byte_len: 3 }, Utf8Char { str: "\u{F0}\u{8F}\u{BF}\u{BF}".to_string(), byte_len: 4 });
}

#[test]
fn test_make_utf8_string_ok() {
    let str = "Hello Здравствуйте こんにちは 🚩😁";
    let ustr = make_utf8_string(str);
    assert_eq!(ustr.byte_len, 5 + 1 + 12 * 2 + 1 + 5 * 3 + 1 + 2 * 4);
    assert_eq!(ustr.str, str);
}

#[test]
fn test_make_utf8_string_err() {
    let ustr = make_utf8_string("Hello Здравствуйте\u{C0}\u{C0} こんにちは 🚩😁");
    assert_eq!(ustr.str, "");
    assert_eq!(ustr.byte_len, 0);
}

#[test]
fn test_make_utf8_string_lossy_ok() {
    let str = "Hello Здравствуйте こんにちは 🚩😁";
    let owned_ustr = make_utf8_string_lossy(str);
    assert_eq!(owned_ustr.byte_len, 5 + 1 + 12 * 2 + 1 + 5 * 3 + 1 + 2 * 4);
    assert_eq!(owned_ustr.str, str);
    free_owned_utf8_string(&mut owned_ustr.clone());
}

#[test]
fn test_make_utf8_string_lossy_invalid_sequence() {
    let str_test = "\u{C0}He\u{C0}llo Здр\u{C0}авствуйте\u{C0}\u{C0} こんに\u{C0}\u{C0}\u{C0}\u{C0}ちは 🚩\u{C0}😁\u{C0}";
    let expected = "�He�llo Здр�авствуйте�� こんに����ちは 🚩�😁�";
    let owned_ustr = make_utf8_string_lossy(str_test);
    assert_eq!(owned_ustr.byte_len, expected.len());
    assert_eq!(owned_ustr.str, expected);
    free_owned_utf8_string(&mut owned_ustr.clone());
}

#[test]
fn test_make_utf8_string_lossy_completely_invalid() {

    let test_string = "\u{C0}\u{C0}\u{C0}\u{C0}";
    let expected = "����";
    let owned_ustr = make_utf8_string_lossy(test_string);
    assert_eq!(owned_ustr.byte_len, expected.len());
    assert_eq!(owned_ustr.str, expected);
    free_owned_utf8_string(&mut owned_ustr.clone());
}

#[test]
fn test_make_utf8_string_slice_ok() {
    let str = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let slice = slice_utf8_string(str, 6, 24);
    assert_eq!(slice.byte_len, 12 * 2);
    assert_eq!(&slice.str[..slice.byte_len], "Здравствуйте");
}

#[test]
fn test_make_utf8_string_slice_start_out_of_bounds_ok() {
    let str = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let slice = slice_utf8_string(str, 1000, 1);
    assert_eq!(slice.byte_len, 0);
    assert_eq!(slice.str, "");
}

#[test]
fn test_make_utf8_string_slice_end_out_of_bounds_ok() {
    let str = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let slice = slice_utf8_string(str, 6, 1000);
    assert_eq!(slice.byte_len, 12 * 2 + 1 + 5 * 3 + 1 + 2 * 4);
    assert_eq!(&slice.str[..slice.byte_len], "Здравствуйте こんにちは 🚩😁");
}

#[test]
fn test_make_utf8_string_slice_start_non_boundary_err() {
    let str = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let slice = slice_utf8_string(str, 7, 3);
    assert_eq!(slice.str, "");
    assert_eq!(slice.byte_len, 0);
}

#[test]
fn test_make_utf8_string_slice_end_non_boundary_err() {
    let str = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let slice = slice_utf8_string(str, 6, 3);
    assert_eq!(slice.str, "");
    assert_eq!(slice.byte_len, 0);
}

#[test]
fn test_utf8_char_iter() {
    let str = make_utf8_string("Hдこ😁");
    let mut iter = make_utf8_char_iter(str);
    let mut ch = next_utf8_char(&mut iter);
    assert_eq!(ch.byte_len, 1);
    assert_eq!(&ch.str[..ch.byte_len as usize], "H");
    ch = next_utf8_char(&mut iter);
    assert_eq!(ch.byte_len, 2);
    assert_eq!(&ch.str[..ch.byte_len as usize], "д");
    ch = next_utf8_char(&mut iter);
    assert_eq!(ch.byte_len, 3);
    assert_eq!(&ch.str[..ch.byte_len as usize], "こ");
    ch = next_utf8_char(&mut iter);
    assert_eq!(ch.byte_len, 4);
    assert_eq!(&ch.str[..ch.byte_len as usize], "😁");

    ch = next_utf8_char(&mut iter);
    assert_eq!(ch.byte_len, 0);
    assert_eq!(ch.str, "");
    ch = next_utf8_char(&mut iter);
    assert_eq!(ch.byte_len, 0);
    assert_eq!(ch.str, "");
}

#[test]
fn test_utf8_char_count_zero() {
    let ustr = make_utf8_string("");
    let count = utf8_char_count(ustr);
    assert_eq!(count, 0);
}

#[test]
fn test_utf8_char_count() {
    let ustr = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let count = utf8_char_count(ustr);
    assert_eq!(count, 5 + 1 + 12 + 1 + 5 + 1 + 2);
}

#[test]
fn test_is_utf8_char_boundary() {
    let mut str = "Hдこ😁";
    assert!(is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(!is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(!is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(!is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(!is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(!is_utf8_char_boundary(str));
    str = &str[1..];
    assert!(!is_utf8_char_boundary(str));
    assert_eq!(str, "");
}

#[test]
fn test_nth_utf8_char_valid_index_ok() {
    let ustr = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let ch = nth_utf8_char(ustr, 20);
    assert_eq!(ch.byte_len, 3);
    assert_eq!(&ch.str[..ch.byte_len as usize], "ん");
}

#[test]
fn test_nth_utf8_char_first_index_ok() {
    let ustr = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let ch = nth_utf8_char(ustr, 0);
    assert_eq!(ch.byte_len, 1);
    assert_eq!(ch.str.chars().next().unwrap(), 'H');
}

#[test]
fn test_nth_utf8_char_last_index_ok() {
    let ustr = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let ch = nth_utf8_char(ustr, 26);
    assert_eq!(ch.byte_len, 4);
    assert_eq!(&ch.str[..ch.byte_len as usize], "😁");
}

#[test]
fn test_nth_utf8_char_invalid_index_err() {
    let ustr = make_utf8_string("Hello Здравствуйте こんにちは 🚩😁");
    let ch = nth_utf8_char(ustr, 100);
    assert_eq!(ch.str, "");
    assert_eq!(ch.byte_len, 0);
}

#[test]
fn test_nth_utf8_char_empty_string_err() {
    let ustr = make_utf8_string("");
    let ch = nth_utf8_char(ustr, 0);
    assert_eq!(ch.str, "");
    assert_eq!(ch.byte_len, 0);
}

#[test]
fn test_unicode_code_point() {
    let ustr = make_utf8_string("Hдこ😁");
    let mut iter = make_utf8_char_iter(ustr);
    assert_eq!(unicode_code_point(next_utf8_char(&mut iter)), 72);
    assert_eq!(unicode_code_point(next_utf8_char(&mut iter)), 1076);
    assert_eq!(unicode_code_point(next_utf8_char(&mut iter)), 12371);
    assert_eq!(unicode_code_point(next_utf8_char(&mut iter)), 128513);
}

fn main() {}