
use Remimu::my_regex::{
    regex_match,
    print_regex_tokens, 
    regex_parse, 
    RegexToken
};
#[test]
fn test() {
    let mut tokens = vec![RegexToken::default(); 1024];
    let mut token_count: i16 = 1024;

    // Parse the regex pattern
    let result = regex_parse("[0-9]+\\.[0-9]+", &mut tokens, &mut token_count, 0);
    assert!(result == Err(0), "Regex parsing failed");
    print_regex_tokens(&tokens);

    // Test matching
    let mut cap_pos = [0i64; 10];  // Example size
    let mut cap_span = [0i64; 10]; // Example size

    let match_len = regex_match(&tokens, "23.53) ", 0, 0u16, &mut cap_pos[..], &mut cap_span[..]);
    println!("########### return: {:?}", match_len);
}
fn main(){}