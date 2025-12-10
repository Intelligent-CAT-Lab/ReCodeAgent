// Tests for Levenshtein distance

use wspace::*;

#[test]
fn test_levenshtein_distance() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: usize,
    }

    let tests = vec![
        TestCase { name: "First arg empty", str1: "", str2: "abcde", want: 5 },
        TestCase { name: "Second arg empty", str1: "abcde", str2: "", want: 5 },
        TestCase { name: "Same args", str1: "abcde", str2: "abcde", want: 0 },
        TestCase { name: "ab/aa", str1: "ab", str2: "aa", want: 1 },
        TestCase { name: "ab/ba", str1: "ab", str2: "ba", want: 2 },
        TestCase { name: "ab/aaa", str1: "ab", str2: "aaa", want: 2 },
        TestCase { name: "bbb/a", str1: "bbb", str2: "a", want: 3 },
        TestCase { name: "kitten/sitting", str1: "kitten", str2: "sitting", want: 3 },
        TestCase { name: "distance/difference", str1: "distance", str2: "difference", want: 5 },
        TestCase { name: "a cat/an abct", str1: "a cat", str2: "an abct", want: 4 },
        TestCase { name: "こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", want: 3 }, // "Hello" in Japanese
        TestCase { name: "🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", want: 2 },
    ];

    for tt in tests {
        let got = wspace::levenshtein::levenshtein_distance(tt.str1, tt.str2).unwrap() as usize;
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_osa_damerau_levenshtein_distance() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: usize,
    }

    let tests = vec![
        TestCase { name: "First arg empty", str1: "", str2: "abcde", want: 5 },
        TestCase { name: "Second arg empty", str1: "abcde", str2: "", want: 5 },
        TestCase { name: "Same args", str1: "abcde", str2: "abcde", want: 0 },
        TestCase { name: "ab/aa", str1: "ab", str2: "aa", want: 1 },
        TestCase { name: "ab/ba", str1: "ab", str2: "ba", want: 1 },
        TestCase { name: "ab/aaa", str1: "ab", str2: "aaa", want: 2 },
        TestCase { name: "bbb/a", str1: "bbb", str2: "a", want: 3 },
        TestCase { name: "ca/abc", str1: "ca", str2: "abc", want: 3 },
        TestCase { name: "a cat/an abct", str1: "a cat", str2: "an abct", want: 4 },
        TestCase { name: "dixon/dicksonx", str1: "dixon", str2: "dicksonx", want: 4 },
        TestCase { name: "jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", want: 2 },
        TestCase { name: "こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", want: 2 }, // "Hello" in Japanese
        TestCase { name: "🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", want: 2 },
    ];

    for tt in tests {
        let got = wspace::levenshtein::osa_damerau_levenshtein_distance(tt.str1, tt.str2).unwrap() as usize;
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_damerau_levenshtein_distance() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: usize,
    }

    let tests = vec![
        TestCase { name: "First arg empty", str1: "", str2: "abcde", want: 5 },
        TestCase { name: "Second arg empty", str1: "abcde", str2: "", want: 5 },
        TestCase { name: "Same args", str1: "abcde", str2: "abcde", want: 0 },
        TestCase { name: "ab/aa", str1: "ab", str2: "aa", want: 1 },
        TestCase { name: "ab/ba", str1: "ab", str2: "ba", want: 1 },
        TestCase { name: "ab/aaa", str1: "ab", str2: "aaa", want: 2 },
        TestCase { name: "bbb/a", str1: "bbb", str2: "a", want: 3 },
        TestCase { name: "ca/abc", str1: "ca", str2: "abc", want: 2 },
        TestCase { name: "a cat/an abct", str1: "a cat", str2: "an abct", want: 3 },
        TestCase { name: "dixon/dicksonx", str1: "dixon", str2: "dicksonx", want: 4 },
        TestCase { name: "jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", want: 2 },
        TestCase { name: "こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", want: 2 }, // "Hello" in Japanese
        TestCase { name: "🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", want: 2 },
    ];

    for tt in tests {
        let got = wspace::levenshtein::damerau_levenshtein_distance(tt.str1, tt.str2).unwrap() as usize;
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}
