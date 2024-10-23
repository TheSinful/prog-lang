use std::string;

use crate::processing::lexer;
use crate::processing::lexer::base::BaseLexingReturn;
use crate::processing::parser::{self, Error, Parser};
use crate::processing::syntax_elements::{Position, Token, Variable};
use crate::processing::types::Line;

#[test]
fn test_lexer_lexerize_addition() {
    let base = "1 + 1".to_string();
    let mut parser = parser::Parser::default();
    parser.current_line = base.clone();
    let split = parser.split();
    let tokenized = parser.tokenize(&split, 0).unwrap();
    let mut lexer = lexer::Lexer::default();
    let line = Line::new(tokenized, base, split, 0);
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(2), lexerized);
}

#[test]
fn test_lexer_lexerize_subtraction() {
    let mut parser = parser::Parser::default();
    let base = "5 - 3".to_string();
    parser.current_line = base.clone();
    let split = parser.split();
    let tokenized = parser.tokenize(&split, 0).unwrap();
    let mut lexer = lexer::Lexer::default();
    let line = Line::new(tokenized, base, split, 0);
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(2), lexerized);
}

#[test]
fn test_lexer_lexerize_multiplication() {
    let mut parser = parser::Parser::default();
    let base = "2 * 3".to_string();
    parser.current_line = base.clone();
    let split = parser.split();
    let tokenized = parser.tokenize(&split, 0).unwrap();
    let line = Line::new(tokenized, base, split, 0);
    let mut lexer = lexer::Lexer::default();
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(6), lexerized);
}

#[test]
fn test_lexer_lexerize_division() {
    let mut parser = parser::Parser::default();
    let base = "8 / 2".to_string();
    parser.current_line = base.clone();
    let split = parser.split();
    let tokenized = parser.tokenize(&split, 0).unwrap();
    let mut lexer = lexer::Lexer::default();
    let line = Line::new(tokenized, base, split, 0);
    let lexerized = lexer.lexerize(line).unwrap();
    assert_eq!(BaseLexingReturn::Int(4), lexerized);
}

#[test]
fn test_lexer_variable_assignment() {
    let mut parser = parser::Parser::default();
    let base = "set x = 8 + 2".to_string();
    parser.current_line = base.clone();
    let split = parser.split();
    let tokenized = parser.tokenize(&split, 0).unwrap();
    let mut lexer = lexer::Lexer::default();
    let line = Line::new(tokenized, base, split, 0);
    let lexerized = lexer.lexerize(line).unwrap();

    let variable =
        Variable::new::<BaseLexingReturn>("x".to_string(), &BaseLexingReturn::Int(10), true);

    assert_eq!(lexerized, BaseLexingReturn::Variable(variable.clone()));
    assert_eq!(lexer.variables[0], variable);
}
