use c_string::string_t::*;
use std::vec::Vec;

fn print_str(info: &str, str: &StringT) {
    println!("{}: \"{}\"", info, string_bytes(str));
}

#[test]
fn test_new_string() {
    let sizes = [0, 1, 2, 3, 12, 100];
    for &size in &sizes {
        let str = new_string(size);
        assert_eq!(str.size, size);
        string_free(str);
    }
}

#[test]
fn test_new_string_from_bytes() {
    let bytes = ["", "test", "some another test"];
    for &byte in &bytes {
        let str = new_string_from_bytes(byte);
        assert_eq!(str.size, byte.len());
        assert_eq!(string_bytes(&str), byte);
        string_free(str);
    }
}

#[test]
fn test_string_len() {
    let bytes = ["", "test", "some another test"];
    let len = [0, 4, 17];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        assert_eq!(str.size, byte.len());
        assert_eq!(string_len(&str), len[idx]);
        string_free(str);
    }
}

#[test]
fn test_string_bytes() {
    let bytes = ["", "test", "some another test"];
    for &byte in &bytes {
        let str = new_string_from_bytes(byte);
        let str_bytes = string_bytes(&str);
        assert_eq!(str_bytes, byte);
        string_free(str);
    }
}

#[test]
fn test_string_eq() {
    let left_bytes = ["", "\n\n", "test", "some another test"];
    let right_bytes = ["", "\n\n", "test", "some another test"];
    for (idx, &left_byte) in left_bytes.iter().enumerate() {
        let left_str = new_string_from_bytes(left_byte);
        let right_str = new_string_from_bytes(right_bytes[idx]);
        assert_eq!(left_str.size, right_str.size);
        assert!(string_eq(&left_str, &right_str));
        string_free(left_str);
        string_free(right_str);
    }
}

#[test]
fn test_string_copy() {
    let bytes = ["", "test", "some another test"];
    for &byte in &bytes {
        let str = new_string_from_bytes(byte);
        let str_copy = string_copy(&str);
        assert!(string_eq(&str, &str_copy));
        string_free(str);
        string_free(str_copy);
    }
}

#[test]
fn test_string_concat() {
    let first = ["", "first", "some another"];
    let second = ["", "second", " test"];
    let res = ["", "firstsecond", "some another test"];
    for (idx, &first_byte) in first.iter().enumerate() {
        let first_str = new_string_from_bytes(first_byte);
        let second_str = new_string_from_bytes(second[idx]);
        let res_str = new_string_from_bytes(res[idx]);
        let concat_str = string_concat(&first_str, &second_str);
        assert!(string_eq(&concat_str, &res_str));
        string_free(first_str);
        string_free(second_str);
        string_free(res_str);
        string_free(concat_str);
    }
}

#[test]
fn test_string_strip() {
    let bytes = ["", "vfv\n\n", "  test\t", " some another test  "];
    let stripped_bytes = ["", "vfv", "test", "some another test"];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        let stripped_str = string_strip(&str);
        let exp_stripped_str = new_string_from_bytes(stripped_bytes[idx]);
        assert!(string_eq(&stripped_str, &exp_stripped_str));
        string_free(str);
        string_free(stripped_str);
        string_free(exp_stripped_str);
    }
}

#[test]
fn test_string_substr() {
    let bytes = ["", "vfv\n\n", "test string", " some another test  "];
    let substr_start_pos = [0, 1, 5, 0];
    let substr_len = [0, 2, 3, 5];
    let substr_bytes = ["", "fv", "str", " some"];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        let substr_str = string_substr(&str, substr_start_pos[idx], substr_len[idx]);
        let expected_substr_str = new_string_from_bytes(substr_bytes[idx]);
        assert!(string_eq(&substr_str, &expected_substr_str));
        string_free(str);
        string_free(substr_str);
        string_free(expected_substr_str);
    }
}

#[test]
fn test_string_startswith() {
    let bytes = ["", "vfv\n\n", "test string", " some another test  ", "1234"];
    let startswith_bytes = ["", "vfv", "test string", " some", "2"];
    let res = [true, true, true, true, false];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        assert_eq!(string_startswith(&str, startswith_bytes[idx]), res[idx]);
        string_free(str);
    }
}

#[test]
fn test_string_endswith() {
    let bytes = ["", "vfv\n\n", "test string", " some another test  ", "1234"];
    let endswith_bytes = ["", "fv\n\n", "test string", "test  ", "2"];
    let res = [true, true, true, true, false];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        assert_eq!(string_endswith(&str, endswith_bytes[idx]), res[idx]);
        string_free(str);
    }
}

#[test]
fn test_string_find() {
    let bytes = ["", "vfv\n\n", "test string", "test string", " some another test  "];
    let chars = ["", "\n", "no", "", "another"];
    let expected_pos:Vec<Option<usize>> = vec![Some(0), Some(3), None, Some(0), Some(6)];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        assert_eq!(string_find(&str, chars[idx]), expected_pos[idx]);
        string_free(str);
    }
}

#[test]
fn test_string_split() {
    let bytes = [
        "",
        "1",
        "some",
        " some string 124!",
        "Some github account: vnkrtv",
    ];
    let expected_arr_size = [1, 1, 1, 4, 4];
    let first_arr = vec![new_string_from_bytes("")];
    let second_arr = vec![new_string_from_bytes("1")];
    let third_arr = vec![new_string_from_bytes("some")];
    let fourth_arr = vec![
        new_string_from_bytes(""),
        new_string_from_bytes("some"),
        new_string_from_bytes("string"),
        new_string_from_bytes("124!"),
    ];
    let fifth_arr = vec![
        new_string_from_bytes("Some"),
        new_string_from_bytes("github"),
        new_string_from_bytes("account:"),
        new_string_from_bytes("vnkrtv"),
    ];
    let arr_expected = vec![first_arr, second_arr, third_arr, fourth_arr, fifth_arr];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        let mut arr_size = 0;
        let str_arr = string_split(&str, &mut arr_size);
        let expected_str_arr = &arr_expected[idx];
        assert_eq!(arr_size, expected_arr_size[idx]);
        for (arr_idx, expected_str) in expected_str_arr.iter().enumerate() {
            assert!(string_eq(&str_arr[arr_idx], expected_str));
        }
        string_free(str);
        for s in str_arr {
            string_free(s);
        }
    }
}

#[test]
fn test_string_split_by() {
    let split_by_chars = "W";
    let bytes = [
        "",
        "1",
        "some",
        "WsomeWstringW124!",
        "SomeWgithubWaccount:Wvnkrtv",
    ];
    let expected_arr_size = [1, 1, 1, 4, 4];
    let first_arr = vec![new_string_from_bytes("")];
    let second_arr = vec![new_string_from_bytes("1")];
    let third_arr = vec![new_string_from_bytes("some")];
    let fourth_arr = vec![
        new_string_from_bytes(""),
        new_string_from_bytes("some"),
        new_string_from_bytes("string"),
        new_string_from_bytes("124!"),
    ];
    let fifth_arr = vec![
        new_string_from_bytes("Some"),
        new_string_from_bytes("github"),
        new_string_from_bytes("account:"),
        new_string_from_bytes("vnkrtv"),
    ];
    let arr_expected = vec![first_arr, second_arr, third_arr, fourth_arr, fifth_arr];
    for (idx, &byte) in bytes.iter().enumerate() {
        let str = new_string_from_bytes(byte);
        let mut arr_size = 0;
        let str_arr = string_split_by(&str, &mut arr_size, split_by_chars);
        let expected_str_arr = &arr_expected[idx];
        assert_eq!(arr_size, expected_arr_size[idx]);
        for (arr_idx, expected_str) in expected_str_arr.iter().enumerate() {
            assert!(string_eq(&str_arr[arr_idx], expected_str));
        }
        string_free(str);
        for s in str_arr {
            string_free(s);
        }
    }
}

#[test]
fn test_string_join_arr() {
    let first_arr = vec![new_string_from_bytes("")];
    let second_arr = vec![new_string_from_bytes("1")];
    let third_arr = vec![
        new_string_from_bytes("some"),
        new_string_from_bytes("string"),
    ];
    let fourth_arr = vec![
        new_string_from_bytes(""),
        new_string_from_bytes("some"),
        new_string_from_bytes("string"),
    ];
    let fifth_arr = vec![
        new_string_from_bytes("some"),
        new_string_from_bytes("string"),
    ];
    let str_arr = vec![first_arr, second_arr, third_arr, fourth_arr, fifth_arr];
    let arr_size = [1, 1, 2, 3, 2];
    let space_chars = [" ", " ", " ", "\n", "SOME"];
    let expected_res_str = vec![
        new_string_from_bytes(""),
        new_string_from_bytes("1"),
        new_string_from_bytes("some string"),
        new_string_from_bytes("\nsome\nstring"),
        new_string_from_bytes("someSOMEstring"),
    ];
    for (idx, arr) in str_arr.iter().enumerate() {
        let res_str = string_join_arr(arr, arr_size[idx], space_chars[idx]);
        assert!(string_eq(&res_str, &expected_res_str[idx]));
        string_free(res_str);
    }
}

fn main() {}