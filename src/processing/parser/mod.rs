use super::{
    lexer,
    syntax_elements::{Position, Token},
};

use super::types::{SplitLine, TokenizedLine};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid token {0} at pos {1}")]
    InvalidToken(String, String),
}

#[derive(Default)]
pub struct Parser {
    pub(super) current_line: String,
    pub(super) split_line: SplitLine,
    pub(super) tokenized_line: TokenizedLine,
}

impl Parser {
    pub fn parse(&mut self, body: &str) -> Result<()> {
        let lines = self.separate_to_lines(body);
        let lexer = lexer::Lexer::default();

        for (line_number, line) in lines.iter().enumerate() {
            self.current_line = line.to_string();
            let split = self.split();
            let _tokenized = self.tokenize(split, line_number as i64)?;
            let lexerize = lexer.lexerize(_tokenized);
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
    pub(super) fn split(&mut self) -> SplitLine {
        self.current_line
            .split(' ')
            .map(|s| s.to_string())
            .collect()
    }

    pub(super) fn tokenize(&mut self, split_line: SplitLine, line_number: i64) -> Result<TokenizedLine> {
        let mut t: TokenizedLine = Vec::new();

        for (index, token) in split_line.iter().enumerate() {
            let pos = Position::new(line_number, index as i64);
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
                _ => return Err(Error::InvalidToken(token.to_string(), index.to_string())),
            }
        }

        Ok(t)
    }

    fn is_int(&self, token: &str) -> bool {
        token.parse::<i32>().is_ok()
    }

    fn is_variable(&self, tokens: &TokenizedLine) -> bool {
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