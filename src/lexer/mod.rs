use crate::{
    parser::TokenizedLine,
    tokens::{Position, Token},
};

pub mod math;

pub type Result<T> = std::result::Result<T, Error>;
pub type Line = Vec<Token>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Expected an int but was given a {0} at {1}")]
    ExpectedInt(Token, Position),

    #[error("Expected operator but was given a {0} at {1}")]
    ExpectedOperator(Token, Position),

    /// Lexer cannot figure out what a line does
    #[error("Failed to figure out how to lexerize line #{0}!")]
    Unknown(i64),
}

#[derive(Default)]
pub struct Lexer {
    current_line: TokenizedLine,
}

impl Lexer {
    pub fn lexerize(&self, line: TokenizedLine) -> Result<BaseLexingReturn> {
        let math = math::Math;
        let is_valid_line = math.is_valid_line(&line);

        if is_valid_line {
            return math.execute(&line);
        }

        let line_num = line[0].get_pos();
        Err(Error::Unknown(line_num.line_number))
    }
}

#[derive(PartialEq, Debug)]
pub enum BaseLexingReturn {
    Int(i32),
}

/// Trait that has base functions required for each branch of the lexer
pub(super) trait BaseLexing {
    /// Checks if the line is valid for what this branch of the lexer is attempting to accomplish
    /// For example for math we want to check if the meets rules like two numbers on each side of the operator
    fn is_valid_line(&self, line: &Line) -> bool;

    /// Checks if the line is assigning what we are attempting to do to a variable
    fn is_assigning_to_variable(&self, line: &Line) -> bool {
        match (line.get(0), line.get(1)) {
            (Some(Token::MutVarDeclaration(_)), Some(Token::ImmutVarDeclaration(_))) => true,
            _ => false,
        }
    }

    /// Slices out the variable declaration out the line
    fn slice_variable_dec(&self, line: &Line) -> Line {
        let mut new = line.clone();

        for i in (0..3).rev() {
            new.remove(i);
        }

        new
    }

    fn execute(&self, line: &Line) -> Result<BaseLexingReturn>;
}
