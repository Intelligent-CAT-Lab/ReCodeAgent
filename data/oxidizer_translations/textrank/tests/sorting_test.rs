use textrank::*;

#[test]
fn test_find_sentences() {
    let ranking = NewRank();

    let sentences = FindSentences(&ranking, 999, 1);

    assert_eq!(0, sentences.len());
}
