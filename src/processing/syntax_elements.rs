use super::lexer::base::BaseLexingReturn;
use super::lexer::{Error, Result};
use super::types::{Line, LineNumber, LineTokenizedBody, VariableName};
use std::{
    any::{Any, TypeId},
    fmt::{Display, Formatter},
};

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub line_number: LineNumber,
    pub line_position: i64,
}

impl Position {
    pub fn new(line_number: LineNumber, line_position: i64) -> Position {
        Position {
            line_number,
            line_position,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line: {}, Char: {}",
            self.line_number, self.line_position
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Eol(Position),                 // end of line
    Add(Position),                 // +
    Subtract(Position),            // -
    Divide(Position),              // "/"
    Multiply(Position),            // "*"
    SingleComment(Position),       // "//"
    MutVarDeclaration(Position),   // "set"
    ImmutVarDeclaration(Position), // "const"
    Assignment(Position),          // =
    Int(i32, Position),
    Variable(VariableName, Position),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Eol(p) => write!(f, "end of line pos: {}", p),
            Token::Add(p) => write!(f, "+ pos: {}", p),
            Token::Subtract(p) => write!(f, "- pos: {}", p),
            Token::Divide(p) => write!(f, "/ pos: {}", p),
            Token::Multiply(p) => write!(f, "* pos: {}", p),
            Token::SingleComment(p) => write!(f, "// pos: {}", p),
            Token::MutVarDeclaration(p) => write!(f, "set pos: {}", p),
            Token::ImmutVarDeclaration(p) => write!(f, "const pos: {}", p),
            Token::Assignment(p) => write!(f, "= pos: {}", p),
            Token::Int(value, p) => write!(f, "{} pos: {}", value, p),
            Token::Variable(name, p) => write!(f, "{} pos: {}", name, p),
        }
    }
}

impl Token {
    pub fn get_pos(&self) -> Position {
        match self {
            Token::Add(pos) => pos.clone(),
            Token::Eol(pos) => pos.clone(),
            Token::Subtract(pos) => pos.clone(),
            Token::Divide(pos) => pos.clone(),
            Token::Multiply(pos) => pos.clone(),
            Token::SingleComment(pos) => pos.clone(),
            Token::MutVarDeclaration(pos) => pos.clone(),
            Token::ImmutVarDeclaration(pos) => pos.clone(),
            Token::Assignment(pos) => pos.clone(),
            Token::Int(_, pos) => pos.clone(),
            Token::Variable(_, pos) => pos.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum DataTypes {
    Str(String),
    Int(i32),
    Bool(bool),
    Float(f64),
    Char(char),
}
#[derive(PartialEq, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: DataTypes,
    mutable: bool,
}

impl Variable {
    pub fn new<T: Any>(
        name: String,
        value: &T,
        mutable: bool,
    ) -> Result<Variable> {
        
        let inferred_type = Self::convert_to_data_types(value);

        let value = match inferred_type {
            None => return Err(Error::FailedToInferType),
            Some(i) => i,
        };

        Ok(Variable {
            name,
            value,
            mutable,
        })
    }

    pub fn convert_to_data_types<T>(inferred: &T) -> Option<DataTypes>
    where
        T: Any,
    {
        let inferred_any = inferred as &dyn Any;
        match inferred_any.type_id() {
            id if id == TypeId::of::<i32>() => {
                let value = inferred_any.downcast_ref::<i32>().unwrap();
                Some(DataTypes::Int(*value))
            }
            id if id == TypeId::of::<String>() => {
                let value = inferred_any.downcast_ref::<String>().unwrap();
                Some(DataTypes::Str(value.clone()))
            }
            id if id == TypeId::of::<bool>() => {
                let value = inferred_any.downcast_ref::<bool>().unwrap();
                Some(DataTypes::Bool(*value))
            }
            id if id == TypeId::of::<f64>() => {
                let value = inferred_any.downcast_ref::<f64>().unwrap();
                Some(DataTypes::Float(*value))
            }
            id if id == TypeId::of::<char>() => {
                let value = inferred_any.downcast_ref::<char>().unwrap();
                Some(DataTypes::Char(*value))
            }
            _ => None,
        }
    }
}
