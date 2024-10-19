use crate::tokens::Token;

pub type Result<T> = std::result::Result<T, Error>;
pub type SplitBody = Vec<String>;
pub type TokenizedBody = Vec<Token>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid token {0} at pos {1}")]
    InvalidToken(String, String),
}

#[derive(Default)]
pub struct Parser {
    pub(crate) current_body: String,
    pub(crate) split_body: SplitBody,
    pub(crate) tokenized_body: TokenizedBody,
}

impl Parser {
    pub fn parse(&mut self, body: String) -> Result<()> {
        self.current_body = body;
        self.split()?;
        self.tokenize()?;

        Ok(())
    }

    /// Takes the current_body then converts it to a array of string
    ///
    /// EXAMPLE:
    ///     "let x = 1"
    ///     ["let", "x", "=", "1"]
    pub(crate) fn split(&mut self) -> Result<()> {
        self.split_body = self
            .current_body
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        Ok(())
    }

    pub(crate) fn tokenize(&mut self) -> Result<()> {
        let mut t = Vec::new();

        for (index, token) in self.split_body.iter().enumerate() {
            match token.as_str() {
                "+" => t.push(Token::Add),
                "-" => t.push(Token::Subtract),
                "/" => t.push(Token::Divide),
                "*" => t.push(Token::Multiply),
                "\n" => t.push(Token::Eol),
                "//" => t.push(Token::SingleComment),
                "set" => t.push(Token::MutVarDeclaration),
                "const" => t.push(Token::ImmutVarDeclaration),
                "=" => t.push(Token::Assignment),
                _ if self.is_int(token) => t.push(Token::Int(token.parse().unwrap())),
                _ if self.is_variable(&t) => t.push(Token::Variable(token.to_string())),
                _ => return Err(Error::InvalidToken(token.to_string(), index.to_string())),
            }
        }

        self.tokenized_body = t;
        Ok(())
    }

    fn is_int(&self, token: &str) -> bool {
        token.parse::<i32>().is_ok()
    }

    fn is_variable(&self, tokens: &TokenizedBody) -> bool {
        let last = tokens.last();

        if last.clone() != Some(&Token::MutVarDeclaration)
            && last.clone() != Some(&Token::ImmutVarDeclaration)
        {
            false
        } else {
            true
        }
    }
}
