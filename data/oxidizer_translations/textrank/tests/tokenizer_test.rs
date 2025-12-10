use textrank::*;

#[test]
fn test_tokenize_text() {
    let rule = NewRule();

    let text = TokenizeText(
        "This is the right sentence. This sentence without end mark",
        &rule,
    );

    assert_eq!(
        " This sentence without end mark",
        text.GetSentences()[1].GetOriginal(),
    );
}
