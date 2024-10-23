use crate::processing::lexer;
use crate::processing::lexer::base::BaseLexingReturn;
use crate::processing::lexer::Error;
use crate::processing::parser::{self, Parser};
use crate::processing::syntax_elements::{Position, Token, Variable};
use crate::processing::types::Line;
use crate::processing::utils::variables;
use std::string;

#[test]
fn test_lexer_lexerize_addition() {
    let base = "1 + 1".to_string();
    let mut parser = parser::Parser::default();
    parser.current_line = base.clone();
    let split = parser.split();
    let tokenized = parser.tokenize(&split, 0).unwrap();
    let mut lexer = lexer::Lexer::default();
    let line = Line::new(tokenized, base, split, 0);
    let lexerized = lexer.lexerize(line, &mut parser.variables).unwrap();
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
    let lexerized = lexer.lexerize(line, &mut parser.variables).unwrap();
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
    let lexerized = lexer.lexerize(line, &mut parser.variables).unwrap();
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
    let lexerized = lexer.lexerize(line, &mut parser.variables).unwrap();
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
    let lexerized = lexer.lexerize(line, &mut parser.variables).unwrap();

    let variable = Variable::new::<i32>("x".to_string(), &10, true).unwrap();

    assert_eq!(lexerized, BaseLexingReturn::Variable(variable.clone()));
    assert_eq!(parser.variables[0], variable);
}

#[test]
fn test_lexer_variable_usage() {
    let mut parser = parser::Parser::default();

    // First line: set x = 8 + 2
    let line_content_1 = "set x = 8 + 2".to_string();
    parser.current_line = line_content_1.clone();
    let split_line_1 = parser.split();
    let tokenized_line_1 = parser.tokenize(&split_line_1, 0).unwrap();
    let mut lexer = lexer::Lexer::default();
    let line_1 = Line::new(tokenized_line_1, line_content_1, split_line_1, 0);
    let lexerized_line_1 = lexer.lexerize(line_1, &mut parser.variables).unwrap();

    let variable_x = Variable::new::<i32>("x".to_string(), &10, true).unwrap();

    assert_eq!(
        lexerized_line_1,
        BaseLexingReturn::Variable(variable_x.clone())
    );
    assert_eq!(parser.variables[0], variable_x);

    // Second line: set y = x + 5
    let line_content_2 = "set y = x + 5".to_string();
    parser.current_line = line_content_2.clone();
    let split_line_2 = parser.split();
    let tokenized_line_2 = parser.tokenize(&split_line_2, 1).unwrap();
    let line_2 = Line::new(tokenized_line_2, line_content_2, split_line_2, 1);
    let lexerized_line_2 = lexer.lexerize(line_2, &mut parser.variables).unwrap();
    let variable_y = Variable::new::<i32>("y".to_string(), &15, true).unwrap();

    assert_eq!(
        lexerized_line_2,
        BaseLexingReturn::Variable(variable_y.clone())
    );
    assert_eq!(parser.variables[1], variable_y);
}

#[test]
fn test_duplicate_variable_declaration() {
    let mut parser = parser::Parser::default();
    let mut lexer = lexer::Lexer::default();

    // First line: set x = 8 + 2
    let line_content_1 = "set x = 8 + 2".to_string();
    parser.current_line = line_content_1.clone();
    let split_line_1 = parser.split();
    let tokenized_line_1 = parser.tokenize(&split_line_1, 0).unwrap();
    let line_1 = Line::new(tokenized_line_1, line_content_1, split_line_1, 0);
    let lexerized_line_1 = lexer.lexerize(line_1, &mut parser.variables).unwrap();
    let variable_x = Variable::new::<i32>("x".to_string(), &10, true).unwrap();

    assert_eq!(
        lexerized_line_1,
        BaseLexingReturn::Variable(variable_x.clone())
    );
    assert_eq!(parser.variables[0], variable_x);

    // Second line: set x = 5 + 3 (attempt to declare x again)
    let line_content_2 = "set x = 5 + 3".to_string();
    parser.current_line = line_content_2.clone();
    let split_line_2 = parser.split();
    let tokenized_line_2 = parser.tokenize(&split_line_2, 1).unwrap();
    let line_2 = Line::new(tokenized_line_2, line_content_2, split_line_2, 1);
    let lexerized_line_2 = lexer.lexerize(line_2, &mut parser.variables);
    assert_eq!(
        lexerized_line_2,
        Err(Error::VariableAlreadyExists("x".to_string()))
    );
}
