use super::parser::Parser;
use super::syntax_elements::{Position, Token, Variable};
use super::types::{Line, LineNumber, LineTokenizedBody};
use base::BaseLexing;
use base::BaseLexingReturn;

pub mod base;
pub mod math;
pub mod variables;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Expected an int but was given a {0} at {1}")]
    ExpectedInt(Token, Position),

    #[error("Expected operator but was given a {0} at {1}")]
    ExpectedOperator(Token, Position),

    #[error("Operator in an unexpected position {0}")]
    UnexpectedOperatorPosition(LineNumber),

    #[error("Unable to find operator on line {0}")]
    UnableToFindOperator(LineNumber),

    #[error("Unexpected integer positions on line {0}")]
    UnexpectedIntegerPositions(LineNumber),

    #[error("Expected \"set\" or \"const\" as first token on line {0}")]
    ExpectedVarAssignment(LineNumber),

    // 0 variable name
    // 1 variable position
    #[error("Unable to find value of variable {0}")]
    ExpectedToFindVarValue(String),

    #[error("Unable to find assignment of variable {0}")]
    UnableToFindVarAssignment(String),

    #[error("Unable to find name of variable")]
    UnableToFindVarName,

    #[error("Expected variable name but found {0}")]
    ExpectedVarName(Token),

    /// Lexer cannot figure out what a line does
    #[error("Failed to figure out how to lexerize line #{0}!")]
    Unknown(LineNumber),
}

#[derive(Default)]
pub struct Lexer {
    current_line: Line,
    pub(super) variables: Vec<Variable>,
}

impl Lexer {
    pub fn lexerize(&mut self, line: Line) -> Result<BaseLexingReturn> {
        let math = math::Math;

        match math.is_valid_line(&line) {
            Ok(_) => {
                let execute = math.execute(&line)?;
                match &execute {
                    BaseLexingReturn::Variable(var) => self.variables.push(var.clone()),
                    _ => {}
                }
                return Ok(execute);
            }
            Err(e) => return Err(e),
        }

        Err(Error::Unknown(line.number))
    }
}
