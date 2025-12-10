// Tests for utility functions

use super::utils::*;

// Global test data initialized lazily
fn get_hash_map_a() -> StringHashMap {
    let mut map = StringHashMap::new();
    map.insert("a".to_string());
    map.insert("b".to_string());
    map.insert("c".to_string());
    map
}

fn get_hash_map_b() -> StringHashMap {
    let mut map = StringHashMap::new();
    map.insert("d".to_string());
    map.insert("e".to_string());
    map.insert("f".to_string());
    map
}

#[test]
fn test_min() {
    struct TestCase {
        name: &'static str,
        a: i32,
        b: i32,
        want: i32,
    }

    let tests = vec![
        TestCase {
            name: "Min between 2/4",
            a: 2,
            b: 4,
            want: 2,
        },
        TestCase {
            name: "Min between -25/-42",
            a: -25,
            b: -42,
            want: -42,
        },
    ];

    for tt in tests {
        let got = min(tt.a, tt.b);
        assert_eq!(got, tt.want, "Test case: {} - min() = {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_max() {
    struct TestCase {
        name: &'static str,
        a: i32,
        b: i32,
        want: i32,
    }

    let tests = vec![
        TestCase {
            name: "Min between 2/4",
            a: 2,
            b: 4,
            want: 4,
        },
        TestCase {
            name: "Min between -25/-42",
            a: -25,
            b: -42,
            want: -25,
        },
    ];

    for tt in tests {
        let got = max(tt.a, tt.b);
        assert_eq!(got, tt.want, "Test case: {} - max() = {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_equal() {
    struct TestCase {
        name: &'static str,
        a: Vec<char>,
        b: Vec<char>,
        want: bool,
    }

    let tests = vec![
        TestCase {
            name: "Equal between toto/test",
            a: "toto".chars().collect(),
            b: "test".chars().collect(),
            want: false,
        },
        TestCase {
            name: "Equal between toto/toto",
            a: "toto".chars().collect(),
            b: "toto".chars().collect(),
            want: true,
        },
        TestCase {
            name: "Equal between Toto/toto",
            a: "Toto".chars().collect(),
            b: "toto".chars().collect(),
            want: false,
        },
        TestCase {
            name: "Equal between 🙂😄/🙂😄",
            a: "🙂😄".chars().collect(),
            b: "🙂😄".chars().collect(),
            want: true,
        },
        TestCase {
            name: "Equal between 🙂😄/🙂😄🙂😄",
            a: "🙂😄".chars().collect(),
            b: "🙂😄🙂😄".chars().collect(),
            want: false,
        },
    ];

    for tt in tests {
        let got = equal(&tt.a, &tt.b).unwrap();
        assert_eq!(got, tt.want, "Test case: {} - equal() = {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_string_hash_map_add_all() {
    struct TestCase {
        name: &'static str,
    }

    let tests = vec![
        TestCase {
            name: "add_all between hashMapA/hashMapB",
        },
    ];

    for tt in tests {
        let mut m = get_hash_map_a();
        let src_map = get_hash_map_b();

        let old_len = m.len();
        let src_len = src_map.len();

        m.add_all(&src_map);

        assert_eq!(
            m.len(),
            old_len + src_len,
            "add_all() failed for test case: \"{}\"",
            tt.name
        );
    }
}

#[test]
fn test_string_hash_map_to_array() {
    struct TestCase {
        name: &'static str,
        map: StringHashMap,
    }

    let tests = vec![
        TestCase {
            name: "to_array() hashMapA",
            map: get_hash_map_a(),
        },
        TestCase {
            name: "to_array() hashMapB",
            map: get_hash_map_b(),
        },
    ];

    for tt in tests {
        let got = tt.map.to_array();
        assert_eq!(
            got.len(),
            tt.map.len(),
            "to_array() failed for test case: \"{}\"",
            tt.name
        );
    }
}
