use super::{
    lexer,
    syntax_elements::{Position, Token, Variable},
    types::{Line, LineNumber},
    utils::variables::{look_variable, var_exists},
};

use super::types::{LineSplitBody, LineTokenizedBody};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid token {0} at pos {1}")]
    InvalidToken(String, String),
}

#[derive(Default)]
pub struct Parser {
    pub(super) current_line: String,
    pub(super) split_line: LineSplitBody,
    pub(super) tokenized_line: LineTokenizedBody,
    pub(super) variables: Vec<Variable>,
}

impl Parser {
    pub fn parse(&mut self, body: &str) -> Result<()> {
        let lines = self.separate_to_lines(body);
        let mut lexer = lexer::Lexer::default();

        for (line_number, line) in lines.iter().enumerate() {
            self.current_line = line.to_string();
            let split = self.split();
            let line_number = line_number as LineNumber;
            let _tokenized = self.tokenize(&split, line_number)?;
            let line = Line::new(_tokenized, line.to_string(), split, line_number);
            let lexerize = lexer.lexerize(line, &mut self.variables);
            match lexerize {
                Ok(_) => continue,
                Err(e) => println!("{}", e),
            }
        }

        Ok(())
    }

    fn separate_to_lines(&mut self, body: &str) -> Vec<String> {
        let lines: Vec<String> = body.lines().map(|line| line.to_string()).collect();
        lines
    }

    /// Takes the current_body then converts it to a array of string
    ///
    /// EXAMPLE:
    ///     "let x = 1"
    ///     ["let", "x", "=", "1"]
    pub(super) fn split(&mut self) -> LineSplitBody {
        self.current_line
            .split(' ')
            .map(|s| s.to_string())
            .collect()
    }

    pub(super) fn tokenize(
        &mut self,
        split_line: &LineSplitBody,
        line_number: LineNumber,
    ) -> Result<LineTokenizedBody> {
        let mut t: LineTokenizedBody = Vec::new();

        for (index, token) in split_line.iter().enumerate() {
            let pos = Position::new(line_number, index as LineNumber);
            match token.as_str() {
                "+" => t.push(Token::Add(pos)),
                "-" => t.push(Token::Subtract(pos)),
                "/" => t.push(Token::Divide(pos)),
                "*" => t.push(Token::Multiply(pos)),
                "\n" => t.push(Token::Eol(pos)),
                "//" => t.push(Token::SingleComment(pos)),
                "set" => t.push(Token::MutVarDeclaration(pos)),
                "const" => t.push(Token::ImmutVarDeclaration(pos)),
                "=" => t.push(Token::Assignment(pos)),
                _ if self.is_int(token) => t.push(Token::Int(token.parse().unwrap(), pos)),
                _ if self.is_variable(&t) => t.push(Token::Variable(token.to_string(), pos)),
                _ => {
                    if !var_exists(&self.variables, token.as_str()) {
                        return Err(Error::InvalidToken(token.to_string(), index.to_string()));
                    }

                    t.push(Token::Variable(token.to_string(), pos))
                }
            }
        }
        Ok(t)
    }

    fn is_int(&self, token: &str) -> bool {
        token.parse::<i32>().is_ok()
    }

    fn is_variable(&self, tokens: &LineTokenizedBody) -> bool {
        if let Some(last) = tokens.last() {
            matches!(
                last,
                Token::MutVarDeclaration(_) | Token::ImmutVarDeclaration(_)
            )
        } else {
            false
        }
    }
}
