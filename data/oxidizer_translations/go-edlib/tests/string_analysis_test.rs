// Tests for string analysis API

use wspace::*;
use wspace::string_analysis::*;

// Test data initialized as constants since Rust doesn't have init() functions like Go
const STR_LIST: &[&str] = &[
    "test",
    "tester",
    "tests",
    "testers",
    "testing",
    "tsting",
    "sting",
];

#[test]
fn test_strings_similarity() {
    struct TestCase {
        name: &'static str,
        str1: &'static str,
        str2: &'static str,
        algo: Algorithm,
        want: f32,
        want_err: bool,
    }

    let tests = vec![
        // Levenshtein method
        TestCase { name: "Levenshtein : First arg empty", str1: "", str2: "abcde", algo: LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "Levenshtein : Second arg empty", str1: "abcde", str2: "", algo: LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "Levenshtein : Same args", str1: "abcde", str2: "abcde", algo: LEVENSHTEIN, want: 1.0, want_err: false },
        TestCase { name: "Levenshtein : No characters match", str1: "abcd", str2: "effgghh", algo: LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "Levenshtein : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: LEVENSHTEIN, want: 0.6, want_err: false },
        TestCase { name: "Levenshtein : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: LEVENSHTEIN, want: 0.6666667, want_err: false },
        TestCase { name: "Levenshtein : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: LEVENSHTEIN, want: 0.50, want_err: false },
        TestCase { name: "Levenshtein : jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", algo: LEVENSHTEIN, want: 0.80, want_err: false },
        TestCase { name: "Levenshtein : abcde/бвгдж", str1: "abcde", str2: "бвгдж", algo: LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "Levenshtein : abcde/fghjk", str1: "abcde", str2: "fghjk", algo: LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "Levenshtein : こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", algo: LEVENSHTEIN, want: 0.4, want_err: false },
        TestCase { name: "Levenshtein : 🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", algo: LEVENSHTEIN, want: 0.5, want_err: false },

        // DamerauLevenshtein method
        TestCase { name: "DamerauLevenshtein : First arg empty", str1: "", str2: "abcde", algo: DAMERAU_LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "DamerauLevenshtein : Second arg empty", str1: "abcde", str2: "", algo: DAMERAU_LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "DamerauLevenshtein : Same args", str1: "abcde", str2: "abcde", algo: DAMERAU_LEVENSHTEIN, want: 1.0, want_err: false },
        TestCase { name: "DamerauLevenshtein : No characters match", str1: "abcd", str2: "effgghh", algo: DAMERAU_LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "DamerauLevenshtein : ab/aaa", str1: "ab", str2: "aaa", algo: DAMERAU_LEVENSHTEIN, want: 0.33333334, want_err: false },
        TestCase { name: "DamerauLevenshtein : bbb/a", str1: "bbb", str2: "a", algo: DAMERAU_LEVENSHTEIN, want: 0.0, want_err: false },
        TestCase { name: "DamerauLevenshtein : ca/abc", str1: "ca", str2: "abc", algo: DAMERAU_LEVENSHTEIN, want: 0.33333334, want_err: false },
        TestCase { name: "DamerauLevenshtein : a cat/an abct", str1: "a cat", str2: "an abct", algo: DAMERAU_LEVENSHTEIN, want: 0.5714286, want_err: false },
        TestCase { name: "DamerauLevenshtein : dixon/dicksonx", str1: "dixon", str2: "dicksonx", algo: DAMERAU_LEVENSHTEIN, want: 0.5, want_err: false },
        TestCase { name: "DamerauLevenshtein : jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", algo: DAMERAU_LEVENSHTEIN, want: 0.8, want_err: false },
        TestCase { name: "DamerauLevenshtein : こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", algo: DAMERAU_LEVENSHTEIN, want: 0.6, want_err: false },
        TestCase { name: "DamerauLevenshtein : 🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", algo: DAMERAU_LEVENSHTEIN, want: 0.5, want_err: false },

        // OSADamerauLevenshtein method
        TestCase { name: "OSADamerauLevenshtein : First arg empty", str1: "", str2: "abcde", algo: OSADamerauLevenshtein, want: 0.0, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : Second arg empty", str1: "abcde", str2: "", algo: OSADamerauLevenshtein, want: 0.0, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : Same args", str1: "abcde", str2: "abcde", algo: OSADamerauLevenshtein, want: 1.0, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : No characters match", str1: "abcd", str2: "effgghh", algo: OSADamerauLevenshtein, want: 0.0, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : ab/aaa", str1: "ab", str2: "aaa", algo: OSADamerauLevenshtein, want: 0.33333334, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : bbb/a", str1: "bbb", str2: "a", algo: OSADamerauLevenshtein, want: 0.0, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : ca/abc", str1: "ca", str2: "abc", algo: OSADamerauLevenshtein, want: 0.0, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : a cat/an abct", str1: "a cat", str2: "an abct", algo: OSADamerauLevenshtein, want: 0.428571429, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : dixon/dicksonx", str1: "dixon", str2: "dicksonx", algo: OSADamerauLevenshtein, want: 0.5, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", algo: OSADamerauLevenshtein, want: 0.8, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", algo: OSADamerauLevenshtein, want: 0.6, want_err: false },
        TestCase { name: "OSADamerauLevenshtein : 🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", algo: OSADamerauLevenshtein, want: 0.5, want_err: false },

        // Lcs method
        TestCase { name: "LCS : First arg empty", str1: "", str2: "abcde", algo: Lcs, want: 0.0, want_err: false },
        TestCase { name: "LCS : Second arg empty", str1: "abcde", str2: "", algo: Lcs, want: 0.0, want_err: false },
        TestCase { name: "LCS : Same args", str1: "abcde", str2: "abcde", algo: Lcs, want: 1.0, want_err: false },
        TestCase { name: "LCS : ABCDGH/AEDFHR", str1: "ABCDGH", str2: "AEDFHR", algo: Lcs, want: 0.0, want_err: false },
        TestCase { name: "LCS : AGGTAB/GXTXAYB", str1: "AGGTAB", str2: "GXTXAYB", algo: Lcs, want: 0.2857143, want_err: false },
        TestCase { name: "LCS : XMJYAUZ/MZJAWXU", str1: "XMJYAUZ", str2: "MZJAWXU", algo: Lcs, want: 0.14285715, want_err: false },
        TestCase { name: "LCS : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: Lcs, want: 0.2, want_err: false },
        TestCase { name: "LCS : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: Lcs, want: 0.6666667, want_err: false },
        TestCase { name: "LCS : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: Lcs, want: 0.375, want_err: false },
        TestCase { name: "LCS : jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", algo: Lcs, want: 0.7, want_err: false },
        TestCase { name: "Lcs : こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", algo: Lcs, want: 0.4, want_err: false },
        TestCase { name: "Lcs : 🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", algo: Lcs, want: 0.5, want_err: false },

        // Hamming method
        TestCase { name: "Hamming : First arg empty", str1: "", str2: "abcde", algo: Hamming, want: 0.0, want_err: true },
        TestCase { name: "Hamming : Second arg empty", str1: "abcde", str2: "", algo: Hamming, want: 0.0, want_err: true },
        TestCase { name: "Hamming : Same args", str1: "abcde", str2: "abcde", algo: Hamming, want: 1.0, want_err: false },
        TestCase { name: "Hamming : No characters match", str1: "abcd", str2: "effgghh", algo: Hamming, want: 0.0, want_err: true },
        TestCase { name: "Hamming : aa/aa", str1: "aa", str2: "aa", algo: Hamming, want: 1.0, want_err: false },
        TestCase { name: "Hamming : ab/aa", str1: "ab", str2: "aa", algo: Hamming, want: 0.5, want_err: false },
        TestCase { name: "Hamming : ab/ba", str1: "ab", str2: "ba", algo: Hamming, want: 0.0, want_err: false },
        TestCase { name: "Hamming : a cat/an abct", str1: "a cat", str2: "an abct", algo: Hamming, want: 0.0, want_err: true },
        TestCase { name: "Hamming : dixon/dicksonx", str1: "dixon", str2: "dicksonx", algo: Hamming, want: 0.0, want_err: true },
        TestCase { name: "Hamming : jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", algo: Hamming, want: 0.0, want_err: true },
        TestCase { name: "Hamming : こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", algo: Hamming, want: 0.0, want_err: true },
        TestCase { name: "Hamming : 🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", algo: Hamming, want: 0.0, want_err: false },

        // Jaro method
        TestCase { name: "Jaro : First arg empty", str1: "", str2: "abcde", algo: Jaro, want: 0.0, want_err: false },
        TestCase { name: "Jaro : Second arg empty", str1: "abcde", str2: "", algo: Jaro, want: 0.0, want_err: false },
        TestCase { name: "Jaro : Same args", str1: "abcde", str2: "abcde", algo: Jaro, want: 1.0, want_err: false },
        TestCase { name: "Jaro : No characters match", str1: "abcd", str2: "effgghh", algo: Jaro, want: 0.0, want_err: false },
        TestCase { name: "Jaro : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: Jaro, want: 0.73333335, want_err: false },
        TestCase { name: "Jaro : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: Jaro, want: 0.9444444, want_err: false },
        TestCase { name: "Jaro : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: Jaro, want: 0.76666665, want_err: false },
        TestCase { name: "Jaro : jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", algo: Jaro, want: 0.8962963, want_err: false },
        TestCase { name: "Jaro : こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", algo: Jaro, want: 0.84999996, want_err: false },
        TestCase { name: "Jaro : こんににんち/こんにちは", str1: "こんににんち", str2: "こんにちは", algo: Jaro, want: 0.82222223, want_err: false },
        TestCase { name: "Jaro : 🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", algo: Jaro, want: 0.8333333, want_err: false },

        // JaroWinkler method
        TestCase { name: "JaroWinkler : First arg empty", str1: "", str2: "abcde", algo: JARO_WINKLER, want: 0.0, want_err: false },
        TestCase { name: "JaroWinkler : Second arg empty", str1: "abcde", str2: "", algo: JARO_WINKLER, want: 0.0, want_err: false },
        TestCase { name: "JaroWinkler : Same args", str1: "abcde", str2: "abcde", algo: JARO_WINKLER, want: 1.0, want_err: false },
        TestCase { name: "JaroWinkler : No characters match", str1: "abcd", str2: "effgghh", algo: JARO_WINKLER, want: 0.0, want_err: false },
        TestCase { name: "JaroWinkler : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: JARO_WINKLER, want: 0.73333335, want_err: false },
        TestCase { name: "JaroWinkler : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: JARO_WINKLER, want: 0.96111107, want_err: false },
        TestCase { name: "JaroWinkler : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: JARO_WINKLER, want: 0.81333333, want_err: false },
        TestCase { name: "JaroWinkler : jellyfish/smellyfish", str1: "jellyfish", str2: "smellyfish", algo: JARO_WINKLER, want: 0.8962963, want_err: false },
        TestCase { name: "JaroWinkler : こにんち/こんにちは", str1: "こにんち", str2: "こんにちは", algo: JARO_WINKLER, want: 0.86499995, want_err: false },
        TestCase { name: "JaroWinkler : こんににんち/こんにちは", str1: "こんににんち", str2: "こんにちは", algo: JARO_WINKLER, want: 0.8755556, want_err: false },
        TestCase { name: "JaroWinkler : 🙂😄🙂😄/😄🙂😄🙂", str1: "🙂😄🙂😄", str2: "😄🙂😄🙂", algo: JARO_WINKLER, want: 0.8333333, want_err: false },

        // Cosine method
        TestCase { name: "Cosine : First arg empty", str1: "", str2: "abcde", algo: COSINE, want: 0.0, want_err: false },
        TestCase { name: "Cosine : Second arg empty", str1: "abcde", str2: "", algo: COSINE, want: 0.0, want_err: false },
        TestCase { name: "Cosine : Same args", str1: "abcde", str2: "abcde", algo: COSINE, want: 1.0, want_err: false },
        TestCase { name: "Cosine : No characters match", str1: "abcd", str2: "effgghh", algo: COSINE, want: 0.0, want_err: false },
        TestCase { name: "Cosine : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: COSINE, want: 0.25, want_err: false },
        TestCase { name: "Cosine : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: COSINE, want: 0.4, want_err: false },
        TestCase { name: "Cosine : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: COSINE, want: 0.3779645, want_err: false },
        TestCase { name: "Cosine : Sentence 1", str1: "Radiohead", str2: "Carly Rae Jepsen", algo: COSINE, want: 0.09759001, want_err: false },
        TestCase { name: "Cosine : Sentence 2", str1: "I love horror movies", str2: "Lights out is a horror movie", algo: COSINE, want: 0.5335784, want_err: false },
        TestCase { name: "Cosine : Sentence 3", str1: "love horror movies", str2: "Lights out horror movie", algo: COSINE, want: 0.61977977, want_err: false },

        // Jaccard method
        TestCase { name: "Jaccard : First arg empty", str1: "", str2: "abcde", algo: JACCARD, want: 0.0, want_err: false },
        TestCase { name: "Jaccard : Second arg empty", str1: "abcde", str2: "", algo: JACCARD, want: 0.0, want_err: false },
        TestCase { name: "Jaccard : Same args", str1: "abcde", str2: "abcde", algo: JACCARD, want: 1.0, want_err: false },
        TestCase { name: "Jaccard : No characters match", str1: "abcd", str2: "effgghh", algo: JACCARD, want: 0.0, want_err: false },
        TestCase { name: "Jaccard : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: JACCARD, want: 0.14285715, want_err: false },
        TestCase { name: "Jaccard : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: JACCARD, want: 0.25, want_err: false },
        TestCase { name: "Jaccard : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: JACCARD, want: 0.22222222, want_err: false },
        TestCase { name: "Jaccard : Sentence 1", str1: "Radiohead", str2: "Carly Rae Jepsen", algo: JACCARD, want: 0.04761905, want_err: false },
        TestCase { name: "Jaccard : Sentence 2", str1: "I love horror movies", str2: "Lights out is a horror movie", algo: JACCARD, want: 0.3548387, want_err: false },
        TestCase { name: "Jaccard : Sentence 3", str1: "love horror movies", str2: "Lights out horror movie", algo: JACCARD, want: 0.44, want_err: false },
        TestCase { name: "Jaccard : Sentence 4", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", algo: JACCARD, want: 0.61538464, want_err: false },
        TestCase { name: "Jaccard : Sentence 5", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", algo: JACCARD, want: 0.8, want_err: false },

        // SorensenDice method
        TestCase { name: "SorensenDice : First arg empty", str1: "", str2: "abcde", algo: SorensenDice, want: 0.0, want_err: false },
        TestCase { name: "SorensenDice : Second arg empty", str1: "abcde", str2: "", algo: SorensenDice, want: 0.0, want_err: false },
        TestCase { name: "SorensenDice : Same args", str1: "abcde", str2: "abcde", algo: SorensenDice, want: 1.0, want_err: false },
        TestCase { name: "SorensenDice : No characters match", str1: "abcd", str2: "effgghh", algo: SorensenDice, want: 0.0, want_err: false },
        TestCase { name: "SorensenDice : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: SorensenDice, want: 0.25, want_err: false },
        TestCase { name: "SorensenDice : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: SorensenDice, want: 0.4, want_err: false },
        TestCase { name: "SorensenDice : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: SorensenDice, want: 0.36363637, want_err: false },
        TestCase { name: "SorensenDice Sentence 1", str1: "night", str2: "nacht", algo: SorensenDice, want: 0.25, want_err: false },
        TestCase { name: "SorensenDice Sentence 2", str1: "Radiohead", str2: "Radiohead", algo: SorensenDice, want: 1.0, want_err: false },
        TestCase { name: "SorensenDice Sentence 3", str1: "", str2: "", algo: SorensenDice, want: 0.0, want_err: false },
        TestCase { name: "SorensenDice Sentence 4", str1: "Radiohead", str2: "Carly Rae Jepsen", algo: SorensenDice, want: 0.09090909, want_err: false },
        TestCase { name: "SorensenDice Sentence 5", str1: "I love horror movies", str2: "Lights out is a horror movie", algo: SorensenDice, want: 0.52380955, want_err: false },
        TestCase { name: "SorensenDice Sentence 6", str1: "love horror movies", str2: "Lights out horror movie", algo: SorensenDice, want: 0.6111111, want_err: false },
        TestCase { name: "SorensenDice Sentence 7", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", algo: SorensenDice, want: 0.7619048, want_err: false },
        TestCase { name: "SorensenDice Sentence 8", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", algo: SorensenDice, want: 0.8888889, want_err: false },

        // Qgram method
        TestCase { name: "Qgram: First arg empty", str1: "", str2: "abcde", algo: QGRAM, want: 0.0, want_err: false },
        TestCase { name: "Qgram : Second arg empty", str1: "abcde", str2: "", algo: QGRAM, want: 0.0, want_err: false },
        TestCase { name: "Qgram : Same args", str1: "abcde", str2: "abcde", algo: QGRAM, want: 1.0, want_err: false },
        TestCase { name: "Qgram : No characters match", str1: "abcd", str2: "effgghh", algo: QGRAM, want: 0.0, want_err: false },
        TestCase { name: "Qgram : CRATE/TRACE", str1: "CRATE", str2: "TRACE", algo: QGRAM, want: 0.25, want_err: false },
        TestCase { name: "Qgram : MARTHA/MARHTA", str1: "MARTHA", str2: "MARHTA", algo: QGRAM, want: 0.39999998, want_err: false },
        TestCase { name: "Qgram : DIXON/DICKSONX", str1: "DIXON", str2: "DICKSONX", algo: QGRAM, want: 0.36363637, want_err: false },
        TestCase { name: "Qgram Sentence 1", str1: "Radiohead", str2: "Radiohead", algo: QGRAM, want: 1.0, want_err: false },
        TestCase { name: "Qgram Sentence 2", str1: "ABCD", str2: "ABCE", algo: QGRAM, want: 0.6666666, want_err: false },
        TestCase { name: "Qgram Sentence 3", str1: "Radiohead", str2: "Carly Rae Jepsen", algo: QGRAM, want: 0.04545456, want_err: false },
        TestCase { name: "Qgram Sentence 4", str1: "I love horror movies", str2: "Lights out is a horror movie", algo: QGRAM, want: 0.47619045, want_err: false },
        TestCase { name: "Qgram Sentence 5", str1: "love horror movies", str2: "Lights out horror movie", algo: QGRAM, want: 0.5833334, want_err: false },
        TestCase { name: "Qgram Sentence 6", str1: "私の名前はジョンです", str2: "私の名前はジョン・ドゥです", algo: QGRAM, want: 0.7619048, want_err: false },
        TestCase { name: "Qgram Sentence 7", str1: "🙂😄🙂😄 😄🙂😄", str2: "🙂😄🙂😄 😄🙂😄 🙂😄🙂", algo: QGRAM, want: 0.5555556, want_err: false },

        // Note: Can't easily test invalid algorithm in Rust's type-safe enum system
    ];

    for tt in tests {
        let result = wspace::string_analysis::strings_similarity(tt.str1, tt.str2, tt.algo);
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
fn test_fuzzy_search() {
    struct TestCase {
        name: &'static str,
        str: &'static str,
        str_list: Vec<String>,
        algo: Algorithm,
        want: &'static str,
        want_err: bool,
    }

    let str_list: Vec<String> = STR_LIST.iter().map(|s| s.to_string()).collect();

    let tests = vec![
        TestCase { name: "FuzzySearch 'testing'", str: "testnig", str_list: str_list.clone(), algo: LEVENSHTEIN, want: "testing", want_err: false },
        TestCase { name: "FuzzySearch 'testing'", str: "test", str_list: str_list.clone(), algo: LEVENSHTEIN, want: "test", want_err: false },
        TestCase { name: "FuzzySearch 'testing' err", str: "testnig", str_list: str_list.clone(), algo: Hamming, want: "", want_err: true },
    ];

    for tt in tests {
        let result = wspace::string_analysis::fuzzy_search(tt.str, &tt.str_list, tt.algo);
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
fn test_fuzzy_search_threshold() {
    struct TestCase {
        name: &'static str,
        str: &'static str,
        str_list: Vec<String>,
        min_sim: f32,
        algo: Algorithm,
        want: &'static str,
        want_err: bool,
    }

    let str_list: Vec<String> = STR_LIST.iter().map(|s| s.to_string()).collect();

    let tests = vec![
        TestCase { name: "FuzzySearch 'testing'", str: "testnig", str_list: str_list.clone(), min_sim: 0.7, algo: LEVENSHTEIN, want: "testing", want_err: false },
        TestCase { name: "FuzzySearch 'testing'", str: "test", str_list: str_list.clone(), min_sim: 0.7, algo: LEVENSHTEIN, want: "test", want_err: false },
        TestCase { name: "FuzzySearch 'testing'", str: "hello", str_list: str_list.clone(), min_sim: 0.7, algo: LEVENSHTEIN, want: "", want_err: false },
        TestCase { name: "FuzzySearch 'testing' err", str: "testing", str_list: str_list.clone(), min_sim: 0.7, algo: Hamming, want: "", want_err: true },
    ];

    for tt in tests {
        let result = wspace::string_analysis::fuzzy_search_threshold(tt.str, &tt.str_list, tt.min_sim, tt.algo);
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
fn test_fuzzy_search_set() {
    struct TestCase {
        name: &'static str,
        str: &'static str,
        str_list: Vec<String>,
        quantity: usize,
        algo: Algorithm,
        want: Vec<&'static str>,
        want_err: bool,
    }

    let str_list: Vec<String> = STR_LIST.iter().map(|s| s.to_string()).collect();

    let tests = vec![
        TestCase { name: "FuzzySearch 'testing'", str: "testnig", str_list: str_list.clone(), quantity: 3, algo: LEVENSHTEIN, want: vec!["testing", "test", "tester"], want_err: false },
        TestCase { name: "FuzzySearch 'testing' err", str: "testnig", str_list: str_list.clone(), quantity: 3, algo: Hamming, want: vec![], want_err: true },
    ];

    for tt in tests {
        let result = wspace::string_analysis::fuzzy_search_set(tt.str, &tt.str_list, tt.quantity, tt.algo);
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
fn test_fuzzy_search_set_threshold() {
    struct TestCase {
        name: &'static str,
        str: &'static str,
        str_list: Vec<String>,
        quantity: usize,
        min_sim: f32,
        algo: Algorithm,
        want: Vec<&'static str>,
        want_err: bool,
    }

    let str_list: Vec<String> = STR_LIST.iter().map(|s| s.to_string()).collect();

    let tests = vec![
        TestCase { name: "FuzzySearch 'testing'", str: "testnig", str_list: str_list.clone(), quantity: 3, min_sim: 0.7, algo: LEVENSHTEIN, want: vec!["testing", "", ""], want_err: false },
        TestCase { name: "FuzzySearch 'testing'", str: "testnig", str_list: str_list.clone(), quantity: 3, min_sim: 0.5, algo: LEVENSHTEIN, want: vec!["testing", "test", "tester"], want_err: false },
        TestCase { name: "FuzzySearch 'testing' err", str: "testnig", str_list: str_list.clone(), quantity: 3, min_sim: 0.7, algo: Hamming, want: vec![], want_err: true },
    ];

    for tt in tests {
        let result = wspace::string_analysis::fuzzy_search_set_threshold(tt.str, &tt.str_list, tt.quantity, tt.min_sim, tt.algo);
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

