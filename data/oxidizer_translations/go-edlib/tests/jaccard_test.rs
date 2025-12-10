// Tests for Jaccard similarity

use wspace::*;

#[test]
fn test_jaccard_similarity() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: f32,
    }

    let tests = vec![
        TestCase { name: "Jaccard sim 1", str1: "Radiohead", str2: "Carly Rae Jepsen", want: 0.0 },
        TestCase { name: "Jaccard sim 2", str1: "I love horror movies", str2: "Lights out is a horror movie", want: 1.0 / 9.0 },
        TestCase { name: "Jaccard sim 3", str1: "love horror movies", str2: "Lights out horror movie", want: 1.0 / 6.0 },
        TestCase { name: "Jaccard sim 4", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", want: 0.0 },
        TestCase { name: "Jaccard sim 5", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", want: 2.0 / 3.0 },
    ];

    for tt in tests {
        let got = wspace::jaccard::jaccard_similarity(tt.str1, tt.str2, 0).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}

#[test]
fn test_jaccard_shingle_similarity() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        want: f32,
    }

    let tests = vec![
        TestCase { name: "Jaccard shingle sim 1", str1: "Radiohead", str2: "Carly Rae Jepsen", want: 0.04761905 },
        TestCase { name: "Jaccard shingle sim 2", str1: "I love horror movies", str2: "Lights out is a horror movie", want: 0.3548387 },
        TestCase { name: "Jaccard shingle sim 3", str1: "love horror movies", str2: "Lights out horror movie", want: 0.44 },
        TestCase { name: "Jaccard shingle sim 4", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", want: 0.61538464 },
        TestCase { name: "Jaccard shingle sim 5", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", want: 0.8 },
    ];

    for tt in tests {
        let got = wspace::jaccard::jaccard_similarity(tt.str1, tt.str2, 2).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}
