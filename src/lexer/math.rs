use crate::tokens::Token;

use super::{BaseLexing, Line, Result};

pub(super) struct Math;

impl Math {
    fn num_rules(&self, line: Line) -> bool {
        let operator_position = self.find_operator(line);

        true
    }

    fn is_adding(&self, line: Line) -> bool {
        line.contains(&Token::Add)
    }

    fn is_subtraction(&self, line: Line) -> bool {
        line.contains(&Token::Subtract)
    }

    fn is_dividing(&self, line: Line) -> bool {
        line.contains(&Token::Divide)
    }

    fn is_multiplying(&self, line: Line) -> bool {
        line.contains(&Token::Multiply)
    }

    fn find_operator(&self, line: Line) -> Option<usize> {
        line.iter().position(|token| {
            matches!(
                token,
                Token::Add | Token::Subtract | Token::Divide | Token::Multiply
            )
        })
    }
}

impl BaseLexing for Math {
    fn is_valid_line(&self, line: Line) -> bool {
        if !self.is_assigning_to_variable(line) {}

        true
    }
}
