// Tests for Hamming distance

use wspace::*;

#[test]
fn test_hamming_distance() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: usize,
        want_err: bool,
    }

    let tests = vec![
        TestCase { name: "aa/aa", str1: "aa", str2: "aa", want: 0, want_err: false },
        TestCase { name: "ab/aa", str1: "ab", str2: "aa", want: 1, want_err: false },
        TestCase { name: "ab/ba", str1: "ab", str2: "ba", want: 2, want_err: false },
        TestCase { name: "ab/aaa", str1: "ab", str2: "aaa", want: 0, want_err: true },
        TestCase { name: "bbb/a", str1: "bbb", str2: "a", want: 0, want_err: true },
        TestCase { name: "🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", want: 4, want_err: false },
    ];

    for tt in tests {
        let result = wspace::hamming::hamming_distance(tt.str1, tt.str2);

        if tt.want_err {
            assert!(result.is_err(), "Test case: {} - HammingDistance() expected error", tt.name);
        } else {
            assert!(result.is_ok(), "Test case: {} - HammingDistance() unexpected error: {:?}", tt.name, result.err());
            let got = result.unwrap();
            assert_eq!(got, tt.want, "Test case: {} - HammingDistance() = {}, want {}", tt.name, got, tt.want);
        }
    }
}
