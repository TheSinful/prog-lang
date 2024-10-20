use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub line_number: i64,
    pub line_position: i64,
}

impl Position {
    pub fn new(line_number: i64, line_position: i64) -> Position {
        Position {
            line_number: line_number,
            line_position: line_position,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line: {}, Char: {}",
            self.line_number, self.line_position
        )
    }
}

pub type VariableName = String;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Eol(Position),                 // end of line
    Add(Position),                 // +
    Subtract(Position),            // -
    Divide(Position),              // "/"
    Multiply(Position),            // "*"
    SingleComment(Position),       // "//"
    MutVarDeclaration(Position),   // "set"
    ImmutVarDeclaration(Position), // "const"
    Assignment(Position),          // =
    Int(i32, Position),
    Variable(VariableName, Position),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Token::Eol(p) => write!(f, "end of line pos: {}", p),
            Token::Add(p) => write!(f, "+ pos: {}", p),
            Token::Subtract(p) => write!(f, "- pos: {}", p),
            Token::Divide(p) => write!(f, "/ pos: {}", p),
            Token::Multiply(p) => write!(f, "* pos: {}", p),
            Token::SingleComment(p) => write!(f, "// pos: {}", p),
            Token::MutVarDeclaration(p) => write!(f, "set pos: {}", p),
            Token::ImmutVarDeclaration(p) => write!(f, "const pos: {}", p),
            Token::Assignment(p) => write!(f, "= pos: {}", p),
            Token::Int(value, p) => write!(f, "{} pos: {}", value, p),
            Token::Variable(name, p) => write!(f, "{} pos: {}", name, p),
        }
    }
}

impl Token {
    pub fn get_pos(&self) -> Position {
        match self {
            Token::Add(pos) => pos.clone(),
            Token::Eol(pos) => pos.clone(),
            Token::Subtract(pos) => pos.clone(),
            Token::Divide(pos) => pos.clone(),
            Token::Multiply(pos) => pos.clone(),
            Token::SingleComment(pos) => pos.clone(),
            Token::MutVarDeclaration(pos) => pos.clone(),
            Token::ImmutVarDeclaration(pos) => pos.clone(),
            Token::Assignment(pos) => pos.clone(),
            Token::Int(_, pos) => pos.clone(),
            Token::Variable(_, pos) => pos.clone(),
        }
    }
}

// set x =
