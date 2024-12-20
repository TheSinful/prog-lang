use std::any::Any;

use super::Error;
use super::Result;
use super::Token;
use crate::processing::syntax_elements::DataTypes;
use crate::processing::syntax_elements::Position;
use crate::processing::syntax_elements::Variable;
use crate::processing::types::Line;
use crate::processing::types::LineTokenizedBody;
use crate::processing::types::VecPosition;

#[derive(PartialEq, Debug, Clone)]
pub enum BaseLexingReturn {
    Int(i32),
    Variable(Variable),
}

/// Trait that has base functions required for each branch of the lexer
pub(super) trait BaseLexing {
    /// Checks if the line is valid for what this branch of the lexer is attempting to accomplish
    /// For example for math we want to check if the meets rules like two numbers on each side of the operator
    fn is_valid_line(&self, line: &Line) -> Result<()>;

    /// Checks if the line is assigning what we are attempting to do to a variable
    fn is_assigning_to_variable(&self, line: &Line) -> bool {
        match line.tokenized_body.first() {
            Some(Token::MutVarDeclaration(_)) => true,
            Some(Token::ImmutVarDeclaration(_)) => true,
            _ => false,
        }
    }

    fn vec_to_string(&self, tokens: Vec<Token>) -> String {
        tokens
            .into_iter()
            .map(|token| token.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }


    fn execute(&self, line: &Line) -> Result<BaseLexingReturn>;
}
