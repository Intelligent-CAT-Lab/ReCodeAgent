use textrank::*;

#[test]
fn test_language() {
    let mut lang = NewLanguage();

    lang.SetActiveLanguage("hu");
    lang.SetWords("hu", vec!["word1".to_string()]);

    assert_eq!(true, lang.IsStopWord("word1"));
    assert_eq!(false, lang.IsStopWord("word2"));
}
