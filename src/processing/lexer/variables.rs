use super::base::{BaseLexing, BaseLexingReturn};
use super::Error;
use super::Result;
use crate::processing::syntax_elements::{DataTypes, Token, Variable};
use crate::processing::types::{Line, VecPosition};

pub struct Variables {
    line: Line,
}

impl Variables {
    fn rules(&self) -> Result<()> {
        self.is_first_token_var_declaration()?;
        self.get_var_name(&self.line)?;
        self.find_assignment_position()?;
        self.find_value_position()?;

        Ok(())
    }

    fn is_first_token_var_declaration(&self) -> Result<()> {
        match self.line.tokenized_body[0] {
            Token::MutVarDeclaration(_) => Ok(()),
            Token::ImmutVarDeclaration(_) => Ok(()),
            _ => Err(Error::ExpectedVarAssignment(self.line.number)),
        }
    }

    fn find_assignment_position(&self) -> Result<VecPosition> {
        let line = self.line.clone();
        let pos = line
            .tokenized_body
            .iter()
            .position(|token| matches!(token, Token::Assignment(_)));

        if pos.is_none() {
            return Err(Error::UnableToFindVarAssignment(self.get_var_name(&line)?));
        }

        Ok(pos.unwrap() as i8)
    }

    fn find_value_position(&self) -> Result<VecPosition> {
        let line = self.line.clone();
        let value_declaration_position = self
            .line
            .tokenized_body
            .iter()
            .position(|token| matches!(token, Token::Variable(_, _)));

        if value_declaration_position.is_none() {
            return Err(Error::ExpectedToFindVarValue(self.get_var_name(&line)?));
        }

        Ok(value_declaration_position.unwrap() as i8)
    }

    fn infer_type(&self) -> Result<DataTypes> {
        todo!()
    }

    fn set_line(&mut self, line: Line) -> () {
        self.line = line;
    }
}

impl BaseLexing for Variables {
    fn is_valid_line(&self, line: &Line) -> Result<()> {
        self.rules()
    }

    fn execute(&self, line: &Line) -> Result<BaseLexingReturn> {
        todo!()
    }
}
