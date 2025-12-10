use textrank::*;

#[test]
fn test_get_word_data() {
    let mut ranking = NewRank();

    let words = ranking.GetWordData();
    assert_eq!(0, words.len());

    ranking.AddNewWord("word1".to_string(), 0, 1);
    ranking.AddNewWord("word2".to_string(), 1, 1);

    let words = ranking.GetWordData();
    assert_eq!(2, words.len());
}
