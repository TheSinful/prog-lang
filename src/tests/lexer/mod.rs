use crate::lexer::{self, BaseLexingReturn};
use crate::parser::{self, Error, Parser};
use crate::tokens::{Position, Token};

#[test]
fn test_lexer_lexerize_addition() {
    let mut parser = parser::Parser::default();
    let base = "1 + 1";
    parser.current_line = base.to_string();
    let split = parser.split();
    let line = parser.tokenize(split, 0).unwrap();
    let lexer = lexer::Lexer::default();
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(2), lexerized);
}

#[test]
fn test_lexer_lexerize_subtraction() {
    let mut parser = parser::Parser::default();
    let base = "5 - 3";
    parser.current_line = base.to_string();
    let split = parser.split();
    let line = parser.tokenize(split, 0).unwrap();
    let lexer = lexer::Lexer::default();
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(2), lexerized);
}

#[test]
fn test_lexer_lexerize_multiplication() {
    let mut parser = parser::Parser::default();
    let base = "2 * 3";
    parser.current_line = base.to_string();
    let split = parser.split();
    let line = parser.tokenize(split, 0).unwrap();
    let lexer = lexer::Lexer::default();
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(6), lexerized);
}

#[test]
fn test_lexer_lexerize_division() {
    let mut parser = parser::Parser::default();
    let base = "8 / 2";
    parser.current_line = base.to_string();
    let split = parser.split();
    let line = parser.tokenize(split, 0).unwrap();
    let lexer = lexer::Lexer::default();
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(4), lexerized);
}
