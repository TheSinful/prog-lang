use super::Token;
use super::{BaseLexing, BaseLexingReturn, Error, Result};
use crate::processing::syntax_elements::Variable;
use crate::processing::types::Line;

pub struct Math;

impl Math {
    fn rules(&self, line: &Line) -> Result<()> {
        let operator_position = self.find_operator(line);

        if operator_position.is_none() {
            return Err(Error::UnableToFindOperator(line.number));
        }

        let operator_pos = operator_position.unwrap();

        // if the operator is the first entry in the array no operation can be computed
        if operator_pos == 0 {
            return Err(Error::UnexpectedOperatorPosition(0));
        }

        // there should be numbers on both left and right of the operator
        let num_1_pos = operator_pos - 1;
        let num_2_pos = operator_pos + 1;

        let num_at_pos_1 = self.num_at_pos(line, num_1_pos);
        let num_at_pos_2 = self.num_at_pos(line, num_2_pos);

        if num_at_pos_1 && num_at_pos_2 {
            Ok(())
        } else {
            Err(Error::UnexpectedIntegerPositions(line.number))
        }
    }

    fn num_at_pos(&self, line: &Line, pos: usize) -> bool {
        matches!(line.tokenized_body[pos], Token::Int(_, _))
    }

    /// finds what position the operator is at
    fn find_operator(&self, line: &Line) -> Option<usize> {
        line.tokenized_body.iter().position(|token| {
            matches!(
                token,
                Token::Add(_) | Token::Subtract(_) | Token::Divide(_) | Token::Multiply(_)
            )
        })
    }
}

impl BaseLexing for Math {
    fn is_valid_line(&self, line: &Line) -> Result<()> {
        if !self.is_assigning_to_variable(line) {
            return self.rules(line);
        }

        self.rules(line)
    }

    fn execute(&self, line: &Line) -> Result<BaseLexingReturn> {
        let assign_to_var = self.is_assigning_to_variable(line);
        let final_line = if assign_to_var {
            self.slice_variable_dec(line)
        } else {
            line.tokenized_body.to_vec()
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
        let value = match operator {
            Token::Add(_) => BaseLexingReturn::Int(num_1 + num_2),
            Token::Subtract(_) => BaseLexingReturn::Int(num_1 - num_2),
            Token::Multiply(_) => BaseLexingReturn::Int(num_1 * num_2),
            Token::Divide(_) => BaseLexingReturn::Int(num_1 / num_2),
            _ => {
                return Err(Error::ExpectedOperator(
                    operator.clone(),
                    operator.get_pos(),
                ))
            }
        };

        if assign_to_var {
            Ok(BaseLexingReturn::Variable(
                Variable::new::<BaseLexingReturn>(
                    self.get_var_name(&line)?,
                    &value,
                    self.is_var_mutable(&line),
                ),
            ))
        } else {
            Ok(value)
        }
    }
}
