#![allow(unused)]

mod lexer;
mod parser;
mod tokens;

#[cfg(test)]
mod tests;

fn main() {
    let mut parser = parser::Parser::default();
    parser.parse("1 + 1");
}
