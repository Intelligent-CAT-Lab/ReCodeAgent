// Tests for Sorensen-Dice coefficient

use wspace::*;

#[test]
fn test_sorensen_dice_coefficient() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        split_length: i32,
        want: f32,
    }

    let tests = vec![
        TestCase { name: "SorensenDiceCoefficient 1", str1: "night", str2: "nacht", split_length: 2, want: 0.25 },
        TestCase { name: "SorensenDiceCoefficient 2", str1: "Radiohead", str2: "Radiohead", split_length: 2, want: 1.0 },
        TestCase { name: "SorensenDiceCoefficient 3", str1: "", str2: "", split_length: 2, want: 0.0 },
        TestCase { name: "SorensenDiceCoefficient 4", str1: "Radiohead", str2: "Carly Rae Jepsen", split_length: 2, want: 0.09090909 },
        TestCase { name: "SorensenDiceCoefficient 5", str1: "I love horror movies", str2: "Lights out is a horror movie", split_length: 2, want: 0.52380955 },
        TestCase { name: "SorensenDiceCoefficient 6", str1: "love horror movies", str2: "Lights out horror movie", split_length: 2, want: 0.6111111 },
        TestCase { name: "SorensenDiceCoefficient 7", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", split_length: 2, want: 0.7619048 },
        TestCase { name: "SorensenDiceCoefficient 8", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", split_length: 2, want: 0.8888889 },
    ];

    for tt in tests {
        let got = wspace::sorensen_dice::sorensen_dice_coefficient(tt.str1, tt.str2, tt.split_length as usize).unwrap();
        assert_eq!(got, tt.want, "Test '{}' failed: got {}, want {}", tt.name, got, tt.want);
    }
}
