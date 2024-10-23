use super::Error;
use super::Result;
use super::Token;
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

    fn get_var_name(&self, line: &Line) -> Result<String> {
        let variable_declaration_position = line.tokenized_body.iter().position(|token| {
            matches!(
                token,
                Token::MutVarDeclaration(_) | Token::ImmutVarDeclaration(_)
            )
        });

        if variable_declaration_position.is_none() {
            return Err(Error::UnableToFindVarName);
        }

        let variable_name_position = variable_declaration_position.unwrap() + 1;
        let variable_name_token = &line.tokenized_body[variable_name_position];

        let variable_name = match variable_name_token {
            Token::Variable(name, _) => name,
            other => return Err(Error::ExpectedVarName(other.clone())),
        };

        Ok(variable_name.to_string())
    }

    fn is_var_mutable(&self, line: &Line) -> bool {
        match line.tokenized_body[0] {
            Token::ImmutVarDeclaration(_) => true,
            Token::MutVarDeclaration(_) => true,
            _ => false,
        }
    }

    /// Slices out the variable declaration out the line
    fn slice_variable_dec(&self, line: &Line) -> LineTokenizedBody {
        let mut new = line.tokenized_body.clone();

        for i in (0..3).rev() {
            new.remove(i);
        }

        new
    }

    fn execute(&self, line: &Line) -> Result<BaseLexingReturn>;
}
