// Tests for Cosine similarity

use wspace::*;

// Test data initialized as constants since Rust doesn't have init() functions like Go
const TEST_ARR1: &[&str] = &["a", "b", "d"];
const TEST_ARR2: &[&str] = &["a", "e"];

#[test]
fn test_cosine_similarity() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: f32,
    }

    let tests = vec![
        TestCase { name: "Cosine sim 1", str1: "Radiohead", str2: "Carly Rae Jepsen", want: 0.0 },
        TestCase { name: "Cosine sim 2", str1: "I love horror movies", str2: "Lights out is a horror movie", want: 0.20412414 },
        TestCase { name: "Cosine sim 3", str1: "love horror movies", str2: "Lights out horror movie", want: 0.28867513 },
    ];

    for tt in tests {
        let got = wspace::cosine::cosine_similarity(tt.str1, tt.str2, 0).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_cosine_shingle_similarity() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: f32,
    }

    let tests = vec![
        TestCase { name: "Cosine shingle sim 1", str1: "Radiohead", str2: "Carly Rae Jepsen", want: 0.09759001 },
        TestCase { name: "Cosine shingle sim 2", str1: "I love horror movies", str2: "Lights out is a horror movie", want: 0.5335784 },
        TestCase { name: "Cosine shingle sim 3", str1: "love horror movies", str2: "Lights out horror movie", want: 0.61977977 },
        TestCase { name: "Cosine shingle sim 4", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", want: 0.76980036 },
        TestCase { name: "Cosine shingle sim 5", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", want: 0.8944272 },
    ];

    for tt in tests {
        let got = wspace::cosine::cosine_similarity(tt.str1, tt.str2, 2).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}
#[test]
fn test_union() {
    // Convert test arrays to Vec<String>
    let test_arr1: Vec<String> = TEST_ARR1.iter().map(|s| s.to_string()).collect();
    let test_arr2: Vec<String> = TEST_ARR2.iter().map(|s| s.to_string()).collect();

    let want: Vec<Vec<char>> = vec![vec!['a'], vec!['b'], vec!['d'], vec!['e']];
    let got = wspace::cosine::union(&test_arr1, &test_arr2).unwrap();

    assert_eq!(got, want, "union() failed: got {:?}, want {:?}", got, want);
}

#[test]
fn test_find() {
    struct TestCase {
        name: &'static str,
        slice: Vec<Vec<char>>,
        val: Vec<char>,
        want: i32,
    }

    let tests = vec![
        TestCase {
            name: "Find function test true",
            slice: vec![vec!['a'], vec!['b'], vec!['d'], vec!['e']],
            val: vec!['e'],
            want: 3,
        },
        TestCase {
            name: "Find function test false",
            slice: vec![vec!['a'], vec!['b'], vec!['d'], vec!['e']],
            val: vec!['f'],
            want: -1,
        },
    ];

    for tt in tests {
        let got = wspace::cosine::find(&tt.slice, &tt.val).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_sum() {
    let arr = vec![10, 40, 5, 2, 20];
    let want = 77;
    let got = wspace::cosine::sum(&arr);

    assert_eq!(got, want, "sum() failed: got {}, want {}", got, want);
}

