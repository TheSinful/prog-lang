use crate::processing::{
    parser::Parser,
    syntax_elements::{Position, Token},
};

#[test]
fn test_tokenize() {
    let body = "1 + 1 \n".to_string();
    let mut parser = Parser::default();
    parser.current_line = body;
    let split = parser.split();
    let tokenized = parser.tokenize(&split, 0).unwrap();

    let success: Vec<Token> = vec![
        Token::Int(1, Position::new(0, 0)),
        Token::Add(Position::new(0, 1)),
        Token::Int(1, Position::new(0, 2)),
        Token::Eol(Position::new(0, 3))
    ];

    assert_eq!(tokenized, success);
}