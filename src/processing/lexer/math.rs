use super::Token;
use super::{BaseLexing, BaseLexingReturn, Error, Result};
use crate::processing::types::Line;

pub struct Math;

impl Math {
    fn rules(&self, line: &Line) -> bool {
        let operator_position = self.find_operator(line);

        if operator_position.is_none() {
            return false;
        }

        let operator_pos = operator_position.unwrap();

        // if the operator is the first entry in the array no operation can be computed
        if operator_pos == 0 {
            return false;
        }

        // there should be numbers on both left and right of the operator
        let num_1_pos = operator_pos - 1;
        let num_2_pos = operator_pos + 1;

        let num_at_pos_1 = self.num_at_pos(line, num_1_pos);
        let num_at_pos_2 = self.num_at_pos(line, num_2_pos);

        num_at_pos_1 && num_at_pos_2
    }

    fn num_at_pos(&self, line: &Line, pos: usize) -> bool {
        matches!(line[pos], Token::Int(_, _))
    }

    /// finds what position the operator is at
    fn find_operator(&self, line: &Line) -> Option<usize> {
        line.iter().position(|token| {
            matches!(
                token,
                Token::Add(_) | Token::Subtract(_) | Token::Divide(_) | Token::Multiply(_)
            )
        })
    }
}

impl BaseLexing for Math {
    fn is_valid_line(&self, line: &Line) -> bool {
        if !self.is_assigning_to_variable(line) {
            return self.rules(line);
        }

        let var_sliced = self.slice_variable_dec(line);

        self.rules(&var_sliced)
    }

    fn execute(&self, line: &Line) -> Result<BaseLexingReturn> {
        let final_line = if self.is_assigning_to_variable(line) {
            self.slice_variable_dec(line)
        } else {
            line.to_vec()
        };

        let num_1_token = &final_line[0];
        let num_1_position = num_1_token.get_pos();
        let num_1 = match num_1_token {
            Token::Int(value, _) => value,
            _ => return Err(Error::ExpectedInt(num_1_token.clone(), num_1_position)),
        };

        let num_2_token = &final_line[2];
        let num_2 = match num_2_token {
            Token::Int(value, _) => value,
            _ => {
                return Err(Error::ExpectedInt(
                    num_2_token.clone(),
                    num_2_token.get_pos(),
                ))
            }
        };

        let operator = &final_line[1];
        match operator {
            Token::Add(_) => Ok(BaseLexingReturn::Int(num_1 + num_2)),
            Token::Subtract(_) => Ok(BaseLexingReturn::Int(num_1 - num_2)),
            Token::Multiply(_) => Ok(BaseLexingReturn::Int(num_1 * num_2)),
            Token::Divide(_) => Ok(BaseLexingReturn::Int(num_1 / num_2)),
            _ => Err(Error::ExpectedOperator(
                operator.clone(),
                operator.get_pos(),
            )),
        }
    }
}