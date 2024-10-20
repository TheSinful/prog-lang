use super::parser::Parser;
use super::syntax_elements::{Position, Token};
use super::types::TokenizedLine;
use base::BaseLexing;
use base::BaseLexingReturn;

pub mod base;
pub mod math;

pub type Result<T> = std::result::Result<T, Error>;

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
