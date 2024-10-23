use super::Token;
use super::{BaseLexing, BaseLexingReturn, Error, Result};
use crate::processing::syntax_elements::{DataTypes, Position, Variable};
use crate::processing::types::Line;
use crate::processing::utils::variables::{
    get_var_name, is_var_mutable, look_variable, slice_variable_dec, var_exists,
};

pub struct Math {
    variables: Vec<Variable>,
}

impl Math {
    pub fn new(variables: &Vec<Variable>) -> Math {
        Math {
            variables: variables.to_vec(),
        }
    }

    fn rules(&self, line: &Line) -> Result<()> {
        let operator_position = self.find_operator(line);

        if operator_position.is_none() {
            return Err(Error::UnableToFindOperator(line.number));
        }

        let operator_pos = operator_position.unwrap();

        // if the operator is the first entry in the array no operation can be computed
        if operator_pos == 0 {
            return Err(Error::UnexpectedOperatorPosition(line.number));
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
        let token = line.tokenized_body[pos].clone();
        if matches!(&token, Token::Int(_, _)) {
            return true;
        }

        let var_name = match token {
            Token::Variable(name, _) => name,
            _ => return false,
        };

        for variable in self.variables.iter() {
            if variable.name != var_name {
                continue;
            }

            match variable.value {
                DataTypes::Int(_) => return true,
                _ => return false,
            }
        }

        true
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

    fn extract_int_value(
        &self,
        token: &Token,
        variables: &Vec<Variable>,
        pos: Position,
    ) -> Result<i32> {
        match token {
            Token::Int(value, _) => Ok(*value),
            Token::Variable(name, _) => {
                let variable = look_variable(name, variables, DataTypes::Int(0));
                if variable.is_none() {
                    return self.expected_int_err(token, pos);
                }

                let value = variable.unwrap().value;

                match value {
                    DataTypes::Int(variable_value) => Ok(variable_value),
                    _ => self.expected_int_err(token, pos),
                }
            }
            _ => self.expected_int_err(token, pos),
        }
    }

    fn expected_int_err(&self, token: &Token, pos: Position) -> Result<i32> {
        Err(Error::ExpectedInt(token.clone(), pos))
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
            slice_variable_dec(line)
        } else {
            line.tokenized_body.to_vec()
        };

        let num_1_token = &final_line[0];
        let num_1_position = num_1_token.get_pos();
        let num_1 = self.extract_int_value(num_1_token, &self.variables, num_1_position)?;

        let num_2_token = &final_line[2];
        let num_2_position = num_2_token.get_pos();
        let num_2 = self.extract_int_value(num_2_token, &self.variables, num_2_position)?;

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

        if !assign_to_var {
            return Ok(value);
        }

        let name = get_var_name(&line)?;
        let var_value = match value {
            BaseLexingReturn::Int(n) => n,
            _ => return Err(Error::FailedToInferType),
        };
        let mutable = is_var_mutable(&line);

        if var_exists(&self.variables, &name) {
            return Err(Error::VariableAlreadyExists(name));
        }

        Ok(BaseLexingReturn::Variable(Variable::new::<i32>(
            name,
            &var_value,
            mutable,
        )?))
    }
}
