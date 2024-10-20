use crate::lexer::{self, BaseLexingReturn};
use crate::parser::{self, Error, Parser};
use crate::tokens::{Position, Token};

// #[test]
// fn test_success_split() {
//     let mut parser = Parser::default();
//     parser.parse("set x = 1").unwrap();
//     assert_eq!(parser.split_line, vec!["set", "x", "=", "1"]);
// }

// #[test]
// fn test_invalid_token_error() {
//     let mut parser = Parser::default();
//     let result = parser.parse("invalid_token x = 1");
//     assert!(
//         matches!(result, Err(Error::InvalidToken(ref token, ref pos)) if token == "invalid_token" && pos == "0")
//     );
// }

// #[test]
// fn test_success_tokenize() {
//     let mut parser = Parser::default();
//     let base = "1 + 1".to_string();
//     parser.parse(&base).unwrap();
//     let success = vec![
//         BaseLexingReturn::Int(2)
//     ];

//     assert_eq!(parser.tokenized_line, success)
// }