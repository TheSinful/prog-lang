use super::Result;
use super::Token;
use crate::processing::types::Line;

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
        matches!(
            (line.first(), line.get(1)),
            (
                Some(Token::MutVarDeclaration(_)),
                Some(Token::ImmutVarDeclaration(_))
            )
        )
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
