// Tests for Shingle functions

use wspace::*;
use std::collections::HashMap;

#[test]
fn test_shingle() {
    struct TestCase {
        name: &'static str,
        str: &'static str,
        k: i32,
        want: HashMap<String, usize>,
    }

    let mut want1 = HashMap::new();
    want1.insert("Ra".to_string(), 1);
    want1.insert("ad".to_string(), 2);
    want1.insert("di".to_string(), 1);
    want1.insert("ea".to_string(), 1);
    want1.insert("he".to_string(), 1);
    want1.insert("io".to_string(), 1);
    want1.insert("oh".to_string(), 1);

    let mut want2 = HashMap::new();
    want2.insert("Rad".to_string(), 1);
    want2.insert("adi".to_string(), 1);
    want2.insert("dio".to_string(), 1);
    want2.insert("ead".to_string(), 1);
    want2.insert("hea".to_string(), 1);
    want2.insert("ioh".to_string(), 1);
    want2.insert("ohe".to_string(), 1);

    let mut want3 = HashMap::new();
    want3.insert(" h".to_string(), 1);
    want3.insert(" l".to_string(), 1);
    want3.insert(" m".to_string(), 1);
    want3.insert("I ".to_string(), 1);
    want3.insert("e ".to_string(), 1);
    want3.insert("es".to_string(), 1);
    want3.insert("ho".to_string(), 1);
    want3.insert("ie".to_string(), 1);
    want3.insert("lo".to_string(), 1);
    want3.insert("mo".to_string(), 1);
    want3.insert("or".to_string(), 2);
    want3.insert("ov".to_string(), 2);
    want3.insert("r ".to_string(), 1);
    want3.insert("ro".to_string(), 1);
    want3.insert("rr".to_string(), 1);
    want3.insert("ve".to_string(), 1);
    want3.insert("vi".to_string(), 1);

    let mut want4 = HashMap::new();
    want4.insert("です".to_string(), 1);
    want4.insert("の名".to_string(), 1);
    want4.insert("はジ".to_string(), 1);
    want4.insert("ジョ".to_string(), 1);
    want4.insert("ョン".to_string(), 1);
    want4.insert("ンで".to_string(), 1);
    want4.insert("前は".to_string(), 1);
    want4.insert("名前".to_string(), 1);
    want4.insert("私の".to_string(), 1);

    let mut want5 = HashMap::new();
    want5.insert(" 😄".to_string(), 1);
    want5.insert("😄 ".to_string(), 1);
    want5.insert("😄🙂".to_string(), 2);
    want5.insert("🙂😄".to_string(), 3);

    let want6 = HashMap::new();
    let want7 = HashMap::new();

    let mut want8 = HashMap::new();
    want8.insert("四畳半神話大系".to_string(), 1);

    let tests = vec![
        TestCase { name: "shingle 1", str: "Radiohead", k: 2, want: want1 },
        TestCase { name: "shingle 1-1", str: "Radiohead", k: 3, want: want2 },
        TestCase { name: "shingle 2", str: "I love horror movies", k: 2, want: want3 },
        TestCase { name: "shingle 3", str: "私の名前はジョンです", k: 2, want: want4 },
        TestCase { name: "shingle 4", str: "🙂😄🙂😄 😄🙂😄", k: 2, want: want5 },
        TestCase { name: "shingle 5", str: "", k: 100, want: want6 },
        TestCase { name: "shingle 6", str: "hello", k: 0, want: want7 },
        TestCase { name: "shingle 7", str: "四畳半神話大系", k: 7, want: want8 },
    ];

    for tt in tests {
        let got = wspace::shingle::shingle(tt.str, tt.k as usize);
        assert_eq!(got, tt.want, "Test case: {} - Shingle() = {:?}, want {:?}", tt.name, got, tt.want);
    }
}
