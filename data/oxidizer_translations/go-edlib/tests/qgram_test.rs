// Tests for Q-gram distance

use wspace::*;

#[test]
fn test_qgram_distance() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        split_length: i32,
        want: i32,
    }

    let tests = vec![
        TestCase { name: "Qgram sim 1", str1: "Radiohead", str2: "Radiohead", split_length: 2, want: 0 },
        TestCase { name: "Qgram sim 2", str1: "ABCD", str2: "ABCE", split_length: 2, want: 2 },
        TestCase { name: "Qgram sim 3", str1: "Radiohead", str2: "Carly Rae Jepsen", split_length: 2, want: 21 },
        TestCase { name: "Qgram sim 4", str1: "I love horror movies", str2: "Lights out is a horror movie", split_length: 2, want: 22 },
        TestCase { name: "Qgram sim 5", str1: "love horror movies", str2: "Lights out horror movie", split_length: 2, want: 15 },
        TestCase { name: "Qgram sim 6", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", split_length: 2, want: 5 },
        TestCase { name: "Qgram sim 7", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", split_length: 2, want: 4 },
        TestCase { name: "Qgram sim 8", str1: "", str2: "", split_length: 2, want: 0 },
    ];

    for tt in tests {
        let got = wspace::qgram::qgram_distance(tt.str1, tt.str2, tt.split_length as usize);
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}
