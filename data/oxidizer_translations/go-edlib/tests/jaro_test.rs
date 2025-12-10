// Tests for Jaro and Jaro-Winkler similarity

use wspace::*;

#[test]
fn test_jaro_similarity() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: f32,
    }

    let tests = vec![
        TestCase { name: "First arg empty", str1: "", str2: "abcde", want: 0.0 },
        TestCase { name: "Second arg empty", str1: "abcde", str2: "", want: 0.0 },
        TestCase { name: "Same args", str1: "abcde", str2: "abcde", want: 1.0 },
        TestCase { name: "No characters match", str1: "abcd", str2: "effgghh", want: 0.0 },
        TestCase { name: "CRATE/TRACE", str1: "CRATE", str2: "TRACE", want: 0.73333335 },
        TestCase { name: "MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", want: 0.9444444 },
        TestCase { name: "DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", want: 0.76666665 },
        TestCase { name: "jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", want: 0.8962963 },
    ];

    for tt in tests {
        let got = wspace::jaro::jaro_similarity(tt.str1, tt.str2).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_jaro_winkler_similarity() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: f32,
    }

    let tests = vec![
        TestCase { name: "First arg empty", str1: "", str2: "abcde", want: 0.0 },
        TestCase { name: "Second arg empty", str1: "abcde", str2: "", want: 0.0 },
        TestCase { name: "Same args", str1: "abcde", str2: "abcde", want: 1.0 },
        TestCase { name: "No characters match", str1: "abcd", str2: "effgghh", want: 0.0 },
        TestCase { name: "TRACE/TRACE", str1: "TRACE", str2: "TRACE", want: 1.0 },
        TestCase { name: "CRATE/TRACE", str1: "CRATE", str2: "TRACE", want: 0.73333335 },
        TestCase { name: "TRATE/TRACE", str1: "TRATE", str2: "TRACE", want: 0.90666664 },
        TestCase { name: "DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", want: 0.81333333 },
    ];

    for tt in tests {
        let got = wspace::jaro::jaro_winkler_similarity(tt.str1, tt.str2).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}
