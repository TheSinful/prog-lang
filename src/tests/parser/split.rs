use crate::parser::{Error, Parser};
use crate::tokens::Token;

#[test]
fn test_success_split() {
    let mut parser = Parser::default();
    parser.parse("set x = 1".to_string()).unwrap();
    assert_eq!(parser.split_body, vec!["set", "x", "=", "1"]);
}

#[test]
fn test_invalid_token_error() {
    let mut parser = Parser::default();
    let result = parser.parse("invalid_token x = 1".to_string());
    assert!(
        matches!(result, Err(Error::InvalidToken(ref token, ref pos)) if token == "invalid_token" && pos == "0")
    );
}

#[test]
fn test_success_tokenize() {
    let mut parser = Parser::default();
    let base = "set x = 1".to_string();
    parser.parse(base).unwrap();
    let success = vec![
        Token::MutVarDeclaration,
        Token::Variable("x".to_string()),
        Token::Assignment,
        Token::Int(1),
    ];

    assert_eq!(parser.tokenized_body, success)
}
