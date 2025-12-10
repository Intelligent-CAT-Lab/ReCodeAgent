// Tests for LCS algorithms

use wspace::*;

#[test]
fn test_lcs() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: usize,
    }

    let tests = vec![
        TestCase { name: "AB/empty", str1: "AB", str2: "", want: 0 },
        TestCase { name: "empty/AB", str1: "", str2: "AB", want: 0 },
        TestCase { name: "AB/AB", str1: "AB", str2: "AB", want: 2 },
        TestCase { name: "ABCD/ACBAD", str1: "ABCD", str2: "ACBAD", want: 3 },
        TestCase { name: "ABCDGH/AEDFHR", str1: "ABCDGH", str2: "AEDFHR", want: 3 },
        TestCase { name: "AGGTAB/GXTXAYB", str1: "AGGTAB", str2: "GXTXAYB", want: 4 },
        TestCase { name: "XMJYAUZ/MZJAWXU", str1: "XMJYAUZ", str2: "MZJAWXU", want: 4 },
    ];

    for tt in tests {
        let got = wspace::lcs::lcs(tt.str1, tt.str2).unwrap() as usize;
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_lcs_backtrack() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: &'static str,
        want_err: bool,
    }

    let tests = vec![
        TestCase { name: "AB/empty", str1: "AB", str2: "", want: "", want_err: true },
        TestCase { name: "empty/AB", str1: "", str2: "AB", want: "", want_err: true },
        TestCase { name: "AB/AB", str1: "AB", str2: "AB", want: "AB", want_err: false },
        TestCase { name: "ABCD/ACBAD", str1: "ABCD", str2: "ACBAD", want: "ABD", want_err: false },
        TestCase { name: "ABCDGH/AEDFHR", str1: "ABCDGH", str2: "AEDFHR", want: "ADH", want_err: false },
        TestCase { name: "AGGTAB/GXTXAYB", str1: "AGGTAB", str2: "GXTXAYB", want: "GTAB", want_err: false },
        TestCase { name: "XMJYAUZ/MZJAWXU", str1: "XMJYAUZ", str2: "MZJAWXU", want: "MJAU", want_err: false },
        TestCase { name: "你好先生/你好夫人", str1: "你好先生", str2: "你好夫人", want: "你好", want_err: false },
    ];

    for tt in tests {
        let result = wspace::lcs::lcs_backtrack(tt.str1, tt.str2);
        if tt.want_err {
            assert!(result.is_err(), "Test '{}' expected error but got: {:?}", tt.name, result);
        } else {
            assert!(result.is_ok(), "Test '{}' expected success but got error: {:?}", tt.name, result);
            let got = result.unwrap();
            assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
        }
    }
}

#[test]
fn test_lcs_backtrack_all() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: Vec<&'static str>,
        want_err: bool,
    }

    let tests = vec![
        TestCase { name: "AB/empty", str1: "AB", str2: "", want: vec![], want_err: true },
        TestCase { name: "empty/AB", str1: "", str2: "AB", want: vec![], want_err: true },
        TestCase { name: "AB/AB", str1: "AB", str2: "AB", want: vec!["AB"], want_err: false },
        TestCase { name: "ABCD/ACBAD", str1: "ABCD", str2: "ACBAD", want: vec!["ABD", "ACD"], want_err: false },
        TestCase { name: "ABCDGH/AEDFHR", str1: "ABCDGH", str2: "AEDFHR", want: vec!["ADH"], want_err: false },
        TestCase { name: "AGGTAB/GXTXAYB", str1: "AGGTAB", str2: "GXTXAYB", want: vec!["GTAB"], want_err: false },
        TestCase { name: "XMJYAUZ/MZJAWXU", str1: "XMJYAUZ", str2: "MZJAWXU", want: vec!["MJAU"], want_err: false },
        TestCase { name: "AZBYCWDX/ZAYBWCXD", str1: "AZBYCWDX", str2: "ZAYBWCXD", want: vec!["ABCD", "ABCX", "ABWD", "ABWX", "AYCD", "AYCX", "AYWD", "AYWX", "ZBCD", "ZBCX", "ZBWD", "ZBWX", "ZYCD", "ZYCX", "ZYWD", "ZYWX"], want_err: false },
        TestCase { name: "AATCC/ACACG", str1: "AATCC", str2: "ACACG", want: vec!["AAC", "ACC"], want_err: false },
        TestCase { name: "您好女士，你好吗？/先生，你好吗？", str1: "您好女士 你好吗？", str2: "先生 你好吗？", want: vec![" 你好吗？"], want_err: false },
        TestCase { name: " 是ab是cde22f123g/222222是ab是cd123", str1: " 是ab是cde22f123g", str2: "222222是ab是cd123", want: vec!["是ab是cd123"], want_err: false },
    ];

    for tt in tests {
        let result = wspace::lcs::lcs_backtrack_all(tt.str1, tt.str2);
        if tt.want_err {
            assert!(result.is_err(), "Test '{}' expected error but got: {:?}", tt.name, result);
        } else {
            assert!(result.is_ok(), "Test '{}' expected success but got error: {:?}", tt.name, result);
            let mut got = result.unwrap();
            got.sort();
            let mut want: Vec<String> = tt.want.iter().map(|s| s.to_string()).collect();
            want.sort();
            assert_eq!(got, want, "Test '{}' failed: got {:?}, want {:?}", tt.name, got, want);
        }
    }
}

#[test]
fn test_lcs_diff() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: Vec<&'static str>,
        want_err: bool,
    }

    let tests = vec![
        TestCase { name: "AB/empty", str1: "AB", str2: "", want: vec![], want_err: true },
        TestCase { name: "empty/AB", str1: "", str2: "AB", want: vec![], want_err: true },
        TestCase { name: "AB/AB", str1: "AB", str2: "AB", want: vec!["AB"], want_err: false },
        TestCase { name: "computer/houseboat", str1: "computer", str2: "houseboat", want: vec![" h c o m p u s e b o a t e r", " + -   - -   + + + + +   - -"], want_err: false },
        TestCase { name: "您好女士，你好吗？/先生，你好吗？", str1: "您好女士 你好吗？", str2: "先生 你好吗？", want: vec![" 先 生 您 好 女 士   你 好 吗 ？", " + + - - - -          "], want_err: false },
    ];

    for tt in tests {
        let result = wspace::lcs::lcs_diff(tt.str1, tt.str2);
        if tt.want_err {
            assert!(result.is_err(), "Test '{}' expected error but got: {:?}", tt.name, result);
        } else {
            assert!(result.is_ok(), "Test '{}' expected success but got error: {:?}", tt.name, result);
            let got = result.unwrap();
            let want: Vec<String> = tt.want.iter().map(|s| s.to_string()).collect();
            assert_eq!(got, want, "Test '{}' failed: got {:?}, want {:?}", tt.name, got, want);
        }
    }
}

#[test]
fn test_lcs_edit_distance() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: usize,
    }

    let tests = vec![
        TestCase { name: "AB/empty", str1: "AB", str2: "", want: 2 },
        TestCase { name: "empty/AB", str1: "", str2: "AB", want: 2 },
        TestCase { name: "No characters match", str1: "abcd", str2: "effgghh", want: 11 },
        TestCase { name: "AB/AB", str1: "AB", str2: "AB", want: 0 },
        TestCase { name: "CAT/CUT", str1: "CAT", str2: "CUT", want: 2 },
        TestCase { name: "ACB/AB", str1: "ACB", str2: "AB", want: 1 },
        TestCase { name: "ABC/ACD", str1: "ABC", str2: "ACD", want: 2 },
        TestCase { name: "ABCD/ACBAD", str1: "ABCD", str2: "ACBAD", want: 3 },
        TestCase { name: "ABCDGH/AEDFHR", str1: "ABCDGH", str2: "AEDFHR", want: 6 },
        TestCase { name: "AGGTAB/GXTXAYB", str1: "AGGTAB", str2: "GXTXAYB", want: 5 },
        TestCase { name: "XMJYAUZ/MZJAWXU", str1: "XMJYAUZ", str2: "MZJAWXU", want: 6 },
    ];

    for tt in tests {
        let got = wspace::lcs::lcs_edit_distance(tt.str1, tt.str2).unwrap() as usize;
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}
