use crate::{parser::TokenizedBody, tokens::Token};

pub mod math;

pub type Result<T> = std::result::Result<T, Error>;
pub type Line = Vec<Token>;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[derive(Default)]
pub struct Lexer {
    base_tokenized: TokenizedBody,
    split_by_lines: Vec<TokenizedBody>,
}

impl Lexer {
    pub fn split_lines(&mut self) -> Result<()> {
        let mut split = Vec::new();
        let mut current_line: Vec<Token> = Vec::new();

        for token in &self.base_tokenized {
            if *token == Token::Eol {
                split.push(current_line);
                current_line = Vec::new();
            } else {
                current_line.push(token.clone());
            }
        }

        // Push the last line if it's not empty
        if !current_line.is_empty() {
            split.push(current_line);
        }

        self.split_by_lines = split;
        Ok(())
    }
}

/// Trait that has base functions required for each branch of the lexer
pub(super) trait BaseLexing {
    /// Checks if the line is valid for what this branch of the lexer is attempting to accomplish
    /// For example for math we want to check if the meets rules like two numbers on each side of the operator
    fn is_valid_line(&self, line: Line) -> bool; 

    /// Checks if the line is assigning what we are attempting to do to a variable
    fn is_assigning_to_variable(&self, line: Line) -> bool {
        if line[0] == Token::MutVarDeclaration && line[1] == Token::ImmutVarDeclaration {
            true
        } else {
            false
        }
    }



}
